# Quick Reference: Architectural Patterns from React to Rust

**Purpose**: Fast lookup guide for developers implementing patterns from React repository  
**Last Updated**: 2025-11-17  
**Constitution Version**: 1.5.0 (Unconditional modular architecture)  
**Specification Version**: 3.1.0 (Mandatory package structure)  
**Related**: MISSING-PATTERNS-ANALYSIS.md, ARCHITECTURAL-COMPARISON.md

**⚠️ CRITICAL**: ALL functionality MUST be implemented as packages in `packages/` directory. This is NON-NEGOTIABLE.

---

## Pattern Lookup Table

| React Pattern | Rust Equivalent | Priority | Phase |
|--------------|-----------------|----------|-------|
| PNPM catalog | `[workspace.dependencies]` | ⭐ HIGH | 1 |
| TypeORM | SeaORM + sqlx | ⭐ HIGH | 1 |
| Zod validation | validator crate | 🔶 MEDIUM | 1-2 |
| express-rate-limit | governor crate | 🔶 MEDIUM | 1-2 |
| Swagger/OpenAPI | utoipa | 🔶 MEDIUM | 1-2 |
| Vitest | cargo test | ⭐ HIGH | 1 |
| Testing Library | wasm-bindgen-test | ⭐ HIGH | 1 |
| Coverage (v8) | cargo tarpaulin | ⭐ HIGH | 1 |
| Husky | cargo-husky | ⭐ HIGH | 1 |
| ESLint | cargo clippy | ⭐ HIGH | 1 |
| Prettier | rustfmt | ⭐ HIGH | 1 |
| Vite dev server | trunk serve | 🔶 MEDIUM | 1-2 |
| nodemon | cargo-watch | 🔶 MEDIUM | 1-2 |
| Turborepo | Cargo workspace | 🔵 NICE | 1 |
| Winston/Pino logging | tracing crate | 🔵 NICE | 2-3 |
| http-errors | thiserror + anyhow | 🔵 NICE | 2 |
| cors middleware | actix-cors | 🔵 NICE | 2 |
| multer (uploads) | actix-multipart | 🔵 NICE | 2 |

---

## Quick Setup Commands

### 1. Workspace Dependencies (HIGH PRIORITY)

**Root Cargo.toml:**
```toml
[workspace.dependencies]
# Core
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }

# Web Framework
actix-web = "4.4"
actix-cors = "0.7"

# Frontend
yew = "0.21"
yew-router = "0.18"

# Database
sea-orm = { version = "0.12", features = ["sqlx-postgres", "runtime-tokio-native-tls"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }

# API Client
reqwest = { version = "0.11", features = ["json"] }

# Validation
validator = { version = "0.16", features = ["derive"] }

# API Documentation
utoipa = { version = "4.0", features = ["actix_extras"] }
utoipa-swagger-ui = { version = "4.0", features = ["actix-web"] }

# Rate Limiting
governor = "0.6"
actix-governor = "0.4"

# Error Handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

**Package Cargo.toml:**
```toml
[dependencies]
serde = { workspace = true }
actix-web = { workspace = true }
# ... etc
```

### 2. Development Tools Installation

```bash
# Essential tools (HIGH PRIORITY)
cargo install cargo-watch        # Auto-rebuild
cargo install trunk             # WASM dev server
cargo install cargo-husky       # Git hooks
cargo install cargo-tarpaulin   # Coverage

# Database tools
cargo install sea-orm-cli       # Migrations

# Optional but recommended
cargo install cargo-make        # Task runner
cargo install just              # Command runner
```

### 3. Git Hooks Setup (HIGH PRIORITY)

**Add to root Cargo.toml:**
```toml
[dev-dependencies]
cargo-husky = { version = "1", features = ["user-hooks"] }
```

**Create `.cargo-husky/hooks/pre-commit`:**
```bash
#!/bin/sh
set -e

echo "Running pre-commit checks..."

# Format check
cargo fmt --all -- --check || {
    echo "❌ Code not formatted. Run: cargo fmt --all"
    exit 1
}

# Linting
cargo clippy --workspace -- -D warnings || {
    echo "❌ Clippy warnings found. Fix them and try again."
    exit 1
}

# Optional: Tests (can be slow)
# cargo test --workspace

echo "✅ Pre-commit checks passed!"
```

### 4. SeaORM Setup (HIGH PRIORITY)

**Create entity:**
```bash
# Generate migration
sea-orm-cli migrate generate create_clusters

# Apply migration
sea-orm-cli migrate up

# Generate entity from database
sea-orm-cli generate entity \
    --output-dir packages/clusters-srv/base/src/models
```

**Entity example:**
```rust
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "clusters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::domain::Entity")]
    Domains,
}
```

### 5. Validation Setup (MEDIUM PRIORITY)

**API request with validation:**
```rust
use validator::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateClusterRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(max = 500))]
    pub description: Option<String>,
    
    #[validate(custom = "validate_uuid")]
    pub owner_id: String,
}

// In route handler
async fn create_cluster(
    req: web::Json<CreateClusterRequest>
) -> Result<HttpResponse, Error> {
    req.validate()?;  // Returns 400 if validation fails
    // ... handle request
}
```

### 6. API Documentation Setup (MEDIUM PRIORITY)

**Route with utoipa:**
```rust
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
struct Cluster {
    id: Uuid,
    name: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/clusters",
    responses(
        (status = 200, description = "List of clusters", body = [Cluster]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Clusters"
)]
async fn list_clusters() -> Json<Vec<Cluster>> {
    // implementation
}

// In main.rs
#[derive(OpenApi)]
#[openapi(
    paths(list_clusters, create_cluster),
    components(schemas(Cluster, CreateClusterRequest))
)]
struct ApiDoc;

// Serve Swagger UI
HttpServer::new(|| {
    App::new()
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-doc/openapi.json", ApiDoc::openapi())
        )
})
```

### 7. Rate Limiting Setup (MEDIUM PRIORITY)

```rust
use actix_governor::{Governor, GovernorConfigBuilder};

let governor_conf = GovernorConfigBuilder::default()
    .per_second(2)
    .burst_size(5)
    .finish()
    .unwrap();

HttpServer::new(move || {
    App::new()
        .wrap(Governor::new(&governor_conf))
        .service(api_routes)
})
```

### 8. Hot Reload Workflow (MEDIUM PRIORITY)

**Backend:**
```bash
# Basic auto-reload
cargo watch -x 'run -p clusters-srv'

# With clear screen and ignoring files
cargo watch -c -x 'run -p clusters-srv' -i '*.md'

# With environment variables
RUST_LOG=debug cargo watch -x 'run -p clusters-srv'
```

**Frontend:**
```bash
cd packages/clusters-frt/base
trunk serve --open
```

**Full Stack (use tmux or separate terminals):**
```bash
# Terminal 1: Backend
cargo watch -x 'run -p clusters-srv'

# Terminal 2: Frontend
cd packages/clusters-frt/base && trunk serve

# Terminal 3: Shared packages (rebuild on change)
cargo watch -x 'build -p universo-types -p universo-utils'
```

### 9. Testing Setup (HIGH PRIORITY)

**Unit test:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cluster_creation() {
        let cluster = Cluster::new("Test");
        assert_eq!(cluster.name, "Test");
    }
}
```

**Actix integration test:**
```rust
#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    
    #[actix_web::test]
    async fn test_list_clusters() {
        let app = test::init_service(
            App::new().service(list_clusters)
        ).await;
        
        let req = test::TestRequest::get()
            .uri("/api/v1/clusters")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

**WASM test:**
```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_component() {
    // Yew component test
}
```

**Coverage:**
```bash
cargo tarpaulin --workspace --out Html
# Opens coverage report in browser
```

### 10. CI/CD Quality Gates (HIGH PRIORITY)

**.github/workflows/ci.yml:**
```yaml
name: CI

on: [push, pull_request]

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Clippy
        run: cargo clippy --workspace -- -D warnings
      
      - name: Tests
        run: cargo test --workspace
      
      - name: Coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --workspace --out Xml
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

---

## Package Structure Standard

```
packages/[name]-srv/base/
├── src/
│   ├── models/          # SeaORM entities
│   │   ├── mod.rs
│   │   ├── cluster.rs
│   │   └── domain.rs
│   ├── routes/          # Actix-web route handlers
│   │   ├── mod.rs
│   │   └── clusters.rs
│   ├── services/        # Business logic
│   │   ├── mod.rs
│   │   └── cluster_service.rs
│   ├── validators/      # Input validation
│   │   ├── mod.rs
│   │   └── cluster_validators.rs
│   ├── migrations/      # Database migrations
│   │   └── m20240101_000000_create_clusters.rs
│   └── lib.rs           # Package entry point
├── tests/               # Integration tests
│   └── api_tests.rs
├── Cargo.toml
├── README.md
└── README-RU.md
```

```
packages/[name]-frt/base/
├── src/
│   ├── components/      # Yew components
│   │   ├── mod.rs
│   │   ├── list.rs
│   │   └── detail.rs
│   ├── pages/           # Page components
│   │   ├── mod.rs
│   │   └── clusters_page.rs
│   ├── services/        # API clients
│   │   ├── mod.rs
│   │   └── cluster_service.rs
│   └── lib.rs           # Package entry point
├── Cargo.toml
├── index.html           # Trunk entry point
├── README.md
└── README-RU.md
```

---

## Common Commands Cheatsheet

```bash
# Build entire workspace
cargo build --workspace

# Run specific package
cargo run -p clusters-srv

# Test entire workspace
cargo test --workspace

# Test specific package
cargo test -p clusters-srv

# Format all code
cargo fmt --all

# Lint all code
cargo clippy --workspace -- -D warnings

# Coverage
cargo tarpaulin --workspace --out Html

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check without building
cargo check --workspace

# Build for release
cargo build --workspace --release

# Run SeaORM migration
sea-orm-cli migrate up

# Generate SeaORM entities
sea-orm-cli generate entity -o src/models

# WASM build (frontend)
trunk build --release

# WASM dev server
trunk serve --open
```

---

## Priority Implementation Order

### Week 1: Foundation
1. ✅ Set up workspace dependencies in root Cargo.toml
2. ✅ Install development tools (cargo-watch, trunk, cargo-husky, cargo-tarpaulin)
3. ✅ Configure git hooks with cargo-husky
4. ✅ Set up CI/CD with quality gates

### Week 2: Database & Validation
1. ✅ Set up SeaORM and create first migration
2. ✅ Configure validator crate patterns
3. ✅ Set up utoipa for API documentation
4. ✅ Configure governor for rate limiting

### Week 3: Testing & Polish
1. ✅ Set up comprehensive testing infrastructure
2. ✅ Configure cargo tarpaulin for coverage
3. ✅ Document hot reload workflow
4. ✅ Create package structure templates

---

## Troubleshooting

### "Dependency version conflict"
**Solution**: Use workspace dependencies in root Cargo.toml, reference with `workspace = true`

### "SeaORM entity not found"
**Solution**: Run `sea-orm-cli generate entity` after migrations

### "Trunk build fails"
**Solution**: Ensure wasm32-unknown-unknown target installed: `rustup target add wasm32-unknown-unknown`

### "Pre-commit hook fails"
**Solution**: Run `cargo fmt --all` and fix clippy warnings: `cargo clippy --workspace --fix`

### "Coverage shows 0%"
**Solution**: Ensure tests exist and run `cargo test` first to verify they pass

---

**For detailed explanations, see:**
- MISSING-PATTERNS-ANALYSIS.md - Full pattern catalog with examples
- ARCHITECTURAL-COMPARISON.md - React vs Rust comparison
- ARCHITECTURAL-UPDATES-SUMMARY.md - Summary of changes
- constitution.md - Principles XVI and XVII

**Quick Links:**
- [SeaORM Docs](https://www.sea-ql.org/SeaORM/)
- [validator crate](https://docs.rs/validator/)
- [utoipa Docs](https://docs.rs/utoipa/)
- [governor crate](https://docs.rs/governor/)
- [Actix Web](https://actix.rs/)
- [Yew Framework](https://yew.rs/)
