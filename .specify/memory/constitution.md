<!--
Sync Impact Report - Constitution v1.5.0
========================================
Version Change: 1.4.0 → 1.5.0 (Strengthened modular architecture requirements)
Date: 2025-11-17

Principles Updated in v1.5.0:
- II. Package Structure Convention - MAJOR UPDATE: Added NON-NEGOTIABLE status, explicit prohibition of functionality outside packages/, future extraction requirements, and repository root restrictions

Principles Added in v1.4.0:
- XVI. Workspace Dependency Catalog (NEW)
- XVII. Development Workflow Standardization (NEW)

Principles Added in v1.3.0:
- XII. Shared Infrastructure Priority
- XIII. Template System Architecture
- XIV. UPDL as Core Abstraction
- XV. Build Tooling Strategy

Principles Added in v1.2.0:
- IX. Non-Functional Requirements Priority
- X. Integration Contracts
- XI. Risk Management

Principles Added in v1.1.0:
- VIII. Repository Boundaries and Exclusions

Principles Updated in v1.1.0:
- VII. Best Practices for Rust Fullstack - Added acknowledgment of React version's partial state and Flowise legacy

Previous Version (v1.0.0):
Principles Added:
- I. Monorepo with Rust Workspace
- II. Package Structure Convention  
- III. Bilingual Documentation (NON-NEGOTIABLE)
- IV. Database Flexibility
- V. Issue-Driven Development
- VI. Specification-First Approach
- VII. Best Practices for Rust Fullstack

Sections Added:
- Technology Stack
- Development Workflow
- Governance

Templates Status:
✅ plan-template.md - Reviewed, no changes needed (generic structure)
✅ spec-template.md - Reviewed, no changes needed (generic structure)
✅ tasks-template.md - Reviewed, no changes needed (generic structure)

Follow-up TODOs:
- Update specifications with shared infrastructure requirements
- Add UPDL system to Phase 2 deliverables
- Document template system architecture
-->

# Universo Platformo Rust Constitution

## Core Principles

### I. Monorepo with Rust Workspace

The project MUST be organized as a monorepo using Cargo workspace management. All packages MUST be defined in the root `Cargo.toml` workspace configuration. This enables unified dependency management, consistent tooling, and efficient cross-package development.

**Rationale**: Monorepo structure facilitates code sharing, simplifies dependency management, and enables atomic cross-package changes. Cargo workspaces provide native Rust tooling support for this pattern.

### II. Package Structure Convention (NON-NEGOTIABLE)

**CRITICAL**: ALL application functionality MUST be implemented as packages in the `packages/` directory. This is MANDATORY and NON-NEGOTIABLE.

**Structural Requirements**:
- ALL application code MUST reside in packages within `packages/` directory
- Packages requiring both frontend and backend MUST be split into separate packages with `-frt` (frontend) and `-srv` (server/backend) suffixes
- Example: `packages/clusters-frt` and `packages/clusters-srv`
- Each package MUST contain a root `base/` directory to accommodate future alternative implementations
- Package naming MUST be lowercase with hyphens as separators

**Repository Root Restrictions**:
- The repository root MAY ONLY contain:
  - `Cargo.toml` (workspace configuration)
  - `rust-toolchain.toml` (toolchain specification)
  - `.gitignore`, `.github/` (repository configuration)
  - `.specify/` (specification workflow)
  - `README.md`, `README-RU.md`, `LICENSE` (documentation)
  - Root-level build/launch scripts (if absolutely necessary)
- The repository root MUST NOT contain:
  - Application source code (`src/` directories)
  - Feature implementations
  - Business logic
  - UI components
  - API endpoints

**Prohibited Patterns**:
- ❌ Implementing functionality directly in repository root
- ❌ Creating `src/` directory in repository root for application code
- ❌ Placing feature code outside `packages/` directory
- ❌ Monolithic applications without package separation

**Future-Proofing Requirement**:
- Packages MUST be designed for eventual extraction into separate repositories
- Package boundaries MUST enable independent versioning and deployment
- Inter-package dependencies MUST be explicit and manageable
- Each package MUST be self-contained with clear interfaces

**Rationale**: Modular package architecture is essential for:
1. **Scalability**: As the project grows, packages can be extracted to separate repositories
2. **Independent Development**: Teams can work on different packages simultaneously
3. **Clear Boundaries**: Package separation enforces architectural discipline
4. **Reusability**: Packages can be shared across different projects
5. **Deployment Flexibility**: Packages can be deployed independently

This structure mirrors the proven architecture of [Universo Platformo React](https://github.com/teknokomo/universo-platformo-react) and is fundamental to the platform's long-term maintainability and evolution.

### III. Bilingual Documentation (NON-NEGOTIABLE)

All documentation MUST be provided in both English and Russian with identical structure:
- English is the primary standard and MUST be created first
- Russian translation MUST have the same content, structure, and line count as English version
- README files MUST use the pattern: `README.md` (English) and `README-RU.md` (Russian)
- GitHub Issues and Pull Requests MUST include both English and Russian content using `<details><summary>In Russian</summary>` spoiler tags
- The spoiler tag MUST be exactly `<summary>In Russian</summary>` - no variations allowed

**Rationale**: The project serves both English and Russian-speaking communities. Maintaining identical structure ensures no information loss across languages and simplifies maintenance.

### IV. Database Flexibility

Database layer MUST be designed for abstraction:
- Primary focus on Supabase integration initially
- Architecture MUST support future addition of other database systems (PostgreSQL, MySQL, etc.)
- Database access MUST be encapsulated through trait-based abstractions
- No direct database-specific code in business logic

**Rationale**: While Supabase is the initial choice, the platform's evolution may require supporting multiple database backends. Designing for flexibility from the start prevents costly refactoring later.

### V. Issue-Driven Development

All development work MUST follow the issue-driven workflow:
- Create GitHub Issue BEFORE implementing any feature or fix
- Issues MUST follow the format specified in `.github/instructions/github-issues.md`
- Issues MUST be labeled according to `.github/instructions/github-labels.md`
- Pull Requests MUST reference their corresponding issue and follow `.github/instructions/github-pr.md`

**Rationale**: Issue-driven development provides traceability, enables planning, and creates a searchable history of decisions and implementations.

### VI. Specification-First Approach

Features MUST be specified before implementation:
- Create specification documents in `specs/` directory before coding
- Specifications MUST include user stories, requirements, and success criteria
- Use the specification template from `.specify/templates/spec-template.md`
- Implementation MUST follow the approved specification

**Rationale**: Specification-first approach ensures shared understanding, reduces rework, and provides clear acceptance criteria for validation.

### VII. Best Practices for Rust Fullstack

Implementation MUST follow Rust ecosystem best practices:
- Use idiomatic Rust patterns and conventions
- Leverage Yew for frontend (WebAssembly)
- Leverage Actix Web for backend
- Authentication using Rust-native solutions (compatible with Passport.js patterns from original)
- UI components following Material Design principles
- Do NOT blindly port bad patterns from the React implementation - analyze and improve
- Consult the reference implementation at https://github.com/teknokomo/universo-platformo-react for concept ONLY
- Note: The React implementation is partially complete and contains legacy Flowise code; avoid porting these incomplete or legacy sections

**Rationale**: Each technology stack has its own best practices. While the React implementation provides the conceptual foundation, this Rust implementation must use patterns appropriate to the Rust ecosystem for maintainability and performance.

### VIII. Repository Boundaries and Exclusions

The following MUST NOT be included in this repository:
- **docs/ directory**: Documentation will be hosted in a separate repository (docs.universo.pro). Do NOT create a `docs/` folder in this repository.
- **AI Agent Configuration Files**: Users will create their own AI agent rules and configuration files as needed. Do NOT pre-create `.github/agents/` or similar directories for AI instructions.

**Exceptions**:
- `.github/instructions/` for development guidelines IS allowed and required (github-issues.md, github-labels.md, github-pr.md, i18n-docs.md)
- `.specify/` directory for specification workflow IS allowed and required

**Rationale**: Keeping documentation and AI agent configurations separate allows them to evolve independently and prevents repository bloat. The React implementation's inclusion of these was identified as a pattern NOT to replicate.

## Technology Stack

**Frontend**: 
- Language: Rust + WebAssembly
- Framework: Yew
- UI Library: Material Design principles (Rust implementations)

**Backend**:
- Language: Rust
- Framework: Actix Web
- Database: Supabase (primary), with abstraction for future DBMS support
- Authentication: Rust-native auth solutions (Passport.js pattern compatibility)

**Monorepo Management**: Cargo workspaces

**Quality Tools**:
- Formatter: `rustfmt`
- Linter: `clippy`
- Testing: `cargo test`

## Development Workflow

1. **Planning Phase**:
   - Analyze requirements and create GitHub Issue
   - Write specification in `specs/[###-feature-name]/spec.md`
   - Get specification approval before implementation

2. **Implementation Phase**:
   - Create feature branch from main
   - Implement according to specification
   - Write tests alongside code
   - Update documentation (English first, then Russian)

3. **Quality Gates**:
   - All code MUST pass `cargo clippy` with no warnings
   - All code MUST be formatted with `rustfmt`
   - All tests MUST pass with `cargo test`
   - Documentation MUST be bilingual with identical structure

4. **Review & Merge**:
   - Create Pull Request following `.github/instructions/github-pr.md`
   - PR MUST reference the corresponding Issue
   - PR MUST include both English and Russian descriptions
   - PR MUST pass all CI checks before merge

## Governance

**Constitutional Authority**: This constitution supersedes all other development practices and guidelines. In case of conflict between this document and other guidance, this constitution takes precedence.

**Amendments**: 
- Constitution amendments require explicit documentation of the change rationale
- Version MUST be incremented following semantic versioning:
  - MAJOR: Backward incompatible principle removals or redefinitions
  - MINOR: New principles added or materially expanded guidance
  - PATCH: Clarifications, wording fixes, non-semantic refinements

**Compliance**:
- All Pull Requests MUST verify compliance with these principles
- Complexity or exceptions MUST be explicitly justified in PR descriptions
- Template files in `.specify/templates/` provide implementation guidance
- Agent instructions in `.github/agents/` MUST align with these principles

**Runtime Guidance**: Use `.github/instructions/*.md` files for specific runtime development guidance on GitHub workflows, documentation standards, and labeling conventions.

### IX. Non-Functional Requirements Priority

Non-functional requirements (performance, security, accessibility, maintainability) MUST be treated as first-class requirements alongside functional requirements:
- Performance targets MUST be quantified (build time, response time, bundle size)
- Security requirements MUST be explicit (authentication, encryption, vulnerability scanning)
- Accessibility MUST meet WCAG 2.1 Level AA standards
- Code quality MUST be enforced through automated tools (clippy, rustfmt, test coverage)

**Rationale**: Non-functional requirements are often neglected until late in development, leading to costly refactoring. Specifying them early ensures they are designed into the architecture from the start.

### X. Integration Contracts

Frontend and backend packages MUST communicate through explicitly defined API contracts:
- Contracts MUST be defined in shared type packages
- API endpoints MUST follow REST conventions
- Request/response schemas MUST use serde-compatible Rust types
- Breaking changes MUST follow semantic versioning

**Rationale**: Explicit contracts prevent integration issues and enable independent frontend/backend development. Shared types ensure type safety across the stack.

### XI. Risk Management

All specifications MUST identify and document risks with mitigation strategies:
- Probability and impact assessment for each risk
- Proactive mitigation strategies
- Contingency plans for high-impact risks
- Regular risk review and updates

**Rationale**: Proactive risk management prevents project delays and failures. Documented risks enable informed decision-making and resource allocation.

### XII. Shared Infrastructure Priority

All projects MUST establish shared infrastructure packages before implementing domain features:
- **universo-types**: Shared type definitions with serde traits for all entities, API contracts, and UPDL structures
- **universo-utils**: Common utilities, processors (UPDL processor), and helper functions
- **universo-api-client**: Centralized HTTP client for backend communication using reqwest
- **universo-i18n**: Centralized internationalization using fluent-rs or rust-i18n
- **universo-ui-components**: Shared Yew components following Material Design principles

**Dependency Rule**: Domain packages (clusters, metaverses, etc.) MUST depend on shared packages, never the reverse.

**Rationale**: Analysis of the React implementation revealed that creating shared packages early prevents massive code duplication (e.g., 7692 lines eliminated in chat components). Building shared infrastructure first ensures consistency across all domain packages and simplifies maintenance.

### XIII. Template System Architecture

The platform MUST support a template system for multi-platform export:
- Templates are specialized packages that convert UPDL (Universal Platform Description Language) to platform-specific implementations
- Each template package MUST implement a standard trait-based interface
- Templates generate deployable artifacts (HTML, JS, WASM) from UPDL structures
- Initial templates: AR.js (web-based AR) and PlayCanvas (3D engine)
- UPDL processor MUST be in shared utilities, not duplicated per template

**Template Interface Requirements**:
- Accept UPDL flow data as input
- Validate UPDL structure
- Transform to target platform code
- Generate deployable assets
- Return build artifacts or deployment URLs

**Rationale**: The React implementation's template system (`template-quiz`, `template-mmoomm`) enables the core value proposition of the platform: create once, deploy to multiple platforms. This is not an afterthought feature but a core architectural component.

### XIV. UPDL as Core Abstraction

UPDL (Universal Platform Description Language) MUST be treated as a first-class architectural component:
- UPDL defines a platform-agnostic representation of 3D/AR/VR scenes
- All visual programming nodes MUST produce UPDL output
- UPDL structures MUST be defined in `universo-types` with serde serialization
- UPDL processor in `universo-utils` converts flow graphs to UPDL
- Templates consume UPDL and produce platform-specific code

**UPDL Node Categories**:
- High-level nodes: Scene, Entity, Transform, Material, Interaction, Animation, Export
- Legacy nodes (for compatibility): Object, Camera, Light
- All nodes MUST have clear type definitions, validation rules, and documentation

**Integration Points**:
- Visual editor produces flow graphs
- Flow graphs are converted to UPDL
- UPDL is stored in database
- Templates read UPDL and generate code

**Rationale**: UPDL is the abstraction layer that enables platform independence. Without a well-defined UPDL system, the platform devolves into a collection of platform-specific editors with no interoperability.

### XV. Build Tooling Strategy

Build tooling MUST be planned comprehensively from the start:
- **WASM Frontend Builds**: Use `wasm-pack` or `trunk` for consistent WASM compilation
- **Asset Handling**: Define explicit strategy for SVG icons, images, and static files
- **Development Workflow**: Configure `cargo-watch` for hot-reload during development
- **Multi-Target Builds**: Support both native (backend) and WASM (frontend) from same codebase
- **Build Caching**: Leverage Cargo's incremental compilation and consider `sccache`
- **CI/CD Pipeline**: GitHub Actions with Rust toolchain, clippy, rustfmt, tests, and security audit

**Required Build Scripts**:
- `cargo build --workspace` - Build all packages
- `cargo test --workspace` - Run all tests
- `cargo clippy --workspace -- -D warnings` - Lint with zero warnings allowed
- `cargo fmt --all -- --check` - Verify formatting
- Per-package WASM builds for frontend packages

**Asset Handling**:
- Store assets in `assets/` directory within each package
- Use `include_bytes!()` or `include_str!()` for embedding
- For WASM, assets must be accessible via HTTP or embedded in WASM binary
- Document asset loading patterns for both native and WASM targets

**Rationale**: The React implementation evolved through multiple build systems (tsc+gulp → tsdown), causing technical debt and migration effort. Rust has the advantage of choosing the right tools upfront. Additionally, WASM builds have specific requirements that must be addressed from day one to avoid costly refactoring.

### XVI. Workspace Dependency Catalog

All shared dependencies MUST be defined in root `Cargo.toml` [workspace.dependencies] section:
- **Version Consistency**: All packages reference the same version of each dependency
- **Single Source of Truth**: Change version in one place, propagates to all packages
- **Conflict Prevention**: Eliminates dependency version conflicts ("dependency hell")
- **Simplified Upgrades**: Update dependency versions from central location

**Package Dependencies Pattern**:
```toml
# Root Cargo.toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
actix-web = "4.4"
yew = "0.21"
reqwest = { version = "0.11", features = ["json"] }
sea-orm = { version = "0.12", features = ["sqlx-postgres"] }

# Package Cargo.toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
actix-web = { workspace = true }
```

**Exceptions**:
- Development dependencies MAY have package-specific versions if needed for testing
- Feature flags MUST be consistent across workspace when dependency is shared

**Rationale**: The React implementation uses PNPM's catalog feature to prevent version conflicts and ensure consistency. Cargo workspaces provide equivalent functionality through [workspace.dependencies], enabling centralized version management that prevents drift and simplifies maintenance.

### XVII. Development Workflow Standardization

Development tooling and workflows MUST be standardized across the project:

**Required Development Tools**:
- `cargo-watch` for auto-rebuild during development
- `trunk` for WASM frontend development with hot reload
- `cargo-husky` for git pre-commit hooks
- `cargo-tarpaulin` for code coverage measurement

**Quality Gates**:
- Pre-commit hooks MUST run `cargo fmt --all -- --check`
- Pre-commit hooks MUST run `cargo clippy --workspace -- -D warnings`
- Pre-commit hooks SHOULD run `cargo test --workspace` (optional for speed)

**Hot Reload Workflow**:
- Backend development: `cargo watch -x 'run -p [package-name]'`
- Frontend development: `trunk serve` in package directory
- Shared packages: `cargo watch -x 'build -p universo-types -p universo-utils'`

**Database Development**:
- ORM: SeaORM as primary database abstraction layer
- Migrations: Use SeaORM CLI for migration generation and application
- Complex Queries: MAY use sqlx for type-safe SQL when needed
- Entity Pattern: All database models MUST derive SeaORM Entity trait

**Validation Strategy**:
- Input validation MUST use `validator` crate
- All API request types MUST implement Validate trait
- Validation errors MUST return structured error responses

**API Documentation**:
- API endpoints MUST be documented with `utoipa` macros
- OpenAPI specifications MUST be auto-generated from code
- Swagger UI MUST be available during development

**Testing Infrastructure**:
- Unit tests: Standard `cargo test`
- WASM tests: `wasm-bindgen-test` for frontend
- Integration tests: `actix-web::test` for backend
- Coverage: `cargo tarpaulin --workspace`

**Rationale**: The React implementation uses various tools (Vite, Vitest, ESLint, Prettier, Husky) to ensure development velocity and code quality. Standardizing equivalent Rust tooling from day one prevents ad-hoc tool adoption and ensures consistent developer experience.

**Version**: 1.5.0 | **Ratified**: 2025-11-15 | **Last Amended**: 2025-11-17
