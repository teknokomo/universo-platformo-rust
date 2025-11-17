# Research: Initialize Universo Platformo Rust Project

**Feature**: 001-initialize-rust-platformo  
**Date**: 2025-11-17  
**Status**: Complete

## Overview

This research document consolidates findings for initializing the Universo Platformo Rust project, covering technology choices, best practices, and architectural decisions for the monorepo structure with shared infrastructure packages.

## Research Areas

### 1. Cargo Workspace Management for Monorepos

**Decision**: Use Cargo workspaces with centralized dependency management

**Rationale**:
- Cargo workspaces provide native Rust support for monorepo structures
- Shared dependencies across all packages reduce duplication and version conflicts
- Unified build command (`cargo build --workspace`) compiles all packages
- Efficient incremental compilation when packages share dependencies
- Native support in rust-analyzer for IDE integration across packages

**Best Practices**:
- Define all packages in root `Cargo.toml` `[workspace]` section
- Use workspace dependencies (`[workspace.dependencies]`) for shared crates
- Package-specific dependencies reference workspace versions: `serde = { workspace = true }`
- Place common dev-dependencies at workspace level
- Use `cargo workspaces` crate for advanced multi-package operations (publishing, versioning)

**Alternatives Considered**:
- Multiple independent repositories: Rejected due to dependency management complexity and difficulty coordinating changes
- Git submodules: Rejected due to poor ergonomics and CI/CD complications
- Monorepo with manual dependency management: Rejected in favor of native Cargo workspace features

**References**:
- [The Cargo Book - Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo Workspace Best Practices](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)

---

### 2. Yew Framework for Frontend (WebAssembly)

**Decision**: Yew 0.21+ for frontend with wasm-pack/trunk build tooling

**Rationale**:
- Mature Rust framework for building web UIs with WebAssembly
- Component-based architecture similar to React (easier conceptual port)
- Strong type safety across frontend codebase
- Virtual DOM for efficient rendering
- Active community and good documentation
- Hooks API for state management (similar to React hooks)

**Best Practices**:
- Use function components with hooks for new code
- Leverage `use_state`, `use_effect`, `use_callback` for state management
- Component props should derive `Properties` trait
- Use `yew-router` for client-side routing
- HTML macro for type-safe templating: `html! { <div>...</div> }`
- Async operations via `use_async` or spawn_local with `wasm-bindgen-futures`

**Build Tooling**:
- **Primary**: Trunk for development (hot reload, asset bundling)
- **Alternative**: wasm-pack for library crates
- Asset handling: Place assets in root `assets/` or `static/`, reference in HTML
- WASM optimization: `wasm-opt` for size reduction in release builds

**Alternatives Considered**:
- Leptos: Rejected as newer/less mature, though promising for future
- Seed: Rejected due to smaller community and less active development
- Sycamore: Rejected due to different reactivity model and smaller ecosystem
- Dioxus: Rejected as it targets multiple platforms; we focus on web

**References**:
- [Yew Documentation](https://yew.rs/)
- [Trunk Build Tool](https://trunkrs.dev/)
- [WebAssembly Best Practices](https://rustwasm.github.io/book/)

---

### 3. Actix Web for Backend

**Decision**: Actix Web 4.x for backend HTTP server and API

**Rationale**:
- High-performance async web framework built on Tokio
- Actor-based architecture for concurrent request handling
- Extensive middleware ecosystem (CORS, compression, logging)
- Excellent performance benchmarks (top-tier in TechEmpower)
- Strong community support and extensive documentation
- Native async/await support with Rust's async runtime

**Best Practices**:
- Structure: Organize by feature (routes, handlers, services, models per feature)
- Use `actix-web::web::Data` for shared application state
- Extractors for request data: `Json<T>`, `Path<T>`, `Query<T>`
- Middleware for cross-cutting concerns (auth, logging, error handling)
- Error handling: Implement `ResponseError` trait for custom errors
- Testing: Use `actix-web::test` utilities for integration tests

**Authentication**:
- Use `actix-web-httpauth` for JWT or session-based auth
- Supabase integration via custom middleware verifying tokens
- Session storage: Redis or in-memory for development

**Alternatives Considered**:
- Axum: Rejected due to less mature ecosystem, though architecture is cleaner
- Rocket: Rejected due to slower adoption of async and less flexibility
- Warp: Rejected due to steeper learning curve with filter-based API
- Tide: Rejected as development has slowed significantly

**References**:
- [Actix Web Documentation](https://actix.rs/)
- [Actix Web Examples](https://github.com/actix/examples)
- [Building REST APIs with Actix Web](https://dev.to/wpoch/rest-api-in-rust-with-actix-web-4-3fk)

---

### 4. Database Abstraction with Supabase

**Decision**: Supabase (PostgreSQL) with trait-based repository pattern

**Rationale**:
- Supabase provides PostgreSQL with REST API, authentication, and real-time subscriptions
- Initial implementation focuses on Supabase for rapid development
- Trait-based abstraction enables future DBMS support without rewriting business logic
- SQLx for type-safe SQL queries with compile-time verification

**Best Practices**:
- Define repository traits for each entity (e.g., `ClusterRepository`)
- Implement traits for Supabase using SQLx
- Use SQLx compile-time query checking: `query_as!` macro
- Database migrations with SQLx CLI: `sqlx migrate add <name>`
- Connection pooling: `sqlx::PgPool` shared via `actix-web::web::Data`
- Async/await for all database operations

**Repository Pattern**:
```rust
#[async_trait]
pub trait ClusterRepository {
    async fn create(&self, cluster: NewCluster) -> Result<Cluster, DbError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Cluster>, DbError>;
    async fn list(&self, pagination: Pagination) -> Result<Vec<Cluster>, DbError>;
    async fn update(&self, id: Uuid, updates: ClusterUpdate) -> Result<Cluster, DbError>;
    async fn delete(&self, id: Uuid) -> Result<(), DbError>;
}
```

**Supabase Integration**:
- Direct PostgreSQL connection via SQLx (not REST API)
- Auth verification via Supabase JWT validation
- Environment variables for connection strings and API keys

**Alternatives Considered**:
- Diesel: Rejected due to sync-only nature (async support incomplete)
- SeaORM: Rejected as newer, less battle-tested
- Direct SQL: Rejected due to lack of type safety and migration management

**References**:
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Supabase with Rust](https://supabase.com/docs/guides/integrations)
- [Repository Pattern in Rust](https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html)

---

### 5. Material Design UI Components for Rust/Yew

**Decision**: Custom Yew components following Material Design principles

**Rationale**:
- No mature Material Design library for Yew exists yet
- Build custom components in `universo-ui-components` package
- Follow Material Design 3 specifications for consistency
- Reusable across all frontend packages
- Can integrate with CSS frameworks for styling (Tailwind, vanilla CSS)

**Component Strategy**:
- Core components: Button, TextField, Card, AppBar, Drawer, List, Dialog
- Layout components: Grid, Flexbox wrappers
- Form components: Input, Select, Checkbox, Radio, Switch
- Use CSS Modules or inline styles with Yew
- Accessibility: ARIA attributes, keyboard navigation, focus management

**Styling Approach**:
- Inline styles via Yew: `style="color: blue;"`
- CSS classes: Define in stylesheet, reference via `class="button-primary"`
- CSS-in-Rust: Use `stylist` crate for scoped styles
- Theming: CSS variables for colors, spacing, typography

**Alternatives Considered**:
- Patternfly: Rejected as it's Red Hat specific and not Material Design
- Material-Yew: Rejected as unmaintained/experimental
- Port Material-UI: Rejected due to complexity and JavaScript dependencies
- Seed UI: Rejected as tied to different framework

**References**:
- [Material Design 3](https://m3.material.io/)
- [Yew Styling Guide](https://yew.rs/docs/concepts/html/classes)
- [Stylist for Yew](https://github.com/futursolo/stylist-rs)

---

### 6. Internationalization (i18n) for Rust

**Decision**: `fluent-rs` for internationalization with English and Russian support

**Rationale**:
- Industry-standard localization system by Mozilla
- Supports complex pluralization rules and grammar
- Async-friendly and works with both backend and frontend
- FTL (Fluent Translation List) format is readable and maintainable
- Compile-time checks available via macros

**Best Practices**:
- Store translations in `locales/{en,ru}/` directories
- FTL files per feature: `auth.ftl`, `clusters.ftl`, etc.
- Load translations at startup (backend) or bundle with WASM (frontend)
- Fallback language: English
- Context-aware translations for ambiguous terms
- Translation keys in code: `fl!("welcome-message")`

**Structure**:
```
universo-i18n/
└── base/
    ├── locales/
    │   ├── en/
    │   │   ├── common.ftl
    │   │   └── clusters.ftl
    │   └── ru/
    │       ├── common.ftl
    │       └── clusters.ftl
    └── src/
        └── lib.rs
```

**Alternatives Considered**:
- `rust-i18n`: Simpler but less powerful, no pluralization support
- `gettext`: Rejected as C-based with poor Rust integration
- Custom solution: Rejected due to complexity of pluralization and grammar rules

**References**:
- [Fluent Project](https://projectfluent.org/)
- [fluent-rs Documentation](https://github.com/projectfluent/fluent-rs)
- [i18n Best Practices](https://mozilla-l10n.github.io/localizer-documentation/)

---

### 7. HTTP Client for Frontend-Backend Communication

**Decision**: `reqwest` for HTTP client in `universo-api-client` package

**Rationale**:
- Most popular Rust HTTP client with excellent async support
- Works in both native (backend-to-backend) and WASM (frontend) contexts
- Type-safe API via serde integration
- Middleware support for auth tokens, retry logic, logging
- WebAssembly compatibility via `reqwest-wasm` feature

**Best Practices**:
- Centralized client in `universo-api-client` package
- Type-safe methods using shared types from `universo-types`
- Automatic JSON serialization/deserialization with serde
- Auth token injection via middleware or request headers
- Error handling: Map HTTP errors to domain-specific error types
- Retry logic for transient failures

**API Client Structure**:
```rust
pub struct UniversoApiClient {
    client: reqwest::Client,
    base_url: String,
    auth_token: Option<String>,
}

impl UniversoApiClient {
    pub async fn create_cluster(&self, request: CreateClusterRequest) 
        -> Result<Cluster, ApiError> { ... }
}
```

**Alternatives Considered**:
- `surf`: Rejected due to smaller community and less WASM support
- `ureq`: Rejected as synchronous-only
- `hyper`: Rejected as too low-level for application code
- Direct `fetch` API: Rejected due to lack of type safety

**References**:
- [reqwest Documentation](https://docs.rs/reqwest/)
- [reqwest WASM Support](https://github.com/seanmonstar/reqwest/tree/master/wasm)

---

### 8. Build Tooling and CI/CD Pipeline

**Decision**: Trunk for frontend, Cargo for backend, GitHub Actions for CI/CD

**Rationale**:
- Trunk provides hot-reload and asset bundling for Yew apps
- Native Cargo commands for backend and workspace management
- GitHub Actions has excellent Rust toolchain support
- Free for public repositories, integrated with GitHub Issues/PRs

**Development Workflow**:
- Frontend: `trunk serve` in package directory for hot-reload
- Backend: `cargo watch -x run` for auto-restart on changes
- Full build: `cargo build --workspace --release`
- Testing: `cargo test --workspace`
- Linting: `cargo clippy --workspace -- -D warnings`
- Formatting: `cargo fmt --all -- --check`

**CI/CD Pipeline Steps**:
1. Checkout code
2. Install Rust toolchain (stable, with wasm32 target)
3. Cache Cargo dependencies
4. Run `cargo fmt --all -- --check`
5. Run `cargo clippy --workspace -- -D warnings`
6. Run `cargo test --workspace`
7. Run `cargo build --workspace --release`
8. Build frontend packages with Trunk
9. Run `cargo audit` for security vulnerabilities
10. Upload build artifacts

**Optimization**:
- Use `sccache` or `cargo-cache` for faster builds
- Incremental compilation in development
- LTO (Link-Time Optimization) in release builds
- WASM optimization: `wasm-opt -Oz` for size reduction

**Alternatives Considered**:
- wasm-pack: Rejected for apps in favor of Trunk (used for libraries)
- GitLab CI: Rejected as project uses GitHub
- Jenkins: Rejected due to self-hosting complexity

**References**:
- [Trunk Documentation](https://trunkrs.dev/)
- [GitHub Actions for Rust](https://github.com/actions-rs)
- [Cargo Watch](https://github.com/watchexec/cargo-watch)

---

### 9. Testing Strategy for Rust Fullstack

**Decision**: Multi-level testing with unit, integration, and contract tests

**Rationale**:
- Rust's test framework is built-in and excellent
- Co-located tests improve maintainability
- Integration tests verify API contracts
- WASM tests validate frontend logic
- Test coverage with `cargo-tarpaulin`

**Testing Levels**:
1. **Unit Tests**: In module `#[cfg(test)] mod tests`
2. **Integration Tests**: In `tests/` directory per package
3. **Contract Tests**: Verify API request/response shapes
4. **WASM Tests**: Use `wasm-bindgen-test` for frontend

**Best Practices**:
- Test public APIs, not implementation details
- Use test fixtures for database tests
- Mock external dependencies (HTTP, database)
- Async tests with `#[actix_rt::test]` or `#[tokio::test]`
- Snapshot testing for complex data structures
- Property-based testing with `proptest` for edge cases

**Test Organization**:
```
packages/clusters-srv/
├── src/
│   ├── lib.rs
│   ├── handlers.rs
│   └── handlers/
│       └── tests.rs      # Unit tests co-located
└── tests/
    ├── integration.rs     # Integration tests
    └── fixtures/
        └── clusters.json  # Test data
```

**Coverage Target**: 70% minimum for new code

**Alternatives Considered**:
- End-to-end tests with Playwright: Deferred to future (complex setup)
- Mutation testing: Deferred to future (performance cost)

**References**:
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)

---

### 10. Accessibility (WCAG 2.1 Level AA) for Yew Components

**Decision**: Build accessibility into custom UI components from day one

**Rationale**:
- Legal requirement in many jurisdictions
- Improves usability for all users
- Easier to build in than retrofit
- WCAG 2.1 Level AA is achievable standard

**Implementation Strategy**:
- **Keyboard Navigation**: All interactive elements accessible via Tab, Enter, Space, Arrow keys
- **ARIA Attributes**: Use appropriate roles, labels, and states
- **Focus Management**: Visible focus indicators, focus trap for modals
- **Color Contrast**: Minimum 4.5:1 for normal text, 3:1 for large text
- **Screen Reader Support**: Test with NVDA/JAWS (Windows), VoiceOver (macOS/iOS)
- **Form Validation**: Error messages announced to screen readers

**Component Requirements**:
- Button: `role="button"`, `aria-label` if no text
- Input: Associated `<label>`, `aria-invalid` for errors, `aria-describedby` for hints
- Modal: Focus trap, `role="dialog"`, `aria-modal="true"`, focus return on close
- Navigation: `<nav>` element, `aria-current` for active page
- Lists: `<ul>`/`<ol>` with `<li>`, not `<div>` soup

**Testing**:
- Automated: `axe-core` via browser DevTools
- Manual: Keyboard-only navigation testing
- Screen reader: Test critical flows with actual screen readers

**Alternatives Considered**:
- Ignore accessibility: Rejected as unethical and risky
- WCAG AAA: Rejected as too strict for initial implementation

**References**:
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [WebAIM Resources](https://webaim.org/resources/)

---

## Summary

All technical decisions are made with concrete rationale and alternatives considered. The technology stack (Rust, Yew, Actix Web, Supabase, SQLx, reqwest, fluent-rs) provides a solid foundation for building a performant, type-safe, accessible fullstack web platform. Shared infrastructure packages prevent code duplication and ensure consistency across domain features.

**Next Steps**: Proceed to Phase 1 to create data models, API contracts, and quickstart documentation.
