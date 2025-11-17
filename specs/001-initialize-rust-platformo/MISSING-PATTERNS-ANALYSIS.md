# Missing Patterns Analysis: React to Rust Migration

**Date**: 2025-11-17  
**Purpose**: Identify architectural patterns from React repository not fully documented in current Rust plans  
**React Repository**: https://github.com/teknokomo/universo-platformo-react  
**Related Documents**: ARCHITECTURAL-COMPARISON.md, IMPLEMENTATION-ROADMAP.md

---

## Executive Summary

This document complements the existing architectural comparison by identifying 15 additional patterns and best practices from the React implementation that should be incorporated into the Rust project plans. These patterns cover dependency management, code organization, testing, quality assurance, and development workflows.

## Critical Patterns Requiring Immediate Documentation

### 1. Workspace Dependency Catalog Pattern ⭐ HIGH PRIORITY

**React Implementation:**
```yaml
# pnpm-workspace.yaml catalog feature
catalog:
    typescript: ^5.8.3
    react: ^18.3.1
    i18next: 23.16.8
    # ... all dependencies with versions
```

**Why It Matters:**
- Single source of truth for dependency versions across all packages
- Prevents version conflicts and "dependency hell"
- Simplifies upgrades (change version in one place)
- Enforces consistency across the monorepo

**Rust Equivalent Strategy:**
```toml
# Root Cargo.toml [workspace.dependencies]
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
actix-web = "4.4"
yew = "0.21"
reqwest = { version = "0.11", features = ["json"] }

# Package Cargo.toml references
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

**Documentation Required:**
- Add to constitution as Principle XVI: "Workspace Dependency Catalog"
- Update spec with requirement for centralized dependency management
- Add to implementation roadmap Phase 1

---

### 2. Standardized Package Internal Structure ⭐ HIGH PRIORITY

**React Pattern Observed:**
```
packages/[name]-srv/base/src/
├── database/
│   ├── entities/      # TypeORM entity definitions
│   └── migrations/    # Database schema migrations
├── routes/            # Express route handlers
├── schemas/           # Zod validation schemas
├── types/             # TypeScript type definitions
├── utils/             # Package-specific utilities
├── tests/             # Unit and integration tests
└── index.ts           # Package entry point
```

**Consistency Benefit:**
- Developers immediately know where to find entities, routes, or validators
- Reduces cognitive load when switching between packages
- Simplifies code reviews and onboarding

**Rust Pattern Proposal:**
```
packages/[name]-srv/base/src/
├── models/            # Database entity models (SeaORM/sqlx)
│   ├── mod.rs
│   ├── cluster.rs
│   └── domain.rs
├── routes/            # Actix-web route handlers
│   ├── mod.rs
│   └── clusters.rs
├── services/          # Business logic layer
│   ├── mod.rs
│   └── cluster_service.rs
├── validators/        # Input validation (validator crate)
│   ├── mod.rs
│   └── cluster_validators.rs
├── migrations/        # sqlx or SeaORM migrations
│   └── 001_create_clusters.sql
└── lib.rs             # Package entry point
```

**Documentation Required:**
- Add detailed directory structure guide to spec.md
- Create template package structure in repository
- Document in IMPLEMENTATION-ROADMAP.md

---

### 3. ORM and Migration Strategy ⭐ HIGH PRIORITY

**React Implementation:**
- TypeORM for entity mapping
- Automatic migration generation
- Entity-first approach (code → database)

**Current Rust Plan Status:**
- Mentions "database abstraction" and "trait-based design"
- Does NOT specify which ORM to use
- Does NOT specify migration strategy

**Analysis:**

**Option A: SeaORM** (Recommended)
- Active Rust ORM, similar to TypeORM
- Entity definitions with derive macros
- Migration CLI tool
- Async/await support
- Good PostgreSQL support

**Option B: sqlx**
- Not a full ORM, but compile-time checked SQL
- More control, less magic
- Requires writing raw SQL
- Excellent for Supabase/PostgreSQL

**Option C: Diesel**
- Mature ORM but synchronous
- Complex for async applications
- Not recommended for async web frameworks

**Recommendation: SeaORM + sqlx**
- Use SeaORM for entity definitions and CRUD
- Use sqlx for complex queries
- Best of both worlds

**Documentation Required:**
- Update spec.md NFR-029 with SeaORM specification
- Add migration workflow to implementation roadmap
- Define entity definition patterns

**Spec Update Required:**
```markdown
### Database Requirements (Updated)

- **FR-029**: Database abstraction layer MUST use SeaORM as primary ORM
- **FR-029a**: Complex queries MAY use sqlx for type-safe SQL
- **FR-029b**: All schema changes MUST use SeaORM migrations
- **FR-029c**: Migration files MUST be version-controlled in package migrations/ directory
- **FR-029d**: Entity models MUST use SeaORM Entity trait
```

---

### 4. Validation Strategy (Zod → Rust) 🔶 MEDIUM PRIORITY

**React Implementation:**
```typescript
// Using Zod for runtime validation
const ClusterSchema = z.object({
    name: z.string().min(1).max(100),
    description: z.string().optional(),
    owner_id: z.string().uuid()
});

// Validation in route handler
const validated = ClusterSchema.parse(req.body);
```

**Current Rust Plan Status:**
- Mentions "validation rules" in spec
- Does NOT specify validation approach

**Rust Validation Options:**

**Option A: validator crate** (Recommended)
```rust
use validator::Validate;

#[derive(Validate, Deserialize)]
struct CreateClusterRequest {
    #[validate(length(min = 1, max = 100))]
    name: String,
    
    #[validate(length(max = 500))]
    description: Option<String>,
    
    #[validate(custom = "validate_uuid")]
    owner_id: String,
}

// In route handler
let request = request.validate()?;
```

**Option B: garde crate**
- Newer, more ergonomic
- Better error messages
- Async validation support

**Option C: serde with custom validators**
- More manual but flexible
- No additional dependencies

**Recommendation: validator crate**
- Most mature and widely used
- Good balance of features and simplicity
- Works well with serde

**Documentation Required:**
- Add validation requirements to spec.md
- Specify validator crate as standard
- Add validation patterns to roadmap

---

### 5. Rate Limiting Architecture 🔶 MEDIUM PRIORITY

**React Implementation:**
```typescript
import rateLimit from 'express-rate-limit';
import RedisStore from 'rate-limit-redis';
import { createClient } from 'redis';

const limiter = rateLimit({
    store: new RedisStore({
        client: createClient({ url: process.env.REDIS_URL })
    }),
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100 // limit each IP to 100 requests per windowMs
});
```

**Current Rust Plan Status:**
- NFR-011: "API endpoints MUST implement rate limiting"
- Does NOT specify implementation approach

**Rust Rate Limiting Options:**

**Option A: governor crate + Actix middleware** (Recommended)
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

**Option B: Custom middleware with Redis**
- More control
- Distributed rate limiting
- Requires Redis setup

**Recommendation: Start with governor, add Redis later**
- Phase 1: In-memory with governor (simpler)
- Phase 2: Add Redis for distributed setups

**Documentation Required:**
- Update NFR-011 with specific implementation
- Add rate limiting to implementation roadmap Phase 1

---

### 6. API Documentation Generation Pattern 🔶 MEDIUM PRIORITY

**React Implementation:**
- universo-rest-docs package
- Swagger UI server
- OpenAPI 3.0 specifications
- Manual specification maintenance

**Current Rust Plan Status:**
- Not mentioned in current spec

**Rust API Documentation Strategy:**

**Option A: utoipa crate** (Recommended)
```rust
use utoipa::{OpenApi, ToSchema};

#[derive(ToSchema, Serialize)]
struct Cluster {
    id: Uuid,
    name: String,
    description: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/v1/clusters",
    responses(
        (status = 200, description = "List of clusters", body = [Cluster])
    )
)]
async fn list_clusters() -> Json<Vec<Cluster>> {
    // implementation
}

#[derive(OpenApi)]
#[openapi(paths(list_clusters), components(schemas(Cluster)))]
struct ApiDoc;
```

**Benefits:**
- Auto-generated from code
- Type-safe documentation
- Swagger UI integration
- Always in sync with code

**Documentation Required:**
- Add API documentation requirement to spec
- Add utoipa to Phase 1 shared infrastructure

---

### 7. Testing Infrastructure Pattern ⭐ HIGH PRIORITY

**React Implementation:**
```json
{
  "scripts": {
    "test": "vitest",
    "test:coverage": "vitest --coverage"
  }
}
```

Uses:
- Vitest for unit tests
- Testing Library for React components
- Happy-DOM for DOM simulation
- Coverage reports with v8

**Current Rust Plan Status:**
- TST-001 through TST-013 define requirements
- Does NOT specify testing tools or patterns

**Rust Testing Strategy:**

**Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cluster_creation() {
        let cluster = Cluster::new("Test Cluster");
        assert_eq!(cluster.name, "Test Cluster");
    }
}
```

**Integration Tests (with actix-web):**
```rust
// tests/api_tests.rs
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
```

**WASM Tests (for frontend):**
```rust
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn test_component() {
    // Yew component tests
}
```

**Coverage:**
```bash
cargo tarpaulin --workspace --out Html
```

**Documentation Required:**
- Add specific testing tools to spec (cargo test, tarpaulin, wasm-bindgen-test)
- Create testing guide document
- Add test examples to implementation roadmap

---

### 8. Code Quality and Git Hooks Pattern ⭐ HIGH PRIORITY

**React Implementation:**
```json
{
  "scripts": {
    "format": "prettier --write \"**/*.{ts,tsx,md}\"",
    "lint": "eslint \"**/*.{js,jsx,ts,tsx,json,md}\"",
    "lint-fix": "pnpm lint --fix"
  },
  "lint-staged": {
    "*.{js,jsx,ts,tsx,json,md}": "eslint --fix"
  }
}
```

With Husky for pre-commit hooks.

**Current Rust Plan Status:**
- Mentions clippy and rustfmt
- Does NOT specify git hooks or automation

**Rust Quality Tooling:**

**Pre-commit Hooks (cargo-husky):**
```toml
[dev-dependencies]
cargo-husky = { version = "1", features = ["user-hooks"] }
```

Then create `.cargo-husky/hooks/pre-commit`:
```bash
#!/bin/sh
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

**CI/CD Quality Gates:**
```yaml
# .github/workflows/ci.yml
- name: Check formatting
  run: cargo fmt --all -- --check
  
- name: Clippy
  run: cargo clippy --workspace -- -D warnings
  
- name: Tests
  run: cargo test --workspace
  
- name: Coverage
  run: cargo tarpaulin --workspace --out Xml
```

**Documentation Required:**
- Add git hooks requirement to spec
- Add cargo-husky to Phase 1 setup
- Document quality gates in CI/CD section

---

### 9. Build Orchestration Pattern 🔶 MEDIUM PRIORITY

**React Implementation:**
```json
// turbo.json
{
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**"],
      "cache": false
    }
  }
}
```

Turborepo handles:
- Parallel builds
- Dependency ordering
- Build caching
- Task orchestration

**Current Rust Plan Status:**
- Mentions "cargo build --workspace"
- Does NOT address complex build workflows

**Rust Build Orchestration:**

**Option A: Cargo workspace (built-in)** - Sufficient for most cases
```bash
cargo build --workspace
```

**Option B: cargo-make** - For complex workflows
```toml
[tasks.build-all]
dependencies = ["build-types", "build-utils", "build-packages"]

[tasks.build-types]
command = "cargo"
args = ["build", "-p", "universo-types"]

[tasks.build-utils]
dependencies = ["build-types"]
command = "cargo"
args = ["build", "-p", "universo-utils"]
```

**Option C: just command runner** - Simpler alternative
```just
# justfile
build-all: build-types build-utils build-packages

build-types:
    cargo build -p universo-types

build-utils: build-types
    cargo build -p universo-utils
```

**Recommendation:**
- Start with native Cargo workspace
- Add cargo-make or just only if complex workflows needed

**Documentation Required:**
- Document build workflow in roadmap
- Add example build scripts

---

### 10. Hot Reload Development Workflow 🔶 MEDIUM PRIORITY

**React Implementation:**
- Vite dev server with HMR
- Instant feedback during development
- No manual restarts needed

**Current Rust Plan Status:**
- Mentions cargo-watch
- Mentions trunk for WASM
- Does NOT provide complete workflow

**Rust Hot Reload Setup:**

**Backend (Actix Web):**
```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x 'run -p clusters-srv'
```

**Advanced: Zero-downtime with systemfd + listenfd:**
```bash
systemfd --no-pid -s http::3000 -- cargo watch -x 'run -p clusters-srv'
```

**Frontend (Yew with Trunk):**
```bash
# Install trunk
cargo install trunk

# Run dev server with hot reload
cd packages/clusters-frt/base
trunk serve
```

**Full Stack Development:**
```bash
# Terminal 1: Backend
cargo watch -x 'run -p clusters-srv'

# Terminal 2: Frontend
cd packages/clusters-frt/base && trunk serve

# Terminal 3: Shared packages (rebuild on change)
cargo watch -x 'build -p universo-types -p universo-utils'
```

**Documentation Required:**
- Add hot reload setup to Phase 1 roadmap
- Create development workflow guide
- Add scripts to repository

---

## Additional Patterns Identified

### 11. Environment Configuration Pattern

**React:** Uses dotenv with .env files
**Rust:** Use config crate or envy for environment variables

### 12. Logging Strategy

**React:** Various logging libraries (winston, pino)
**Rust:** Use tracing crate with structured logging

### 13. Error Handling Pattern

**React:** Custom error classes, http-errors
**Rust:** Use thiserror for custom errors, anyhow for applications

### 14. CORS Configuration

**React:** cors middleware with Express
**Rust:** actix-cors middleware

### 15. File Upload Handling

**React:** multer for multipart/form-data
**Rust:** actix-multipart

---

## Patterns Already Well-Covered

The following patterns from React are already well-documented in existing plans:

✅ Shared infrastructure packages (types, utils, api-client, i18n)  
✅ Template system architecture  
✅ UPDL as core abstraction  
✅ Package naming conventions (-frt, -srv, base/)  
✅ Three-entity pattern (Clusters/Domains/Resources)  
✅ Bilingual documentation  
✅ Issue-driven development

---

## Priority Matrix

### Must Have (Phase 1)
1. ⭐ Workspace dependency catalog
2. ⭐ Standardized package structure
3. ⭐ ORM strategy (SeaORM)
4. ⭐ Testing infrastructure
5. ⭐ Code quality tools and git hooks

### Should Have (Phase 1-2)
6. 🔶 Validation strategy (validator crate)
7. 🔶 Rate limiting (governor)
8. 🔶 API documentation (utoipa)
9. 🔶 Hot reload workflow

### Nice to Have (Phase 2-3)
10. 🔵 Build orchestration (cargo-make)
11. 🔵 Advanced logging (tracing)
12. 🔵 Error handling patterns
13. 🔵 CORS configuration
14. 🔵 File upload handling

---

## Recommendations for Plan Updates

### Constitution Updates Needed

**Add Principle XVI: Workspace Dependency Catalog**
```markdown
### XVI. Workspace Dependency Catalog

All shared dependencies MUST be defined in root Cargo.toml [workspace.dependencies]:
- Ensures version consistency across all packages
- Simplifies dependency upgrades
- Prevents version conflicts
- Provides single source of truth

Packages MUST reference workspace dependencies with `workspace = true`.
```

### Specification Updates Needed

1. **Add ORM requirements** (FR-046 to FR-050):
   - FR-046: Database operations MUST use SeaORM as primary ORM
   - FR-047: Complex queries MAY use sqlx for type-safe SQL
   - FR-048: All schema changes MUST use migrations
   - FR-049: Entity models MUST derive SeaORM Entity trait
   - FR-050: Migration files MUST be in package migrations/ directory

2. **Add validation requirements** (FR-051 to FR-053):
   - FR-051: Input validation MUST use validator crate
   - FR-052: All API request types MUST implement Validate trait
   - FR-053: Validation errors MUST return 400 with error details

3. **Update NFR-011** (Rate limiting):
   - Specify governor crate for rate limiting
   - Define default rate limits (e.g., 100 requests per 15 minutes)

4. **Add API documentation requirements** (FR-054 to FR-056):
   - FR-054: API endpoints MUST be documented with utoipa macros
   - FR-055: OpenAPI spec MUST be auto-generated from code
   - FR-056: Swagger UI MUST be available in development

5. **Add testing tool requirements** (TST-014 to TST-016):
   - TST-014: Unit tests MUST use cargo test
   - TST-015: WASM tests MUST use wasm-bindgen-test
   - TST-016: Coverage MUST be measured with cargo tarpaulin

6. **Add git hooks requirements** (CI-007 to CI-009):
   - CI-007: Pre-commit hooks MUST check formatting and linting
   - CI-008: cargo-husky MUST be used for git hooks
   - CI-009: Quality checks MUST run before commit

### Implementation Roadmap Updates Needed

**Phase 1 Week 1 - Add:**
- Set up workspace dependencies catalog in root Cargo.toml
- Install development tools (cargo-watch, trunk, cargo-husky)
- Configure git hooks for quality checks

**Phase 1 Week 2 - Add:**
- Define SeaORM entity patterns
- Set up migration workflow
- Add validator crate patterns
- Configure utoipa for API docs

**Phase 1 Week 3 - Add:**
- Set up cargo tarpaulin for coverage
- Create testing guide documentation
- Configure CI/CD with all quality gates

---

## Conclusion

This analysis identified **15 additional architectural patterns** from the React repository that were not fully documented in the current Rust plans. Of these:

- **5 patterns** are HIGH PRIORITY and must be added to Phase 1
- **4 patterns** are MEDIUM PRIORITY and should be in Phase 1-2
- **6 patterns** are NICE TO HAVE for Phase 2-3

Key actions required:
1. ✅ Update constitution with Principle XVI (Workspace Dependencies)
2. ✅ Add 13 new functional requirements to spec.md
3. ✅ Update implementation roadmap with specific tools and workflows
4. ✅ Create development workflow guide
5. ✅ Set up project template with all patterns

**Next Steps:**
1. Review and approve this analysis
2. Update constitution document
3. Update specification document  
4. Update implementation roadmap
5. Begin Phase 1 implementation with all patterns

---

**Document Version**: 1.0  
**Author**: Architecture Analysis  
**Approved By**: [Pending]  
**Related Documents**: 
- ARCHITECTURAL-COMPARISON.md
- IMPLEMENTATION-ROADMAP.md
- .specify/memory/constitution.md
- spec.md
