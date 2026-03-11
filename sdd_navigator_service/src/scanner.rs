use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::models::{RequirementAnnotation, UncoveredFunction};

pub struct ScanResult {
    pub requirements: Vec<RequirementAnnotation>,
    pub uncovered: Vec<UncoveredFunction>,
    pub total_functions: usize,
    pub scanned_files: usize,
}

pub fn scan_directory(root: &str) -> ScanResult {
    let req_re = Regex::new(r"//\s*@req:\s*([A-Z]+-\d+)\s*(?:-\s*(.+))?").unwrap();
    let fn_re = Regex::new(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(").unwrap();

    let mut requirements = Vec::new();
    let mut uncovered = Vec::new();
    let mut total_functions = 0usize;
    let mut scanned_files = 0usize;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "rs").unwrap_or(false))
    {
        let path = entry.path();
        let Ok(content) = fs::read_to_string(path) else {
            continue;
        };

        scanned_files += 1;
        let file_str = path.to_string_lossy().to_string();

        let lines: Vec<&str> = content.lines().collect();
        let mut pending_req: Option<(String, String, usize)> = None;

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            if let Some(caps) = req_re.captures(line) {
                let req_id = caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                let description = caps
                    .get(2)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                pending_req = Some((req_id, description, line_num));
                continue;
            }

            if let Some(caps) = fn_re.captures(line) {
                let fn_name = caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                total_functions += 1;

                if let Some((req_id, description, req_line)) = pending_req.take() {
                    requirements.push(RequirementAnnotation {
                        req_id,
                        description,
                        file: file_str.clone(),
                        line: req_line,
                        function_name: Some(fn_name),
                    });
                } else {
                    uncovered.push(UncoveredFunction {
                        function_name: fn_name,
                        file: file_str.clone(),
                        line: line_num,
                    });
                }
            } else if !line.trim().is_empty() && !line.trim().starts_with("//") {
                pending_req = None;
            }
        }
    }

    ScanResult {
        requirements,
        uncovered,
        total_functions,
        scanned_files,
    }
}

pub fn make_relative(path: &str, root: &str) -> String {
    Path::new(path)
        .strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string())
}
