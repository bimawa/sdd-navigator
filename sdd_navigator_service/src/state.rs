use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{CoverageMetrics, RequirementAnnotation, UncoveredFunction};
use crate::scanner::{make_relative, scan_directory};

#[derive(Debug, Clone)]
pub struct AppState {
    pub metrics: CoverageMetrics,
    pub requirements: Vec<RequirementAnnotation>,
    pub uncovered: Vec<UncoveredFunction>,
}

pub type SharedState = Arc<RwLock<AppState>>;

fn compute_state(scan_root: &str) -> AppState {
    let result = scan_directory(scan_root);

    let covered_functions = result.requirements.len();
    let uncovered_count = result.uncovered.len();
    let total = result.total_functions;

    let coverage_percentage = if total > 0 {
        (covered_functions as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let unique_ids: HashSet<String> = result.requirements.iter().map(|r| r.req_id.clone()).collect();

    let requirements: Vec<RequirementAnnotation> = result
        .requirements
        .into_iter()
        .map(|mut r| {
            r.file = make_relative(&r.file, scan_root);
            r
        })
        .collect();

    let uncovered: Vec<UncoveredFunction> = result
        .uncovered
        .into_iter()
        .map(|mut u| {
            u.file = make_relative(&u.file, scan_root);
            u
        })
        .collect();

    AppState {
        metrics: CoverageMetrics {
            total_unique_requirements: unique_ids.len(),
            total_functions: total,
            covered_functions,
            uncovered_functions: uncovered_count,
            coverage_percentage: (coverage_percentage * 100.0).round() / 100.0,
            scanned_files: result.scanned_files,
        },
        requirements,
        uncovered,
    }
}

pub fn build_state(scan_root: &str) -> SharedState {
    Arc::new(RwLock::new(compute_state(scan_root)))
}

pub fn build_empty() -> SharedState {
    Arc::new(RwLock::new(AppState {
        metrics: CoverageMetrics {
            total_unique_requirements: 0,
            total_functions: 0,
            covered_functions: 0,
            uncovered_functions: 0,
            coverage_percentage: 0.0,
            scanned_files: 0,
        },
        requirements: vec![],
        uncovered: vec![],
    }))
}

pub async fn rebuild_state(shared: &SharedState, scan_root: &str) {
    let new_state = compute_state(scan_root);
    let mut s = shared.write().await;
    *s = new_state;
}
