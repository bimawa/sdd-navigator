# SDD Navigator Service
> Demo project for **ForEach Partners**

An HTTP service written in Rust that scans a codebase for `@req` annotations, computes specification coverage metrics, and exposes the results via a REST API and a built-in web dashboard.

## Projects

| Directory | Description |
|---|---|
| `sdd_navigator_service/` | The scanner service (Axum + Tokio + Serde) |
| `sdd_test_project/` | Sample codebase used to test the scanner |

## How It Works

The scanner recursively walks `.rs` files in a target directory. Any line matching `// @req: <ID> - <description>` immediately before a `fn` declaration is captured as a covered requirement. Functions without a preceding `@req` annotation are reported as uncovered.

## Stack

- **Axum** — HTTP server and routing
- **Tokio** — async runtime
- **Serde / serde_json** — JSON serialization
- **walkdir** — recursive directory traversal
- **regex** — annotation and function detection
- **tower-http** — CORS middleware
- **tracing / tracing-subscriber** — structured logging

## Getting Started

```bash
cd sdd_navigator_service
cargo run
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

Enter the absolute path to a Rust source directory and click **Scan**.

Optionally pre-load a path at startup:

```bash
SCAN_ROOT=/path/to/your/src cargo run
```

## REST API

| Method | Path | Description |
|---|---|---|
| `GET` | `/` | Web dashboard |
| `GET` | `/metrics` | Coverage metrics (JSON) |
| `GET` | `/requirements` | All found `@req` annotations (JSON) |
| `GET` | `/uncovered` | Functions without coverage (JSON) |
| `POST` | `/scan` | Trigger a new scan for the given path |

### POST /scan

```json
{ "path": "/absolute/path/to/src" }
```

Response:

```json
{
  "ok": true,
  "metrics": {
    "scanned_files": 2,
    "total_functions": 9,
    "covered_functions": 6,
    "uncovered_functions": 3,
    "total_unique_requirements": 6,
    "coverage_percentage": 66.67
  }
}
```

## Annotation Format

```rust
// @req: USR-001 - User must be able to register
fn register_user(username: &str, email: &str) -> bool {
    ...
}
```

ID format: `[A-Z]+-[0-9]+` (e.g. `USR-001`, `PRD-042`, `UTIL-003`)
