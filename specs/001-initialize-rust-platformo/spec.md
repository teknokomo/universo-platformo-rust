# Feature Specification: Initialize Universo Platformo Rust Project

**Feature Branch**: `001-initialize-rust-platformo`  
**Created**: 2025-11-15  
**Status**: Draft  
**Input**: User description: "Initialize Universo Platformo Rust project with proper structure, documentation, and workflow based on the React version"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Repository Setup and Structure (Priority: P1)

As a developer joining the Universo Platformo Rust project, I need a properly structured monorepo with clear documentation so that I can understand the project architecture and start contributing immediately.

**Why this priority**: This is the foundation for all future development. Without proper repository structure and documentation, no other work can proceed effectively. It establishes the development workflow and project standards.

**Independent Test**: Can be fully tested by cloning the repository, reading the README files in both English and Russian, and verifying the directory structure matches the specified monorepo layout with `packages/` structure and proper package organization.

**Acceptance Scenarios**:

1. **Given** a developer clones the repository, **When** they read the root README.md, **Then** they understand what Universo Platformo is, its purpose, and how it differs from the React version
2. **Given** a developer explores the repository, **When** they examine the directory structure, **Then** they find a `packages/` directory ready for frontend and backend packages with `base/` subdirectories
3. **Given** a developer reads the documentation, **When** they look for Russian translations, **Then** they find README.ru.md with identical structure and content as README.md
4. **Given** a developer wants to contribute, **When** they check for guidelines, **Then** they find `.github/instructions/` files explaining issue creation, labels, PRs, and i18n standards

---

### User Story 2 - Issue Labels and Workflow Foundation (Priority: P2)

As a project maintainer, I need a complete set of GitHub issue labels that match the project's structure so that I can organize and track work across frontend, backend, and platform-specific areas.

**Why this priority**: Labels are essential for organizing work and maintaining visibility across the project. This must be done early to establish consistent tracking from the start.

**Independent Test**: Can be fully tested by navigating to the repository's Issues page and verifying all required labels exist with appropriate colors and descriptions for type (bug/feature/enhancement), areas (frontend/backend), and project context (platformo/rust/architecture).

**Acceptance Scenarios**:

1. **Given** a maintainer needs to categorize an issue, **When** they apply labels, **Then** they can select from type labels (feature, bug, enhancement, documentation, refactor, maintenance)
2. **Given** a maintainer works on a feature, **When** they label technical areas, **Then** they can use area labels (frontend, backend, build, testing, i18n, architecture)
3. **Given** a maintainer tracks project-specific work, **When** they apply project labels, **Then** they can use platformo, rust, and repository labels
4. **Given** a developer filters issues, **When** they search by label combinations, **Then** they can find all frontend platformo features or all backend bugs

---

### User Story 3 - Package Structure Template (Priority: P3)

As a developer creating a new feature package, I need a clear template for organizing frontend and backend code so that all packages follow consistent structure and naming conventions.

**Why this priority**: While not immediately blocking, this template establishes patterns that will be replicated across all future packages, making it important to define early but can be refined as first real packages are created.

**Independent Test**: Can be fully tested by examining the `packages/` directory structure and verifying the presence of example or template directories showing the `[name]-frt` and `[name]-srv` naming pattern with `base/` subdirectories.

**Acceptance Scenarios**:

1. **Given** a developer creates a new feature requiring frontend and backend, **When** they follow the structure template, **Then** they create separate packages named `[feature]-frt` and `[feature]-srv` within `packages/`
2. **Given** a package is created, **When** a developer examines its structure, **Then** they find a `base/` directory containing the core implementation
3. **Given** multiple implementations might exist in the future, **When** a developer adds code, **Then** they place the primary Rust/Yew/Actix implementation in the `base/` subdirectory
4. **Given** a developer reviews package examples, **When** they check the repository, **Then** they find documentation explaining the separation between frontend (-frt) and backend (-srv) packages

---

### User Story 4 - Development Environment Configuration (Priority: P4)

As a Rust developer setting up the project, I need configuration files for Cargo workspace and build tools so that I can compile and run the project locally with all dependencies properly managed.

**Why this priority**: Essential for actual development but can be added after basic structure is in place. Developers need this to run the code but it builds on the P1 foundation.

**Independent Test**: Can be fully tested by running `cargo build` in the repository root and verifying that the workspace compiles successfully with proper dependency resolution across packages.

**Acceptance Scenarios**:

1. **Given** a developer has Rust installed, **When** they run `cargo build` in the repository root, **Then** the workspace compiles all packages successfully
2. **Given** a developer adds a new package, **When** they register it in the workspace, **Then** cargo recognizes it and manages dependencies correctly
3. **Given** a developer needs to test, **When** they run `cargo test`, **Then** all package tests execute in the workspace context
4. **Given** a developer wants to follow the monorepo pattern from React version, **When** they examine the workspace configuration, **Then** they see it mirrors the PNPM workspace structure but adapted for Cargo

---

### Edge Cases

- What happens when a developer creates a package that needs only frontend or only backend (not both)? The structure should support single-package features without requiring dummy counterparts.
- How does the system handle bilingual documentation when auto-generating docs from code comments? Documentation standards should specify when to maintain parallel translations versus using a single language.
- What happens when a package needs more than two components (frontend, backend, shared utilities)? The structure should allow additional suffixes like `-shared` or `-common`.
- How are dependencies managed when a frontend package needs types from its backend counterpart? The workspace should support cross-package references while maintaining separation.
- What happens when Russian translations fall out of sync with English documentation? There should be a process for flagging and updating stale translations.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Repository MUST contain a comprehensive root README.md in English explaining Universo Platformo Rust, its relationship to the React version, and its technical stack (Rust, Yew, Actix Web)
- **FR-002**: Repository MUST contain a README.ru.md that is an exact translation of README.md with identical structure and line count
- **FR-003**: Repository MUST have a `packages/` directory for housing all feature packages
- **FR-003a**: ALL application functionality MUST be implemented as packages within `packages/` directory (NON-NEGOTIABLE)
- **FR-003b**: Repository root MUST NOT contain application source code, feature implementations, business logic, UI components, or API endpoints
- **FR-003c**: Repository root MAY ONLY contain workspace configuration (Cargo.toml, rust-toolchain.toml), repository configuration (.gitignore, .github/), specification workflow (.specify/), and documentation files
- **FR-003d**: Each package MUST be designed for potential extraction into a separate repository in the future
- **FR-003e**: Implementing functionality outside `packages/` directory is STRICTLY PROHIBITED
- **FR-004**: Package naming MUST follow the convention `[feature-name]-frt` for frontend and `[feature-name]-srv` for backend
- **FR-005**: Each package MUST contain a `base/` subdirectory for the primary Rust implementation
- **FR-006**: Repository MUST have a complete set of GitHub labels including type labels (feature, bug, enhancement, documentation, refactor, maintenance)
- **FR-007**: Repository MUST have area labels (frontend, backend, build, testing, i18n, architecture)
- **FR-008**: Repository MUST have project-specific labels (platformo, rust, repository, releases)
- **FR-009**: All documentation files MUST be created in both English and Russian versions with identical content structure
- **FR-010**: Repository MUST reference the Universo Platformo React project as the conceptual foundation
- **FR-011**: Repository MUST explain the monorepo structure and how it mirrors PNPM workspace approach but uses Cargo
- **FR-012**: Repository MUST document the decision to use Supabase as the initial database with provision for future DBMS support
- **FR-013**: Repository MUST document that Passport.js authentication approach will need a Rust equivalent
- **FR-014**: Repository MUST document that Material UI will be replaced with a Rust/Yew UI library equivalent
- **FR-015**: Repository MUST NOT include a `docs/` directory as documentation will be in a separate repository
- **FR-016**: Repository structure MUST be prepared for the Clusters feature as the first major functionality (Clusters/Domains/Resources entities)
- **FR-017**: Repository MUST document the pattern that many features will follow similar three-entity structures (like Metaverses/Sections/Entities)
- **FR-018**: Documentation MUST explain the relationship to the LangChain graph nodes system for future Spaces/Canvases functionality
- **FR-019**: Repository MUST include `.github/instructions/` files for issue creation, labeling, PRs, and i18n standards
- **FR-020**: Package structure MUST support features that only need frontend, only backend, or both components
- **FR-021**: Packages requiring shared utilities MUST use `-common` or `-shared` suffix (e.g., `auth-common`, `types-shared`)
- **FR-022**: Cross-package type sharing MUST be implemented through shared packages with explicit dependency declarations in Cargo.toml
- **FR-023**: Frontend-to-backend API contracts MUST be defined using typed interfaces in shared packages
- **FR-024**: Cargo workspace MUST compile all packages successfully with `cargo build` command from repository root
- **FR-025**: All packages MUST pass `cargo clippy` with zero warnings as a quality gate
- **FR-026**: All packages MUST be formatted with `rustfmt` using project-standard configuration
- **FR-027**: Rust toolchain version MUST be specified in `rust-toolchain.toml` file
- **FR-028**: WebAssembly target (wasm32-unknown-unknown) MUST be supported for all frontend packages
- **FR-029**: Database abstraction layer MUST use trait-based design to support multiple DBMS implementations
- **FR-030**: Authentication implementation MUST support session-based and token-based mechanisms compatible with Supabase auth
- **FR-031**: Repository MUST include `universo-types` shared crate with all type definitions using serde traits
- **FR-032**: Repository MUST include `universo-utils` shared crate with common utilities and UPDL processor
- **FR-033**: Repository MUST include `universo-api-client` shared crate for centralized HTTP client using reqwest
- **FR-034**: Repository MUST include `universo-i18n` shared crate for internationalization using fluent-rs or rust-i18n
- **FR-035**: Repository MUST include `universo-ui-components` shared crate with Yew components following Material Design
- **FR-036**: Domain packages (clusters, metaverses, etc.) MUST depend on shared infrastructure packages
- **FR-037**: Shared packages MUST NOT depend on domain packages (unidirectional dependency)
- **FR-038**: UPDL (Universal Platform Description Language) type definitions MUST be in `universo-types` crate
- **FR-039**: UPDL processor for converting flow graphs to UPDL structures MUST be in `universo-utils` crate
- **FR-040**: Template system interface MUST be defined as traits in `universo-utils` crate
- **FR-041**: Template packages MUST convert UPDL structures to platform-specific implementations
- **FR-042**: Template packages MUST generate deployable artifacts (HTML, JS, WASM)
- **FR-043**: WASM build pipeline using `wasm-pack` or `trunk` MUST be documented and configured
- **FR-044**: Asset handling strategy for SVG icons and images MUST be documented and implemented
- **FR-045**: Development hot-reload with `cargo-watch` MUST be configured and documented


### Non-Functional Requirements

#### Performance Requirements

- **NFR-001**: Cargo workspace full build MUST complete in under 5 minutes on standard development hardware (4 cores, 8GB RAM)
- **NFR-002**: Frontend WASM bundle size MUST not exceed 2MB (compressed) for initial page load
- **NFR-003**: Backend API response time MUST be under 200ms for 95th percentile of requests under normal load
- **NFR-004**: Frontend package incremental rebuild MUST complete in under 30 seconds during development
- **NFR-005**: Backend server MUST support at least 1000 concurrent connections with acceptable performance degradation

#### Security Requirements

- **NFR-006**: Authentication MUST use secure token storage (HttpOnly cookies or secure localStorage with encryption)
- **NFR-007**: Database connections MUST use encrypted connections (TLS/SSL)
- **NFR-008**: All user inputs MUST be validated and sanitized before processing
- **NFR-009**: Dependency vulnerability scanning MUST be performed using `cargo audit` in CI pipeline
- **NFR-010**: Password storage MUST use industry-standard hashing (Argon2id, bcrypt, or scrypt)
- **NFR-011**: API endpoints MUST implement rate limiting to prevent abuse
- **NFR-012**: All sensitive configuration MUST be stored in environment variables, not in code

#### Accessibility Requirements

- **NFR-013**: UI components MUST support keyboard navigation for all interactive elements
- **NFR-014**: UI MUST maintain WCAG 2.1 Level AA compliance for color contrast ratios
- **NFR-015**: Screen reader compatibility MUST be verified for all major user flows
- **NFR-016**: Focus indicators MUST be visible and meet accessibility standards
- **NFR-017**: Form validation errors MUST be announced to screen readers

#### Maintainability Requirements

- **NFR-018**: Code coverage MUST be at least 70% for new code, measured by `cargo tarpaulin`
- **NFR-019**: All public APIs MUST have documentation comments following Rust conventions
- **NFR-020**: Complexity metrics MUST be monitored with maximum cyclomatic complexity of 10 per function
- **NFR-021**: Documentation in English and Russian MUST be synchronized within 48 hours of changes
- **NFR-022**: Git commit messages MUST follow conventional commits specification

### Integration Specifications

#### Package Communication Contracts

- **INT-001**: Frontend packages MUST communicate with backend packages exclusively through REST APIs
- **INT-002**: API endpoints MUST follow RESTful conventions: GET (read), POST (create), PUT/PATCH (update), DELETE (delete)
- **INT-003**: API request/response payloads MUST be defined in shared type packages using serde-compatible structs
- **INT-004**: API versioning MUST be implemented via URL path (e.g., `/api/v1/clusters`)
- **INT-005**: WebSocket connections for real-time features MUST use protocol defined in shared packages

#### Cross-Package Dependencies

- **INT-006**: Frontend packages MAY depend on shared type packages but MUST NOT depend on backend packages directly
- **INT-007**: Backend packages MAY depend on shared type packages and other backend utility packages
- **INT-008**: Shared packages MUST NOT depend on frontend or backend packages (unidirectional dependency)
- **INT-009**: Package version compatibility MUST be enforced through Cargo.toml workspace dependencies

#### Database Integration

- **INT-010**: Database access MUST be encapsulated through repository pattern implementations
- **INT-011**: Database schema migrations MUST be managed through migration tools (e.g., sqlx migrations)
- **INT-012**: All database queries MUST use parameterized queries to prevent SQL injection
- **INT-013**: Database connection pooling MUST be implemented for performance and resource management

### Shared Infrastructure Requirements

#### Shared Package Architecture

- **SHR-001**: `universo-types` crate MUST define all shared type definitions with serde Serialize and Deserialize traits
- **SHR-002**: `universo-types` MUST include UPDL node structures, API contracts, database entities, and platform types
- **SHR-003**: `universo-utils` crate MUST provide UPDL processor for converting flow graphs to UPDL
- **SHR-004**: `universo-utils` MUST include template interface traits and common utility functions
- **SHR-005**: `universo-api-client` crate MUST use reqwest for HTTP client with async/await support
- **SHR-006**: `universo-api-client` MUST provide type-safe API methods for all backend services
- **SHR-007**: `universo-i18n` crate MUST support English and Russian languages with fluent-rs or rust-i18n
- **SHR-008**: `universo-i18n` MUST provide macros for compile-time translation key validation
- **SHR-009**: `universo-ui-components` crate MUST provide Yew components following Material Design principles
- **SHR-010**: All shared packages MUST have comprehensive documentation and usage examples

#### Dependency Management

- **SHR-011**: Shared packages MUST be created in Phase 1 before any domain features
- **SHR-012**: Domain packages MUST declare dependencies on shared packages in Cargo.toml
- **SHR-013**: Shared packages MUST NOT depend on domain packages (strict unidirectional)
- **SHR-014**: Version compatibility MUST be enforced through workspace dependencies
- **SHR-015**: Breaking changes in shared packages MUST use semantic versioning

### UPDL System Requirements

#### UPDL Type Definitions

- **UPDL-001**: UPDL node types MUST be defined in `universo-types` with serde traits
- **UPDL-002**: High-level UPDL nodes MUST include: Scene, Entity, Transform, Material, Interaction, Animation, Export
- **UPDL-003**: Legacy UPDL nodes for backward compatibility MUST include: Object, Camera, Light
- **UPDL-004**: Each UPDL node MUST have unique identifier, type discriminator, and properties struct
- **UPDL-005**: UPDL structures MUST be platform-agnostic and not tied to specific rendering engines

#### UPDL Processing

- **UPDL-006**: UPDL processor in `universo-utils` MUST convert flow graph JSON to typed UPDL structures
- **UPDL-007**: UPDL processor MUST validate node connections and data flow
- **UPDL-008**: UPDL processor MUST support both single-scene and multi-scene flows
- **UPDL-009**: UPDL processor MUST handle node property resolution and inheritance
- **UPDL-010**: UPDL validation errors MUST provide clear, actionable error messages

#### UPDL Integration

- **UPDL-011**: Visual editor MUST produce flow graphs that can be converted to UPDL
- **UPDL-012**: UPDL structures MUST be stored in database for persistence
- **UPDL-013**: Template packages MUST consume UPDL as input format
- **UPDL-014**: UPDL specification MUST be documented with examples for each node type
- **UPDL-015**: UPDL schema MUST support versioning for future extensions

### Template System Requirements

#### Template Package Interface

- **TMPL-001**: All template packages MUST implement a standard trait-based interface
- **TMPL-002**: Template trait MUST define methods: validate_updl, process, generate_artifacts
- **TMPL-003**: Template packages MUST accept UPDL structures as input
- **TMPL-004**: Template packages MUST generate deployable artifacts (HTML, JS, WASM)
- **TMPL-005**: Template packages MUST return build status and artifact URLs or errors

#### Template Implementations

- **TMPL-006**: Initial implementation MUST include AR.js template for web-based augmented reality
- **TMPL-007**: Initial implementation MUST include PlayCanvas template for 3D engine experiences
- **TMPL-008**: Each template MUST handle platform-specific node type conversions
- **TMPL-009**: Templates MUST validate UPDL compatibility before processing
- **TMPL-010**: Templates MUST provide clear error messages for unsupported UPDL features

#### Template Processing Pipeline

- **TMPL-011**: Template processing MUST be triggered by export action in UI
- **TMPL-012**: Backend MUST serve raw flow data; frontend processes UPDL and invokes templates
- **TMPL-013**: Template processing MUST be asynchronous with progress reporting
- **TMPL-014**: Generated artifacts MUST be stored with unique identifiers
- **TMPL-015**: Templates MUST support incremental updates without full regeneration

### Clusters Feature Detailed Requirements

#### Entity Schema Requirements

- **CLU-001**: **Cluster** entity MUST have fields: id (UUID), name (String), description (String), owner_id (UUID), created_at (DateTime), updated_at (DateTime)
- **CLU-002**: **Domain** entity MUST have fields: id (UUID), cluster_id (UUID foreign key), name (String), description (String), status (enum), created_at (DateTime), updated_at (DateTime)
- **CLU-003**: **Resource** entity MUST have fields: id (UUID), domain_id (UUID foreign key), name (String), type (enum), config (JSON), created_at (DateTime), updated_at (DateTime)
- **CLU-004**: All entities MUST support soft deletion with deleted_at (Optional<DateTime>) field
- **CLU-005**: Entity relationships: Cluster (1) -> Domains (many), Domain (1) -> Resources (many)

#### Clusters Feature Operations

- **CLU-006**: CRUD operations MUST be implemented for all three entities (Create, Read, Update, Delete)
- **CLU-007**: List operations MUST support pagination with page size limit of 100 items
- **CLU-008**: Search functionality MUST support filtering by name and description fields
- **CLU-009**: Cluster deletion MUST cascade to all child Domains and Resources (soft delete)
- **CLU-010**: Resource types MUST be extensible through enum with "Other" variant for future additions

#### Three-Entity Pattern Template

- **CLU-011**: Clusters feature implementation MUST serve as template for similar features (Metaverses, Uniks)
- **CLU-012**: Pattern documentation MUST include: entity schema guidelines, API endpoint structure, UI component structure
- **CLU-013**: Code structure MUST use generic/trait-based abstractions where appropriate to enable reuse
- **CLU-014**: File organization pattern MUST be documented: models/, services/, controllers/, routes/ for backend; components/, hooks/, services/ for frontend

### Testing Strategy

#### Test Coverage Requirements

- **TST-001**: All public functions MUST have unit tests
- **TST-002**: All API endpoints MUST have integration tests
- **TST-003**: Critical user flows MUST have end-to-end tests
- **TST-004**: Database repositories MUST have integration tests with test database
- **TST-005**: Frontend components MUST have unit tests for logic and integration tests for rendering

#### Test Organization

- **TST-006**: Unit tests MUST be colocated with source code in module `tests` submodules
- **TST-007**: Integration tests MUST be in `tests/` directory at package root
- **TST-008**: Test fixtures and utilities MUST be in dedicated `test-utils` shared package
- **TST-009**: Test data MUST be generated programmatically or loaded from fixture files, not hardcoded

#### Quality Gates

- **TST-010**: All tests MUST pass before PR merge
- **TST-011**: No `cargo clippy` warnings allowed in PR
- **TST-012**: Code MUST be formatted with `rustfmt` before commit
- **TST-013**: Test coverage MUST not decrease from current baseline

### CI/CD Requirements

- **CI-001**: GitHub Actions workflow MUST run on all pushes to feature branches and PRs
- **CI-002**: CI pipeline MUST include: cargo build, cargo test, cargo clippy, rustfmt check, cargo audit
- **CI-003**: WASM build MUST be verified for frontend packages in CI
- **CI-004**: CI pipeline MUST fail if any step fails
- **CI-005**: Build artifacts (WASM bundles) MUST be cached between CI runs
- **CI-006**: Security vulnerability scan MUST run weekly and on dependency updates

### Phased Implementation Approach

#### Phase 1: Foundation Setup (Current)

**Deliverables:**
- Repository structure with packages/ directory
- Cargo workspace configuration
- Documentation (README.md, README-RU.md)
- GitHub labels and issue templates
- Instruction files in .github/instructions/
- Package structure template
- **Shared Infrastructure Packages:**
  - universo-types with base type definitions
  - universo-utils with common utilities
  - universo-api-client foundation
  - universo-i18n with EN/RU support
  - universo-ui-components base components
- Build tooling configuration (WASM, cargo-watch, CI/CD)

**Completion Criteria:**
- All FR-001 through FR-045 satisfied
- All SHR-001 through SHR-015 satisfied
- Documentation 100% bilingual
- Cargo workspace compiles successfully
- Quality gates configured
- Shared infrastructure packages functional
- WASM build pipeline documented and working

**Estimated Effort:** 3-4 weeks

#### Phase 2: UPDL System and Clusters Feature

**Dependencies:** Phase 1 complete, shared infrastructure operational

**Deliverables:**
- **UPDL System:**
  - UPDL type definitions in universo-types (UPDL-001 through UPDL-005)
  - UPDL processor in universo-utils (UPDL-006 through UPDL-010)
  - UPDL documentation and examples
- **Clusters Feature:**
  - clusters-frt package (Yew frontend)
  - clusters-srv package (Actix backend)
  - Database schema and migrations
  - API endpoints for all CRUD operations
  - UI components for cluster management
- Complete test coverage
- Documentation for three-entity pattern

**Completion Criteria:**
- All CLU-001 through CLU-014 satisfied
- All UPDL-001 through UPDL-015 satisfied
- UPDL processor converts flow graphs correctly
- All tests passing
- Demo deployment available
- Pattern documentation complete for reuse

**Estimated Effort:** 5-7 weeks

#### Phase 3: Template System and Additional Features

**Dependencies:** Phase 2 complete, UPDL system operational

**Deliverables:**
- **Template System:**
  - Template trait interface in universo-utils
  - AR.js template package (template-arjs)
  - PlayCanvas template package (template-playcanvas)
  - Template processing pipeline
- **Additional Features:**
  - Metaverses feature (following Clusters pattern)
  - Uniks (workspaces) feature
  - Spaces/Canvases feature
- Publication system for sharing exported experiences

**Completion Criteria:**
- All TMPL-001 through TMPL-015 satisfied
- Templates generate working AR.js and PlayCanvas experiences
- Additional features follow established patterns
- End-to-end export workflow functional
- Published experiences accessible via URLs

**Estimated Effort:** 8-10 weeks

#### Phase 4: Advanced Features (Future)

**Scope:** 
- Space Builder (AI-assisted flow generation)
- Multiplayer infrastructure
- Advanced authentication (OAuth2)
- Analytics and monitoring
- Additional template platforms

### Risk Management

#### Identified Risks

**RISK-001: React Repository Unavailability**
- **Probability:** Low
- **Impact:** Medium
- **Mitigation:** Document key concepts and patterns locally; maintain local clone of React repo
- **Contingency:** Reference archived documentation; proceed with Rust best practices independently

**RISK-002: UI Library Selection Uncertainty**
- **Probability:** Medium
- **Impact:** High
- **Mitigation:** Evaluate top 3 Yew UI libraries early; create proof-of-concept; document decision criteria
- **Contingency:** Build custom components if no suitable library found; budget additional 2-3 weeks

**RISK-003: Authentication Complexity**
- **Probability:** Medium
- **Impact:** High
- **Mitigation:** Use well-established Rust auth crates; follow Supabase auth patterns; security review
- **Contingency:** Implement basic auth first, enhance later; engage security consultant if needed

**RISK-004: WASM Performance Issues**
- **Probability:** Low
- **Impact:** High
- **Mitigation:** Benchmark early; follow WASM optimization best practices; monitor bundle size
- **Contingency:** Identify and optimize hot paths; consider server-side rendering for critical paths

**RISK-005: Bilingual Documentation Maintenance Burden**
- **Probability:** High
- **Impact:** Medium
- **Mitigation:** Use translation tools for initial drafts; establish review process; flag stale translations
- **Contingency:** Prioritize English documentation; mark Russian as "outdated" until updated

**RISK-006: Breaking Changes in Rust Ecosystem**
- **Probability:** Low
- **Impact:** Medium
- **Mitigation:** Pin dependency versions; monitor changelogs; test upgrades in branches
- **Contingency:** Fork and maintain necessary dependencies if breaking changes are critical

**RISK-007: UPDL System Complexity**
- **Probability:** Medium
- **Impact:** High
- **Mitigation:** Start with minimal UPDL node set; comprehensive type system; thorough testing
- **Contingency:** Simplify UPDL structure if complexity grows unmanageable; defer advanced features

**RISK-008: Template System Interoperability**
- **Probability:** Medium
- **Impact:** High
- **Mitigation:** Clear trait interface; extensive validation; platform-specific testing
- **Contingency:** Focus on one template platform initially (AR.js); add others incrementally

**RISK-009: Visual Editor Availability for Rust**
- **Probability:** High
- **Impact:** Critical
- **Mitigation:** Research Rust flow editor libraries early; consider hybrid JS+WASM approach
- **Contingency:** Use existing React Flow in hybrid mode; build custom editor in later phase

**RISK-010: Shared Package Dependency Hell**
- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:** Strict versioning discipline; workspace dependencies; careful API design
- **Contingency:** Revert to local duplication if shared packages cause more problems than they solve

### Technology Migration Specifications

#### Authentication Migration (Passport.js → Rust)

**Required Features:**
- **AUTH-001**: Local username/password authentication
- **AUTH-002**: OAuth2 integration (at minimum: GitHub, Google)
- **AUTH-003**: Session management with secure cookies
- **AUTH-004**: Password reset functionality via email
- **AUTH-005**: User registration with email verification
- **AUTH-006**: JWT token generation and validation for API access
- **AUTH-007**: Role-based access control (RBAC) support
- **AUTH-008**: Integration with Supabase authentication

**Security Properties:**
- Industry-standard password hashing (Argon2id preferred)
- CSRF protection for state-changing operations
- XSS prevention through proper input sanitization
- Rate limiting on authentication endpoints

**Recommended Crates:**
- actix-identity for session management
- argon2 for password hashing
- jsonwebtoken for JWT handling
- oauth2 for OAuth2 flows

#### UI Library Migration (Material UI → Rust/Yew)

**Required Features:**
- **UI-001**: Button component with variants (text, contained, outlined)
- **UI-002**: Form inputs (text, number, select, checkbox, radio)
- **UI-003**: Layout components (Grid, Container, Stack)
- **UI-004**: Navigation components (AppBar, Drawer, Tabs)
- **UI-005**: Feedback components (Alert, Snackbar, Dialog)
- **UI-006**: Data display (Table, List, Card)
- **UI-007**: Theming system with Material Design color palette
- **UI-008**: Responsive design support with breakpoints

**Evaluation Criteria:**
- Component completeness (cover 80% of Material UI usage)
- Documentation quality and examples
- Active maintenance and community support
- Performance characteristics (WASM bundle size impact)
- Accessibility compliance (WCAG 2.1)

**Candidate Libraries:**
- yew-material (if available and maintained)
- Custom components following Material Design guidelines
- Hybrid approach: use available components, build missing ones

### Assumptions

- Developers have basic familiarity with Rust, Cargo workspaces, and monorepo concepts (learning resources will be linked in documentation)
- The Universo Platformo React repository (https://github.com/teknokomo/universo-platformo-react) remains accessible as reference; if not, proceed with documented patterns
- Supabase will be the sole database for initial implementation phases (Phase 1 and 2)
- The project will prioritize Rust best practices over direct JavaScript/TypeScript pattern translation
- UI component library selection for Yew will be evaluated and finalized before Phase 2 implementation begins
- Authentication mechanism equivalent to Passport.js will use established Rust crates (actix-identity, jsonwebtoken, oauth2)
- Legacy Flowise code mentioned in React version will not be ported to Rust version
- Bilingual documentation is mandatory and will be maintained consistently across all files
- The workspace will use Cargo's native workspace features rather than requiring external tools like PNPM
- Standard development hardware is defined as: 4 CPU cores, 8GB RAM, SSD storage
- Rust stable toolchain will be used (minimum version 1.70.0 or later)
- Node.js may be required for WASM tooling (wasm-pack, trunk) but not for core application logic
- CI/CD will use GitHub Actions (no external CI services required initially)
- Supabase account setup and configuration is completed before Phase 2 begins
- Russian translations will be performed by team members fluent in both languages; machine translation for initial drafts only
- "Best Rust practices" refers to: Rust API Guidelines, idiomatic patterns from Rust Book, cargo clippy recommendations
- When React patterns conflict with Rust idioms, Rust idioms take precedence (document decision in PR)
- Monitoring React repository for new features will be performed monthly during active development

### Key Entities

- **Repository**: The root monorepo containing all packages, configuration, and shared documentation
- **Package**: A self-contained module in `packages/` directory, may be frontend (-frt), backend (-srv), or shared (-common/-shared)
- **Base Implementation**: The primary Rust/Yew/Actix code within a package's `base/` subdirectory, allowing for future alternative implementations
- **Feature**: A complete functionality unit that may span multiple packages (e.g., Clusters feature includes clusters-frt, clusters-srv, clusters-common)
- **Label**: GitHub issue/PR categorization tags for type, area, and project context
- **Documentation**: Bilingual markdown files with English originals and Russian translations maintaining identical structure
- **Cluster**: Top-level organizational entity containing Domains (first entity in three-entity pattern)
- **Domain**: Mid-level entity belonging to a Cluster, containing Resources (second entity in three-entity pattern)
- **Resource**: Leaf-level entity belonging to a Domain, represents actual resources (third entity in three-entity pattern)
- **Three-Entity Pattern**: Reusable architectural pattern (Parent/Child/Leaf) used across features (Clusters, Metaverses, Uniks)
- **API Contract**: Typed interface definition in shared packages specifying request/response structures
- **Quality Gate**: Automated check (tests, linting, formatting) that must pass before code merge
- **Phase**: Defined stage of implementation with specific deliverables and completion criteria
- **Shared Infrastructure Package**: Foundation crate (universo-types, universo-utils, universo-api-client, universo-i18n, universo-ui-components) that provides common functionality to domain packages
- **UPDL (Universal Platform Description Language)**: Platform-agnostic representation of 3D/AR/VR scenes that serves as the abstraction layer for multi-platform export
- **UPDL Node**: A component in the visual programming interface that produces UPDL output (Scene, Entity, Transform, Material, Interaction, Animation, Export)
- **UPDL Processor**: Utility in universo-utils that converts visual flow graphs to UPDL structures
- **Template Package**: Specialized package that converts UPDL to platform-specific implementation (e.g., AR.js, PlayCanvas)
- **Template Interface**: Trait-based contract that all template packages must implement for UPDL consumption and artifact generation
- **Build Artifact**: Deployable output from template processing (HTML, JS, WASM binary) that can be hosted and executed


## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer can clone the repository and understand the project's purpose, structure, and relationship to the React version within 10 minutes of reading the README files
- **SC-002**: 100% of documentation files have both English and Russian versions with identical structure and content meaning
- **SC-003**: All required GitHub labels (minimum of 15 labels covering types, areas, and project context) are created and properly described
- **SC-004**: The `packages/` directory structure is ready to receive new feature packages following the established naming convention
- **SC-005**: A new developer can identify where to place frontend code, backend code, and shared utilities based on the documented package structure
- **SC-006**: The repository successfully references and acknowledges the Universo Platformo React project as its conceptual foundation in all relevant documentation
- **SC-007**: 100% compliance with bilingual documentation standards as defined in `.github/instructions/i18n-docs.md`
- **SC-008**: The repository structure accommodates the planned three-entity pattern (Clusters/Domains/Resources) that will be replicated across multiple features
- **SC-009**: Contributors can successfully create issues and PRs following the guidelines in `.github/instructions/` without confusion
- **SC-010**: The repository foundation enables development to proceed immediately on the first feature (Clusters functionality) without additional structural work
- **SC-011**: Cargo workspace builds successfully with `cargo build` completing in under 5 minutes on standard hardware
- **SC-012**: All quality gates (clippy, rustfmt, tests) pass on initial repository setup
- **SC-013**: CI/CD pipeline is configured and runs successfully on all commits
- **SC-014**: Frontend WASM build process is documented and functional for Yew packages
- **SC-015**: Database abstraction layer design is documented with trait definitions
- **SC-016**: Authentication strategy is documented with selected crates and security requirements
- **SC-017**: UI component library evaluation criteria are documented with decision framework
- **SC-018**: Risk management document identifies top 6 risks with mitigation strategies
- **SC-019**: Three-entity pattern is fully specified with schema, relationships, and replication guide
- **SC-020**: Test coverage requirements and strategy are documented for all package types
- **SC-021**: API contract specification format is defined for frontend-backend communication
- **SC-022**: Package versioning and compatibility strategy is documented
- **SC-023**: Phase 1 completion criteria are 100% satisfied before Phase 2 begins
- **SC-024**: Traceability exists between all functional requirements, user stories, and success criteria
- **SC-025**: Non-functional requirements are quantified for performance, security, and accessibility

## Traceability Matrix

This section maps requirements to user stories and success criteria to ensure complete coverage.

### FR to User Story Mapping

**Repository Structure (FR-001 to FR-005, FR-015, FR-019)**
- → User Story 1: Repository Setup and Structure (P1)
- Validates: SC-001, SC-004, SC-005, SC-006

**GitHub Workflow (FR-006 to FR-008, FR-019)**
- → User Story 2: Issue Labels and Workflow Foundation (P2)
- Validates: SC-003, SC-009

**Package Architecture (FR-004, FR-005, FR-020 to FR-023)**
- → User Story 3: Package Structure Template (P3)
- Validates: SC-004, SC-005, SC-021

**Development Environment (FR-024 to FR-028)**
- → User Story 4: Development Environment Configuration (P4)
- Validates: SC-011, SC-012, SC-014

**Documentation Standards (FR-001, FR-002, FR-009, FR-010)**
- → User Story 1: Repository Setup and Structure (P1)
- Validates: SC-002, SC-007

**Database & Auth Architecture (FR-012, FR-013, FR-029, FR-030)**
- → Implicit in Phase 2 preparation
- Validates: SC-015, SC-016

**Clusters Feature (CLU-001 to CLU-014)**
- → Phase 2: Clusters Feature Implementation
- Validates: SC-008, SC-019

**Non-Functional Requirements (NFR-001 to NFR-022)**
- → Cross-cutting across all phases
- Validates: SC-011, SC-012, SC-013, SC-025

**Integration Specs (INT-001 to INT-013)**
- → User Story 3 and Phase 2
- Validates: SC-021, SC-022

**Testing Strategy (TST-001 to TST-013)**
- → All phases, quality gates
- Validates: SC-012, SC-020

**CI/CD Requirements (CI-001 to CI-006)**
- → Phase 1 foundation
- Validates: SC-013

### User Story to Success Criteria Mapping

**User Story 1 (P1): Repository Setup and Structure**
- Primary: SC-001, SC-002, SC-004, SC-005, SC-006, SC-007, SC-010
- Supporting: SC-011, SC-012

**User Story 2 (P2): Issue Labels and Workflow Foundation**
- Primary: SC-003, SC-009
- Supporting: SC-024

**User Story 3 (P3): Package Structure Template**
- Primary: SC-004, SC-005, SC-021, SC-022
- Supporting: SC-008, SC-019

**User Story 4 (P4): Development Environment Configuration**
- Primary: SC-011, SC-012, SC-014
- Supporting: SC-013

### Requirements Coverage Summary

- **Total Functional Requirements**: 30 (FR-001 to FR-030)
- **Total Non-Functional Requirements**: 22 (NFR-001 to NFR-022)
- **Total Integration Requirements**: 13 (INT-001 to INT-013)
- **Total Clusters Requirements**: 14 (CLU-001 to CLU-014)
- **Total Testing Requirements**: 13 (TST-001 to TST-013)
- **Total CI/CD Requirements**: 6 (CI-001 to CI-006)
- **Total Authentication Requirements**: 8 (AUTH-001 to AUTH-008)
- **Total UI Requirements**: 8 (UI-001 to UI-008)
- **Total Risk Items**: 6 (RISK-001 to RISK-006)
- **Total Success Criteria**: 25 (SC-001 to SC-025)
- **Total User Stories**: 4 (P1 to P4)

## Documentation Standards

### Inline Code Documentation

**Rust Doc Comments:**
- All public modules MUST have module-level documentation (`//!`)
- All public functions MUST have doc comments (`///`) describing purpose, parameters, returns, and examples
- All public structs and enums MUST have doc comments explaining their purpose
- Complex algorithms MUST have explanatory comments in code
- Use `# Examples` section in doc comments for non-trivial functions

**Example:**
```rust
/// Represents a Cluster in the Universo Platformo system.
///
/// A Cluster is the top-level organizational entity that contains
/// multiple Domains. Each Cluster has an owner and tracks creation
/// and modification timestamps.
///
/// # Examples
///
/// ```
/// let cluster = Cluster {
///     id: Uuid::new_v4(),
///     name: "Main Cluster".to_string(),
///     description: "Primary organizational cluster".to_string(),
///     owner_id: user.id,
///     created_at: Utc::now(),
///     updated_at: Utc::now(),
/// };
/// ```
pub struct Cluster {
    pub id: Uuid,
    pub name: String,
    // ... other fields
}
```

### API Documentation

**REST API Documentation:**
- Endpoint path, method, description
- Request body schema (if applicable)
- Response body schema
- Status codes and their meanings
- Authentication requirements
- Example requests and responses

**Format:** OpenAPI 3.0 specification (future enhancement)
**Location:** `docs/api/` (when separate docs repo is created)
**Current:** Inline in README.md for Phase 1

### Translation Synchronization Process

**Process for Keeping Documentation in Sync:**

1. **English First**: Always write/update English documentation first
2. **Flag for Translation**: Add comment `<!-- NEEDS RUSSIAN TRANSLATION -->` at top of English file after changes
3. **Translation Window**: Russian translation MUST be completed within 48 hours of English update
4. **Review**: Native Russian speaker reviews translation for accuracy
5. **Sync Check**: Automated script checks line count matches (warning if >5% difference)
6. **Remove Flag**: Remove translation flag comment after Russian version is updated

**Stale Translation Handling:**
- If Russian translation not updated within 48 hours, add warning banner:
  ```markdown
  > ⚠️ **Внимание**: Этот документ может быть устаревшим. Актуальная информация доступна в [английской версии](README.md).
  > 
  > **Warning**: This document may be outdated. Current information is available in the [English version](README.md).
  ```

**Tools:**
- Translation sync checker script: `.github/scripts/check-i18n-sync.sh` (to be created)
- CI check warns if translations are out of sync
- Monthly review of all documentation for sync status

### README Structure Standard

**Root README.md Structure:**
1. Project Title and Badges
2. Basic Information / Overview
3. Inspiration / Mission Statement
4. Contact Information
5. Relationship to Other Projects
6. Current Status
7. Tech Stack
8. Getting Started
9. Development Guide
10. Contributing
11. License

**Package README.md Structure:**
1. Package Name and Purpose
2. Features
3. Installation / Usage
4. API Reference (brief, link to full docs)
5. Examples
6. Development
7. Testing
8. Contributing

### Version Control for Specifications

**Specification Versioning:**
- Initial specification: Version 1.0.0
- Clarifications/corrections: Increment patch (1.0.1)
- New requirements added: Increment minor (1.1.0)
- Major scope changes: Increment major (2.0.0)

**Change Log:**
- Maintain CHANGELOG.md in each spec directory
- Document all changes with date, version, and description
- Reference related PRs and issues

## Monitoring and Maintenance

### React Repository Monitoring

**Schedule:** Monthly review during active development (first 6 months)

**Process:**
1. Review React repo commits from last month
2. Identify new features or significant changes
3. Assess applicability to Rust version
4. Create GitHub issues for relevant features
5. Prioritize based on project roadmap
6. Document decision (port or skip) with rationale

**Responsibility:** Project maintainer or designated team member

**Tools:**
- GitHub watch/notification
- Monthly recurring calendar reminder
- Tracking spreadsheet/issue for feature parity

### Constitution Updates

**When to Update Constitution:**
- New core principle identified
- Existing principle needs clarification
- Conflict between principles discovered
- Technology stack fundamental change

**Update Process:**
1. Create issue proposing constitution change
2. Document rationale and impact analysis
3. Review by project maintainers
4. Update constitution with version increment
5. Announce change to all contributors
6. Update related documentation and instructions

### Quality Metrics Dashboard

**Metrics to Track (Future Enhancement):**
- Code coverage percentage
- Build time trend
- WASM bundle size trend
- API response time (95th percentile)
- Documentation sync status
- Issue resolution time
- PR merge time
- Dependency vulnerability count

**Reporting:** Monthly summary in project discussions/wiki

## Glossary

**Actix Web**: High-performance, actor-based web framework for Rust

**Cargo**: Rust's package manager and build system

**Cargo Workspace**: Monorepo structure in Rust for managing multiple related packages

**Clusters/Domains/Resources**: Three-entity organizational pattern - top/middle/leaf level entities

**CRUD**: Create, Read, Update, Delete - basic data operations

**Material Design**: Design system created by Google for UI components

**Monorepo**: Repository structure containing multiple related packages/projects

**NFR**: Non-Functional Requirement - quality attributes like performance, security

**OAuth2**: Industry-standard protocol for authorization

**Passport.js**: Popular authentication middleware for Node.js (reference from React version)

**RBAC**: Role-Based Access Control - authorization model based on user roles

**reqwest**: Rust HTTP client library for making API requests

**REST API**: Representational State Transfer - architectural style for web services

**serde**: Rust serialization/deserialization framework

**Shared Infrastructure Package**: Foundation crate (universo-types, universo-utils, etc.) providing common functionality

**Supabase**: Open-source Firebase alternative providing database and auth services

**Template Package**: Specialized package converting UPDL to platform-specific implementation (AR.js, PlayCanvas)

**Three-Entity Pattern**: Reusable architecture pattern with Parent/Child/Leaf entities

**trunk**: Build and pipeline tool for Rust WASM applications

**UPDL**: Universal Platform Description Language - platform-agnostic 3D/AR/VR scene representation

**UPDL Node**: Visual programming component producing UPDL output (Scene, Entity, Transform, etc.)

**UPDL Processor**: Utility converting flow graphs to UPDL structures

**WASM**: WebAssembly - binary instruction format for web browsers

**wasm-pack**: Tool for building and packaging Rust WASM projects

**WCAG**: Web Content Accessibility Guidelines - accessibility standards

**Yew**: Rust framework for building frontend web applications using WebAssembly

**Yewdux**: State management library for Yew applications

## References

**Project References:**
- [Universo Platformo React](https://github.com/teknokomo/universo-platformo-react) - Original implementation
- [Universo.pro Website](https://universo.pro) - Project website

**Technical Documentation:**
- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust programming guide
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Best practices for Rust APIs
- [Yew Documentation](https://yew.rs/) - Yew framework documentation
- [Actix Web Documentation](https://actix.rs/) - Actix Web framework documentation
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo package manager guide

**Standards:**
- [Conventional Commits](https://www.conventionalcommits.org/) - Commit message specification
- [Semantic Versioning](https://semver.org/) - Version numbering standard
- [WCAG 2.1](https://www.w3.org/WAI/WCAG21/quickref/) - Web accessibility guidelines
- [OpenAPI 3.0](https://swagger.io/specification/) - API specification standard

**Security:**
- [OWASP Top 10](https://owasp.org/www-project-top-ten/) - Web application security risks
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/) - Security best practices for Rust

---

**Specification Version**: 3.1.0  
**Last Updated**: 2025-11-17  
**Status**: Enhanced with unconditional modular architecture requirements
**Next Review**: Before Phase 2 implementation begins
**Changes in v3.1.0**:
- Added FR-003a through FR-003e: Explicit MANDATORY modular package structure
- Clarified that ALL functionality MUST be in packages/
- Prohibited functionality implementation outside packages/
- Required future repository extraction capability
**Changes in v3.0.0**:
- Added shared infrastructure requirements (SHR-001 through SHR-015)
- Added UPDL system requirements (UPDL-001 through UPDL-015)
- Added template system requirements (TMPL-001 through TMPL-015)
- Updated phased implementation with new structure
- Added new risks (RISK-007 through RISK-010)
- Expanded glossary with UPDL and template terms
- Referenced ARCHITECTURAL-COMPARISON.md for detailed analysis
