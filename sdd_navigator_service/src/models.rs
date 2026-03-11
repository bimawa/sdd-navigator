use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ScanRequest {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementAnnotation {
    pub req_id: String,
    pub description: String,
    pub file: String,
    pub line: usize,
    pub function_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncoveredFunction {
    pub function_name: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    pub total_unique_requirements: usize,
    pub total_functions: usize,
    pub covered_functions: usize,
    pub uncovered_functions: usize,
    pub coverage_percentage: f64,
    pub scanned_files: usize,
}
