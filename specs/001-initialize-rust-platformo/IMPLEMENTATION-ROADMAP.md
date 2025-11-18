# Implementation Roadmap

## Overview

This document provides a practical roadmap for implementing Universo Platformo Rust based on the architectural analysis of the React version and updated specifications (v3.1.0).

**CRITICAL**: ALL functionality MUST be implemented as packages in `packages/` directory. This is NON-NEGOTIABLE.

**Last Updated**: 2025-11-17  
**Constitution Version**: 1.5.0 (Strengthened modular requirements)
**Specification Version**: 3.1.0 (Added mandatory package structure)

## Quick Reference

- **Architectural Analysis**: See `ARCHITECTURAL-COMPARISON.md` for detailed comparison with React implementation
- **Constitution**: See `.specify/memory/constitution.md` for core principles
- **Full Specification**: See `spec.md` for complete requirements

## Phase 1: Foundation + Shared Infrastructure (3-4 weeks)

### Week 1: Repository Setup and Build Infrastructure

**Goal**: Establish basic repository structure with proper build tooling.

**Tasks**:
1. Configure Cargo workspace in root `Cargo.toml`
2. Create `rust-toolchain.toml` specifying stable Rust version
3. Set up `.gitignore` for Rust projects (target/, Cargo.lock for libraries, etc.)
4. Configure GitHub Actions CI/CD:
   - `cargo build --workspace`
   - `cargo test --workspace`
   - `cargo clippy --workspace -- -D warnings`
   - `cargo fmt --all -- --check`
   - `cargo audit` for security
5. Document WASM build process with `wasm-pack` or `trunk`
6. Set up `cargo-watch` for development hot-reload

**Deliverables**:
- Working Cargo workspace
- CI/CD pipeline passing
- Build documentation in README

**Validation**:
```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

### Week 2: Shared Infrastructure - Types and Utils

**Goal**: Create foundation packages that all other packages will depend on.

**Tasks**:

#### 1. universo-types (`packages/universo-types/base/`)

```rust
// src/lib.rs structure
pub mod entities;      // Cluster, Domain, Resource, User, etc.
pub mod api;          // Request/Response types
pub mod updl;         // UPDL node definitions
pub mod platform;     // Platform-specific types

// All structs must have:
#[derive(Debug, Clone, Serialize, Deserialize)]
```

**Key types to implement**:
- Entity types: `Cluster`, `Domain`, `Resource`
- API contracts: `CreateClusterRequest`, `ClusterResponse`
- UPDL nodes: `UPDLScene`, `UPDLEntity`, `UPDLTransform`, etc.
- Common types: `Uuid`, `DateTime<Utc>` (re-exports)

#### 2. universo-utils (`packages/universo-utils/base/`)

```rust
// src/lib.rs structure
pub mod updl_processor; // Convert flow graphs to UPDL
pub mod template;       // Template trait interface
pub mod validation;     // Common validation utilities
pub mod error;         // Error types
```

**Key implementations**:
- `UPDLProcessor::process_flow(flow_data: FlowData) -> Result<UPDL>`
- `Template` trait for template system
- Validation helpers

**Deliverables**:
- `universo-types` crate with comprehensive type system
- `universo-utils` crate with UPDL processor foundation
- Unit tests for all types and processors
- Documentation with examples

**Validation**:
```bash
cargo test -p universo-types
cargo test -p universo-utils
cargo doc -p universo-types --open
cargo doc -p universo-utils --open
```

### Week 3: Shared Infrastructure - API Client and I18n

**Goal**: Complete shared infrastructure with API client and internationalization.

**Tasks**:

#### 3. universo-api-client (`packages/universo-api-client/base/`)

```rust
// src/lib.rs structure
pub mod client;        // Base HTTP client
pub mod clusters;      // Cluster API methods
pub mod auth;         // Authentication API
pub mod error;        // API error types

// Example API structure
pub struct ClusterApi {
    client: reqwest::Client,
    base_url: String,
}

impl ClusterApi {
    pub async fn list(&self) -> Result<Vec<ClusterResponse>, ApiError>;
    pub async fn create(&self, req: CreateClusterRequest) -> Result<ClusterResponse, ApiError>;
    pub async fn get(&self, id: Uuid) -> Result<ClusterResponse, ApiError>;
    pub async fn update(&self, id: Uuid, req: UpdateClusterRequest) -> Result<ClusterResponse, ApiError>;
    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}
```

#### 4. universo-i18n (`packages/universo-i18n/base/`)

**Option A: fluent-rs (Recommended)**
```rust
// locales/en/common.ftl
greeting = Hello, { $name }!
clusters-title = Clusters Management

// locales/ru/common.ftl
greeting = Здравствуйте, { $name }!
clusters-title = Управление кластерами
```

**Option B: rust-i18n (Simpler)**
```yaml
# locales/en.yml
en:
  greeting: "Hello, %{name}!"
  clusters:
    title: "Clusters Management"

# locales/ru.yml
ru:
  greeting: "Здравствуйте, %{name}!"
  clusters:
    title: "Управление кластерами"
```

**Deliverables**:
- `universo-api-client` with reqwest-based HTTP client
- `universo-i18n` with EN/RU translations
- Integration tests for API client (with mock server)
- Translation usage examples

**Validation**:
```bash
cargo test -p universo-api-client
cargo test -p universo-i18n
```

### Week 4: Shared Infrastructure - UI Components

**Goal**: Create reusable Yew components following Material Design.

**Tasks**:

#### 5. universo-ui-components (`packages/universo-ui-components/base/`)

**Initial component set** (based on React duplication analysis):
- Button (variants: text, contained, outlined)
- TextField (text input with validation)
- Dialog (modal dialogs)
- Card (content container)
- AppBar (top navigation)
- Drawer (side navigation)
- Alert (feedback messages)
- DataGrid (tables with sorting/filtering)

**Component structure**:
```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub variant: ButtonVariant,
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            class={format!("btn btn-{}", props.variant)}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
        >
            { &props.label }
        </button>
    }
}
```

**Deliverables**:
- Basic Material Design component library
- Component documentation with examples
- Storybook-like demo page (WASM app)
- CSS/SCSS for styling

**Validation**:
```bash
cd packages/universo-ui-components/base
trunk serve  # View components in browser
```

## Phase 2: UPDL System and Clusters Feature (5-7 weeks)

### Weeks 5-6: UPDL System Implementation

**Goal**: Complete UPDL type system and processor.

**Tasks**:

#### UPDL Type Definitions (in universo-types)

```rust
// packages/universo-types/base/src/updl/mod.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UPDL {
    pub version: String,
    pub scenes: Vec<UPDLScene>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UPDLScene {
    pub id: Uuid,
    pub name: String,
    pub entities: Vec<UPDLEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UPDLEntity {
    pub id: Uuid,
    pub name: String,
    pub entity_type: EntityType,
    pub transform: Option<Transform>,
    pub material: Option<Material>,
    pub children: Vec<UPDLEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EntityType {
    Object { mesh: String },
    Camera { fov: f32, aspect: f32 },
    Light { color: Color, intensity: f32 },
    Primitive { shape: PrimitiveShape },
}

// Continue with all node types...
```

#### UPDL Processor (in universo-utils)

```rust
// packages/universo-utils/base/src/updl_processor.rs

pub struct UPDLProcessor;

impl UPDLProcessor {
    pub fn process_flow(flow_data: &FlowData) -> Result<UPDL, ProcessError> {
        // 1. Parse flow graph nodes and edges
        // 2. Validate node connections
        // 3. Resolve node properties
        // 4. Build UPDL structure
        // 5. Validate final UPDL
        todo!()
    }
    
    fn validate_connections(nodes: &[Node], edges: &[Edge]) -> Result<(), ValidationError> {
        // Validate that Scene nodes have valid child connections
        // Check for cycles
        // Verify required inputs are connected
        todo!()
    }
    
    fn build_scene_tree(nodes: &[Node], edges: &[Edge]) -> Result<UPDLScene, BuildError> {
        // Traverse graph starting from Scene nodes
        // Build hierarchical structure
        todo!()
    }
}
```

**Deliverables**:
- Complete UPDL type definitions (15+ node types)
- UPDL processor with validation
- Comprehensive test suite with fixture files
- UPDL specification document
- Examples for each node type

**Validation**:
```bash
cargo test -p universo-types -- updl
cargo test -p universo-utils -- updl_processor
```

### Weeks 7-9: Clusters Feature Implementation

**Goal**: First complete domain feature using established patterns.

**Tasks**:

#### 1. Backend Package (`packages/clusters-srv/base/`)

**Database Schema** (SQLx migrations):
```sql
-- migrations/001_create_clusters.sql
CREATE TABLE clusters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_clusters_owner ON clusters(owner_id);
CREATE INDEX idx_clusters_deleted ON clusters(deleted_at) WHERE deleted_at IS NULL;

-- Similar for domains and resources...
```

**API Endpoints**:
```rust
// src/routes/clusters.rs
use actix_web::{web, HttpResponse};
use universo_types::api::{CreateClusterRequest, ClusterResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/clusters")
            .route("", web::get().to(list_clusters))
            .route("", web::post().to(create_cluster))
            .route("/{id}", web::get().to(get_cluster))
            .route("/{id}", web::put().to(update_cluster))
            .route("/{id}", web::delete().to(delete_cluster))
    );
}

async fn list_clusters(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let clusters = cluster_service::list(&pool).await?;
    Ok(HttpResponse::Ok().json(clusters))
}

// Implement other handlers...
```

#### 2. Frontend Package (`packages/clusters-frt/base/`)

**Yew Components**:
```rust
// src/pages/cluster_list.rs
use yew::prelude::*;
use yew_router::prelude::*;
use universo_ui_components::{DataGrid, Button};
use universo_api_client::ClusterApi;

#[function_component(ClusterList)]
pub fn cluster_list() -> Html {
    let clusters = use_state(|| Vec::new());
    let api = use_context::<ClusterApi>().expect("ClusterApi not found");
    
    use_effect_with_deps(
        move |_| {
            let clusters = clusters.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api.list().await {
                    Ok(data) => clusters.set(data),
                    Err(e) => log::error!("Failed to load clusters: {:?}", e),
                }
            });
        },
        (),
    );
    
    html! {
        <div class="cluster-list">
            <h1>{ t!("clusters.title") }</h1>
            <Button label={t!("clusters.create")} onclick={/* ... */} />
            <DataGrid data={(*clusters).clone()} />
        </div>
    }
}
```

**Deliverables**:
- Complete backend with CRUD operations
- Database migrations
- Frontend UI for cluster management
- Integration tests (backend)
- End-to-end tests (with wasm-bindgen-test)
- Documentation following three-entity pattern

**Validation**:
```bash
# Backend
cargo test -p clusters-srv
cargo run -p clusters-srv

# Frontend
cd packages/clusters-frt/base
trunk serve
# Open http://localhost:8080 and test UI
```

### Week 10-11: Integration and Polish

**Goal**: Complete Phase 2 with integrated UPDL and Clusters feature.

**Tasks**:
1. Integration testing between frontend and backend
2. UPDL integration with visual editor (if available)
3. Performance optimization
4. Documentation finalization
5. Demo deployment

**Deliverables**:
- End-to-end working demo
- Phase 2 completion report
- Pattern documentation for replication

## Phase 3: Template System (8-10 weeks)

### Overview

This phase implements the template system for multi-platform export, which is the core value proposition of Universo Platformo.

### Weeks 12-14: Template Interface and AR.js Template

**Goal**: Define template interface and create first template.

**Tasks**:

#### 1. Template Trait (in universo-utils)

```rust
// packages/universo-utils/base/src/template.rs

pub trait Template {
    fn name(&self) -> &str;
    fn platform(&self) -> &str;
    
    fn validate_updl(&self, updl: &UPDL) -> Result<(), ValidationError>;
    
    fn process(&self, updl: &UPDL) -> Result<BuildArtifacts, ProcessError>;
    
    fn supported_nodes(&self) -> Vec<NodeType>;
}

pub struct BuildArtifacts {
    pub html: String,
    pub js: Option<String>,
    pub css: Option<String>,
    pub assets: HashMap<String, Vec<u8>>,
}
```

#### 2. AR.js Template (`packages/template-arjs/base/`)

```rust
// src/lib.rs

use universo_utils::template::Template;
use universo_types::updl::UPDL;

pub struct ARJSTemplate;

impl Template for ARJSTemplate {
    fn name(&self) -> &str { "AR.js Educational Quiz" }
    fn platform(&self) -> &str { "arjs" }
    
    fn validate_updl(&self, updl: &UPDL) -> Result<(), ValidationError> {
        // Check for required AR.js features
        // Validate marker configuration
        // Ensure camera setup is correct
        todo!()
    }
    
    fn process(&self, updl: &UPDL) -> Result<BuildArtifacts, ProcessError> {
        // Generate A-Frame HTML
        let html = self.generate_aframe_html(updl)?;
        
        // Generate quiz logic JavaScript
        let js = self.generate_quiz_logic(updl)?;
        
        // Package assets
        let assets = self.collect_assets(updl)?;
        
        Ok(BuildArtifacts { html, js: Some(js), css: None, assets })
    }
    
    fn supported_nodes(&self) -> Vec<NodeType> {
        vec![
            NodeType::Scene,
            NodeType::Camera,
            NodeType::Light,
            NodeType::Primitive,
            NodeType::Text,
            NodeType::Image,
            NodeType::Marker,
        ]
    }
}
```

**Deliverables**:
- Template trait interface
- AR.js template implementation
- Generated AR.js experiences working in browser
- Template documentation and examples

### Weeks 15-17: PlayCanvas Template

**Goal**: Second template platform demonstrating pattern reusability.

**Tasks**: Similar to AR.js but for PlayCanvas engine.

### Weeks 18-19: Publication System

**Goal**: Allow sharing exported experiences via URLs.

**Tasks**:
1. Backend: Storage and serving of generated artifacts
2. Frontend: Export UI with template selection
3. URL generation for published experiences
4. Versioning and update support

## Critical Decision Points

### Decision 1: Visual Flow Editor (Week 2)

**Question**: Which approach for visual flow editor?

**Options**:
1. **Pure Rust/WASM**: Build custom flow editor in Yew
   - Pros: Full Rust stack, no JS dependencies
   - Cons: Huge development effort (6+ months), no existing libraries
   
2. **Hybrid JS+WASM**: Use React Flow in JS, connect via WASM
   - Pros: Proven solution, fast to implement
   - Cons: Mixed tech stack, complexity at boundary
   
3. **Defer to Phase 4**: Start with JSON import/export, build editor later
   - Pros: Focus on core functionality first
   - Cons: Poor user experience initially

**Recommendation**: Option 2 (Hybrid) or 3 (Defer) depending on timeline pressure.

**Risk**: RISK-009 - Visual Editor Availability

### Decision 2: Database Layer (Week 2)

**Question**: SQLx or Diesel?

**Comparison**:
- **SQLx**: Async, compile-time verification, PostgreSQL optimized
  - ✅ Better for Actix Web (async)
  - ✅ Query verification at compile time
  - ❌ Less mature than Diesel
  
- **Diesel**: Sync, most mature, schema-first
  - ✅ Very mature and stable
  - ✅ Great migration tools
  - ❌ Blocking I/O (need thread pool with Actix)

**Recommendation**: SQLx for async compatibility with Actix Web.

### Decision 3: UI Component Library (Week 4)

**Question**: Build custom or use existing?

**Options**:
1. **Yew Material**: If available and maintained
2. **MUI-like custom**: Build Material Design components ourselves
3. **Patternfly Yew**: Alternative component library

**Recommendation**: Evaluate Yew Material first, fall back to custom if needed.

## Success Metrics

### Phase 1 Success Criteria
- ✅ All shared packages compile and pass tests
- ✅ CI/CD pipeline green
- ✅ Documentation complete and bilingual
- ✅ WASM builds working

### Phase 2 Success Criteria
- ✅ UPDL processor converts flow graphs correctly
- ✅ Clusters CRUD fully functional
- ✅ Frontend and backend integrated
- ✅ Demo deployment accessible

### Phase 3 Success Criteria
- ✅ Templates generate working experiences
- ✅ AR.js and PlayCanvas templates functional
- ✅ Published experiences accessible via URL
- ✅ Template pattern documented for future platforms

## Resources and References

### Essential Reading
1. `ARCHITECTURAL-COMPARISON.md` - Detailed React analysis
2. Constitution v1.3.0 - Core principles
3. Specification v3.0.0 - Complete requirements
4. React Repository - https://github.com/teknokomo/universo-platformo-react

### Rust Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Actix Web Guide](https://actix.rs/docs/)
- [Yew Guide](https://yew.rs/docs/getting-started/introduction)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)

### Development Tools
- [cargo-watch](https://crates.io/crates/cargo-watch) - Hot reload
- [trunk](https://trunkrs.dev/) - WASM bundler
- [cargo-audit](https://crates.io/crates/cargo-audit) - Security scanning
- [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) - Code coverage

## Conclusion

This roadmap provides a structured approach to implementing Universo Platformo Rust based on lessons learned from the React implementation. The key innovation is prioritizing shared infrastructure and UPDL system early, rather than discovering their necessity through painful refactoring.

**Remember**: The goal is not to replicate the React version's mistakes but to build a cleaner, more maintainable implementation that incorporates all lessons learned.

---

**Document Version**: 1.0  
**Last Updated**: 2025-11-17  
**Next Review**: Start of Phase 1 implementation
