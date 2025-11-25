# Tasks: Initialize Universo Platformo Rust Project

**Input**: Design documents from `/.specify/specs/001-initialize-rust-platformo/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, ARCHITECTURAL-COMPARISON.md, IMPLEMENTATION-ROADMAP.md

**Tests**: Tests are NOT explicitly requested in the specification, therefore NO test tasks are included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

**Scope**: This tasks.md covers **Phase 1: Foundation + Shared Infrastructure** from IMPLEMENTATION-ROADMAP.md. Future features (Authentication, Uniks, Metaverses, Spaces/Canvases, Node Libraries, Publication System) will be implemented in subsequent feature branches following the established patterns.

## Format: `- [ ] [ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- All tasks include exact file paths

## Critical Architecture Principles

⚠️ **MANDATORY MODULAR STRUCTURE**: ALL application functionality MUST be implemented as packages in `packages/` directory. Repository root is ONLY for workspace configuration.

📋 **Package Naming Convention**: 
- Frontend packages: `[feature]-frt` (e.g., `auth-frt`, `spaces-frt`)
- Backend packages: `[feature]-srv` (e.g., `auth-srv`, `spaces-srv`)
- Shared packages: `universo-[name]` (e.g., `universo-types`, `universo-utils`)
- Each package contains `base/` subdirectory for main implementation

🎯 **Future-Ready Structure**: This initialization establishes patterns for:
- **Authentication** → auth-frt/auth-srv packages (Phase 2)
- **Uniks (Workspaces)** → uniks-frt/uniks-srv packages (Phase 3)
- **Metaverses** → metaverses-frt/metaverses-srv packages (Phase 3)
- **Spaces/Canvases** → spaces-frt/spaces-srv packages with node system (Phase 4)
- **Node Libraries** → LangChain and UPDL node packages (Phase 4)
- **Publication System** → publish-frt/publish-srv packages (Phase 5)

## Relationship to Universo Platformo React

This Rust implementation mirrors the proven package architecture from [Universo Platformo React](https://github.com/teknokomo/universo-platformo-react/tree/main/packages) while applying Rust ecosystem best practices:

### Key Differences from React Version

1. **Optimal Package Separation from Start**
   - React repo has monolithic Flowise packages that will be split later
   - Rust implementation creates properly separated packages immediately
   - Avoid technical debt by planning modular structure from beginning

2. **Technology Stack Adaptations**
   - React/TypeScript → Rust/Yew (WASM)
   - Express.js → Actix Web
   - TypeORM → SQLx with repository traits
   - PNPM workspaces → Cargo workspaces
   - Material-UI → Yew Material components

3. **Shared Infrastructure First**
   - React repo added shared packages (`@universo/types`, `@universo/utils`) after discovering duplication
   - Rust implementation creates shared packages (universo-types, universo-utils, etc.) in Phase 1
   - Prevents code duplication from the start

### Functionality Progression (Matching React Patterns)

The development will follow this progression, similar to React version:

1. **Foundation** (This Phase) → Repository setup, shared packages
2. **Authentication** → Login/registration pages, session management
3. **Uniks/Workspaces** → User workspace management
4. **Clusters** → Three-entity pattern (Clusters/Domains/Resources)
5. **Metaverses** → Three-entity pattern (Metaverses/Sections/Entities)
6. **Spaces/Canvases** → Visual flow editor for node-based development
7. **Node Libraries**:
   - UPDL nodes (Scene, Entity, Transform, Material, Interaction, Animation, Export)
   - LangChain integration nodes (from flowise-components pattern)
   - Custom node system extensibility
8. **Template System** → Multi-platform export (AR.js, PlayCanvas, Babylon.js, Three.js)
9. **Publication System** → Share and discover published experiences
10. **Advanced Features** → Space Builder (AI-assisted), Multiplayer, Analytics

### React Package Reference

For developers familiar with the React version, here's the mapping:

| React Package | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `@universo/types` | `universo-types` | Shared type definitions |
| `@universo/utils` | `universo-utils` | Common utilities, UPDL processor |
| `@universo/api-client` | `universo-api-client` | HTTP client |
| `@universo/i18n` | `universo-i18n` | Internationalization |
| `flowise-template-mui` | `universo-ui-components` | Yew components (Rust equivalent) |
| `universo-rest-docs` | `universo-rest-docs` | API documentation server |
| `auth-frt` | `auth-frt` | Authentication frontend |
| `auth-srv` | `auth-srv` | Authentication backend |
| `profile-frt` | `profile-frt` | User profile frontend |
| `profile-srv` | `profile-srv` | User profile backend |
| `organizations-frt` | `organizations-frt` | Organizations frontend |
| `organizations-srv` | `organizations-srv` | Organizations backend |
| `clusters-frt` | `clusters-frt` | Clusters frontend |
| `clusters-srv` | `clusters-srv` | Clusters backend |
| `metaverses-frt` | `metaverses-frt` | Metaverses frontend |
| `metaverses-srv` | `metaverses-srv` | Metaverses backend |
| `uniks-frt` | `uniks-frt` | Uniks/Workspaces frontend |
| `uniks-srv` | `uniks-srv` | Uniks/Workspaces backend |
| `spaces-frt` | `spaces-frt` | Spaces/Canvases frontend |
| `spaces-srv` | `spaces-srv` | Spaces/Canvases backend |
| `storages-frt` | `storages-frt` | Storages frontend |
| `storages-srv` | `storages-srv` | Storages backend |
| `projects-frt` | `projects-frt` | Projects frontend |
| `projects-srv` | `projects-srv` | Projects backend |
| `updl/base` | `updl-nodes` | UPDL node definitions |
| `flowise-components` | `langchain-nodes` | Node system (fresh implementation, not a port) |
| `template-quiz` | `template-quiz` | AR.js Quiz template (same name as React) |
| `template-mmoomm` | `template-mmoomm` | PlayCanvas MMOOMM template (same name as React) |
| `publish-frt` | `publish-frt` | Publication frontend |
| `publish-srv` | `publish-srv` | Publication backend |
| `analytics-frt` | `analytics-frt` | Analytics dashboard frontend |
| `space-builder-frt` | `space-builder-frt` | AI-assisted flow generation |
| `space-builder-srv` | `space-builder-srv` | LLM integration |
| `multiplayer-colyseus-srv` | `multiplayer-srv` | Multiplayer server (Rust-native) |

**Critical Learning**: The React version has `flowise-components` (17MB CJS, 5.2MB ESM) which contains mixed functionality that should be separated. The Rust implementation will avoid this by:
- Creating separate packages for LangChain nodes, UPDL nodes, and core components from the start
- Using clear package boundaries and dependencies
- Following the optimal structure learned from React version's evolution
- NOT porting Flowise code - implementing fresh Rust implementations

## Phase 1: Setup (Project Initialization)

**Purpose**: Initialize repository structure and basic configuration

- [ ] T001 Create root Cargo.toml workspace configuration with members list
- [ ] T002 Create rust-toolchain.toml specifying Rust 1.75+ stable
- [ ] T003 [P] Create .gitignore for Rust projects (target/, Cargo.lock for libraries, .env)
- [ ] T004 [P] Create packages/ directory for all application packages
- [ ] T005 [P] Create .github/workflows/ directory for CI/CD pipeline
- [ ] T006 [P] Create .vscode/settings.json with rust-analyzer configuration

---

## Phase 2: Foundational (Shared Infrastructure - BLOCKS ALL USER STORIES)

**Purpose**: Core infrastructure packages that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

**Architecture Reference**: These packages mirror the proven patterns from Universo Platformo React:
- `@universo/types` → `universo-types` (shared type definitions)
- `@universo/utils` → `universo-utils` (common utilities, UPDL processor)
- `@universo/api-client` → `universo-api-client` (HTTP client)
- `@universo/i18n` → `universo-i18n` (internationalization)
- Material UI components → `universo-ui-components` (Yew components)

### Shared Type Package (universo-types)

- [ ] T007 Create packages/universo-types/base/ directory structure
- [ ] T008 Create packages/universo-types/base/Cargo.toml with serde, uuid, chrono dependencies
- [ ] T009 [P] Create packages/universo-types/base/src/lib.rs with module exports
- [ ] T010 [P] Create packages/universo-types/base/src/api/mod.rs with Pagination struct
- [ ] T011 [P] Create packages/universo-types/base/src/api/response.rs with ApiResponse enum
- [ ] T012 [P] Create packages/universo-types/base/src/api/error.rs with ApiErrorCode enum
- [ ] T013 [P] Create packages/universo-types/base/src/entities/mod.rs with base traits
- [ ] T014 [P] Create packages/universo-types/base/src/entities/base.rs with BaseEntity struct
- [ ] T015 [P] Create packages/universo-types/base/src/common/mod.rs with UserIdentity
- [ ] T016 [P] Create packages/universo-types/base/src/common/language.rs with Language enum
- [ ] T017 [P] Create packages/universo-types/base/src/common/validation.rs with ValidationError
- [ ] T018 [P] Create packages/universo-types/base/src/updl/mod.rs with placeholder UPDL types

### Shared Utilities Package (universo-utils)

- [ ] T019 Create packages/universo-utils/base/ directory structure
- [ ] T020 Create packages/universo-utils/base/Cargo.toml with dependencies
- [ ] T021 [P] Create packages/universo-utils/base/src/lib.rs with module exports
- [ ] T022 [P] Create packages/universo-utils/base/src/helpers/mod.rs for utility functions
- [ ] T023 [P] Create packages/universo-utils/base/src/processors/mod.rs with UPDL processor placeholder

### API Client Package (universo-api-client)

- [ ] T024 Create packages/universo-api-client/base/ directory structure
- [ ] T025 Create packages/universo-api-client/base/Cargo.toml with reqwest dependency
- [ ] T026 [P] Create packages/universo-api-client/base/src/lib.rs with module exports
- [ ] T027 [P] Create packages/universo-api-client/base/src/client.rs with HTTP client wrapper

### Internationalization Package (universo-i18n)

- [ ] T028 Create packages/universo-i18n/base/ directory structure
- [ ] T029 Create packages/universo-i18n/base/Cargo.toml with i18n dependencies
- [ ] T030 [P] Create packages/universo-i18n/base/src/lib.rs with module exports
- [ ] T031 [P] Create packages/universo-i18n/base/src/locales/ directory for EN/RU translations
- [ ] T032 [P] Create packages/universo-i18n/base/src/translator.rs with translation functions

### UI Components Package (universo-ui-components)

- [ ] T033 Create packages/universo-ui-components/base/ directory structure
- [ ] T034 Create packages/universo-ui-components/base/Cargo.toml with yew dependency
- [ ] T035 [P] Create packages/universo-ui-components/base/src/lib.rs with component exports
- [ ] T036 [P] Create packages/universo-ui-components/base/src/components/mod.rs for UI components
- [ ] T037 [P] Create packages/universo-ui-components/base/src/components/button.rs with Button component
- [ ] T038 [P] Create packages/universo-ui-components/base/src/components/input.rs with Input component

### Update Root Workspace

- [ ] T039 Update root Cargo.toml to include all five shared packages as workspace members
- [ ] T040 Add workspace dependencies section in root Cargo.toml for shared dependencies
- [ ] T041 Verify cargo build --workspace compiles all shared packages successfully

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Repository Setup and Structure (Priority: P1) 🎯 MVP

**Goal**: Create comprehensive repository documentation (README.md, README-RU.md) and basic directory structure so developers can understand the project and start contributing

**Independent Test**: Clone repository, read README files in both languages, verify directory structure matches monorepo specification with packages/ directory and proper organization

### Documentation Files

- [ ] T042 [P] [US1] Create README.md in English with project overview, relationship to React version, tech stack
- [ ] T043 [US1] Create README-RU.md as exact translation of README.md with identical structure
- [ ] T044 [P] [US1] Add section to README.md explaining Universo Platformo concept and mission
- [ ] T045 [P] [US1] Add section to README.md about Rust/Yew/Actix Web tech stack
- [ ] T046 [P] [US1] Add section to README.md explaining monorepo structure with packages/
- [ ] T047 [P] [US1] Add section to README.md about relationship to Universo Platformo React
- [ ] T048 [P] [US1] Add section to README.md about Supabase database and future DBMS support
- [ ] T049 [P] [US1] Add Getting Started section to README.md with prerequisites
- [ ] T050 [P] [US1] Add Development Guide section to README.md with build/test commands
- [ ] T051 [P] [US1] Add Contributing section to README.md referencing .github/instructions/
- [ ] T052 [US1] Mirror all README.md changes to README-RU.md maintaining identical structure
- [ ] T053 [US1] Verify line count and section structure match between README.md and README-RU.md

### GitHub Instructions Files

- [ ] T054 [P] [US1] Create .github/instructions/ directory
- [ ] T055 [P] [US1] Create .github/instructions/github-issues.md with issue creation guidelines
- [ ] T056 [P] [US1] Create .github/instructions/github-labels.md with label usage guidelines
- [ ] T057 [P] [US1] Create .github/instructions/github-pr.md with PR guidelines
- [ ] T058 [P] [US1] Create .github/instructions/i18n-docs.md with i18n documentation standards

### Project Configuration

- [ ] T059 [P] [US1] Create LICENSE file (specify license type based on project requirements)
- [ ] T060 [P] [US1] Create .editorconfig for consistent code formatting across editors
- [ ] T061 [P] [US1] Create rustfmt.toml with project-standard Rust formatting configuration

**Checkpoint**: At this point, User Story 1 should be fully functional - developers can clone, read documentation, and understand the project structure

---

## Phase 4: User Story 2 - Issue Labels and Workflow Foundation (Priority: P2)

**Goal**: Create complete set of GitHub issue labels matching the project structure to organize and track work across frontend, backend, and platform areas

**Independent Test**: Navigate to repository Issues page, verify all required labels exist with appropriate colors and descriptions for types, areas, and project context

### Type Labels

- [ ] T062 [P] [US2] Create GitHub label "type: feature" (color: 0e8a16) for new features
- [ ] T063 [P] [US2] Create GitHub label "type: bug" (color: d73a4a) for bug reports
- [ ] T064 [P] [US2] Create GitHub label "type: enhancement" (color: a2eeef) for improvements
- [ ] T065 [P] [US2] Create GitHub label "type: documentation" (color: 0075ca) for docs
- [ ] T066 [P] [US2] Create GitHub label "type: refactor" (color: fbca04) for refactoring
- [ ] T067 [P] [US2] Create GitHub label "type: maintenance" (color: fef2c0) for maintenance

### Area Labels

- [ ] T068 [P] [US2] Create GitHub label "area: frontend" (color: c5def5) for frontend work
- [ ] T069 [P] [US2] Create GitHub label "area: backend" (color: bfd4f2) for backend work
- [ ] T070 [P] [US2] Create GitHub label "area: build" (color: d4c5f9) for build system
- [ ] T071 [P] [US2] Create GitHub label "area: testing" (color: c2e0c6) for testing
- [ ] T072 [P] [US2] Create GitHub label "area: i18n" (color: fad8c7) for internationalization
- [ ] T073 [P] [US2] Create GitHub label "area: architecture" (color: e99695) for architecture

### Project Context Labels

- [ ] T074 [P] [US2] Create GitHub label "platformo" (color: 006b75) for Universo Platformo specific
- [ ] T075 [P] [US2] Create GitHub label "rust" (color: dea584) for Rust-specific issues
- [ ] T076 [P] [US2] Create GitHub label "repository" (color: 0366d6) for repo management
- [ ] T077 [P] [US2] Create GitHub label "releases" (color: 5319e7) for release planning

### Priority Labels

- [ ] T078 [P] [US2] Create GitHub label "priority: critical" (color: b60205) for critical issues
- [ ] T079 [P] [US2] Create GitHub label "priority: high" (color: d93f0b) for high priority
- [ ] T080 [P] [US2] Create GitHub label "priority: medium" (color: fbca04) for medium priority
- [ ] T081 [P] [US2] Create GitHub label "priority: low" (color: c5def5) for low priority

**Checkpoint**: At this point, User Story 2 is complete - all labels exist and can be used for issue organization

---

## Phase 5: User Story 3 - Package Structure Template (Priority: P3)

**Goal**: Establish clear template for organizing frontend and backend code so all packages follow consistent structure and naming conventions

**Independent Test**: Examine packages/ directory, verify presence of example/documentation showing -frt/-srv naming pattern with base/ subdirectories

### Documentation

- [ ] T082 [P] [US3] Create PACKAGE-STRUCTURE.md documenting package naming conventions
- [ ] T083 [P] [US3] Add section to PACKAGE-STRUCTURE.md explaining -frt/-srv suffix pattern
- [ ] T084 [P] [US3] Add section to PACKAGE-STRUCTURE.md explaining base/ subdirectory purpose
- [ ] T085 [P] [US3] Add section to PACKAGE-STRUCTURE.md with example feature package structure
- [ ] T086 [P] [US3] Add section to PACKAGE-STRUCTURE.md explaining cross-package dependencies
- [ ] T087 [P] [US3] Add section to PACKAGE-STRUCTURE.md about single-component packages
- [ ] T088 [P] [US3] Add section to PACKAGE-STRUCTURE.md about shared packages (-common/-shared)
- [ ] T089 [US3] Create PACKAGE-STRUCTURE-RU.md as exact Russian translation

### Package Template Example

- [ ] T090 [P] [US3] Create packages/README.md explaining package organization
- [ ] T091 [US3] Create packages/README-RU.md as exact Russian translation
- [ ] T092 [P] [US3] Add package template documentation to packages/README.md
- [ ] T093 [P] [US3] Document future package extraction strategy in packages/README.md
- [ ] T094 [P] [US3] Add examples of -frt, -srv, -common package relationships

### Package Creation Guide

- [ ] T095 [P] [US3] Create CONTRIBUTING.md with "Creating a New Package" section
- [ ] T096 [P] [US3] Add step-by-step guide for creating frontend package in CONTRIBUTING.md
- [ ] T097 [P] [US3] Add step-by-step guide for creating backend package in CONTRIBUTING.md
- [ ] T098 [P] [US3] Add step-by-step guide for creating shared package in CONTRIBUTING.md
- [ ] T099 [US3] Create CONTRIBUTING-RU.md as exact Russian translation

**Checkpoint**: At this point, User Story 3 is complete - developers understand package structure and can create new packages following the template

---

## Phase 6: User Story 4 - Development Environment Configuration (Priority: P4)

**Goal**: Provide configuration files for Cargo workspace and build tools so developers can compile and run the project locally with all dependencies properly managed

**Independent Test**: Run `cargo build --workspace` from repository root and verify workspace compiles successfully with proper dependency resolution

### Cargo Workspace Configuration

- [ ] T100 [US4] Verify root Cargo.toml has all workspace members listed
- [ ] T101 [US4] Add workspace.dependencies section with common crate versions
- [ ] T102 [P] [US4] Add serde = "1.0" with derive feature to workspace dependencies
- [ ] T103 [P] [US4] Add tokio = "1.35" with full feature to workspace dependencies
- [ ] T104 [P] [US4] Add actix-web = "4.4" to workspace dependencies
- [ ] T105 [P] [US4] Add yew = "0.21" to workspace dependencies
- [ ] T106 [P] [US4] Add uuid = "1.6" with serde, v4 features to workspace dependencies
- [ ] T107 [P] [US4] Add chrono = "0.4" with serde feature to workspace dependencies
- [ ] T108 [P] [US4] Add reqwest = "0.11" to workspace dependencies
- [ ] T109 [US4] Verify all package Cargo.toml files reference workspace dependencies correctly

### Build and Development Tools

- [ ] T110 [P] [US4] Create .cargo/config.toml with build optimization settings
- [ ] T111 [P] [US4] Configure WASM target settings in .cargo/config.toml
- [ ] T112 [P] [US4] Create Trunk.toml for frontend package build configuration
- [ ] T113 [P] [US4] Configure wasm-opt settings in Trunk.toml for release builds
- [ ] T114 [P] [US4] Create .env.example with required environment variables template
- [ ] T115 [P] [US4] Document DATABASE_URL format in .env.example
- [ ] T116 [P] [US4] Document SUPABASE_URL and keys in .env.example
- [ ] T117 [P] [US4] Document server configuration (HOST, PORT) in .env.example

### CI/CD Pipeline

- [ ] T118 [US4] Create .github/workflows/ci.yml for continuous integration
- [ ] T119 [US4] Add cargo build --workspace step to ci.yml
- [ ] T120 [US4] Add cargo test --workspace step to ci.yml
- [ ] T121 [US4] Add cargo clippy --workspace -- -D warnings step to ci.yml
- [ ] T122 [US4] Add cargo fmt --all -- --check step to ci.yml
- [ ] T123 [US4] Add cargo audit step to ci.yml for security vulnerabilities
- [ ] T124 [US4] Add WASM build verification step for frontend packages to ci.yml
- [ ] T125 [US4] Configure caching for cargo dependencies in ci.yml
- [ ] T126 [US4] Configure workflow to run on push to feature branches and PRs

### Development Scripts

- [ ] T127 [P] [US4] Create scripts/dev.sh for running backend server with hot reload
- [ ] T128 [P] [US4] Create scripts/build-frontend.sh for building frontend WASM bundles
- [ ] T129 [P] [US4] Create scripts/test-all.sh for running all tests across workspace
- [ ] T130 [P] [US4] Create scripts/lint.sh for running clippy and rustfmt checks
- [ ] T131 [P] [US4] Create scripts/setup-db.sh for database initialization
- [ ] T132 [US4] Make all scripts executable with chmod +x

### Verification

- [ ] T133 [US4] Run cargo build --workspace and verify successful compilation
- [ ] T134 [US4] Run cargo test --workspace and verify all tests pass (or no tests yet)
- [ ] T135 [US4] Run cargo clippy --workspace and verify zero warnings
- [ ] T136 [US4] Run cargo fmt --all -- --check and verify code is formatted
- [ ] T137 [US4] Verify rust-analyzer works correctly in VS Code with workspace
- [ ] T138 [US4] Document build time and verify it meets <5min requirement

**Checkpoint**: At this point, User Story 4 is complete - full development environment is configured and functional

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements and documentation that affect multiple user stories

### Health Check Endpoint Implementation

- [ ] T139 [P] Create packages/health-srv/base/ directory structure for health check service
- [ ] T140 [P] Create packages/health-srv/base/Cargo.toml with actix-web dependencies
- [ ] T141 [P] Create packages/health-srv/base/src/lib.rs with health check handler
- [ ] T142 [P] Create packages/health-srv/base/src/routes.rs implementing GET /api/v1/health
- [ ] T143 [P] Create packages/health-srv/base/src/models.rs with HealthCheckResponse struct
- [ ] T144 Add packages/health-srv/base to root Cargo.toml workspace members

### Additional Documentation

- [ ] T145 [P] Create ARCHITECTURE.md documenting high-level system architecture
- [ ] T146 [US] Create ARCHITECTURE-RU.md as exact Russian translation
- [ ] T147 [P] Update README.md with badges (build status, license, version)
- [ ] T148 Update README-RU.md to mirror README.md badge updates
- [ ] T149 [P] Create CHANGELOG.md for tracking version changes
- [ ] T150 [P] Add initial v0.1.0 entry to CHANGELOG.md

### Repository Metadata

- [ ] T151 [P] Create CODE_OF_CONDUCT.md for community guidelines
- [ ] T152 [P] Create SECURITY.md with security policy and vulnerability reporting
- [ ] T153 [P] Create .github/ISSUE_TEMPLATE/ directory
- [ ] T154 [P] Create .github/ISSUE_TEMPLATE/bug_report.md template
- [ ] T155 [P] Create .github/ISSUE_TEMPLATE/feature_request.md template
- [ ] T156 [P] Create .github/PULL_REQUEST_TEMPLATE.md

### Quality Assurance

- [ ] T157 Run cargo build --workspace and verify clean build
- [ ] T158 Run cargo clippy --workspace -- -D warnings and fix any warnings
- [ ] T159 Run cargo fmt --all and commit formatting changes
- [ ] T160 Verify all README files have matching English and Russian versions
- [ ] T161 Run scripts from quickstart.md manually to verify they work
- [ ] T162 Verify all .github/instructions/ files are complete and accurate
- [ ] T163 Check that all 5 shared infrastructure packages compile successfully
- [ ] T164 Verify rust-toolchain.toml specifies correct Rust version
- [ ] T165 Final review of all documentation for completeness and accuracy

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent from other stories
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Independent from other stories
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Independent from other stories

**Note**: All user stories are designed to be independently implementable and testable

### Within Each User Story

**User Story 1**:
- Documentation files before verification
- English documentation before Russian translations
- All content complete before structure verification

**User Story 2**:
- All labels can be created in parallel
- No dependencies between labels

**User Story 3**:
- Documentation files can be created in parallel
- English documentation before Russian translations
- Package template examples independent

**User Story 4**:
- Cargo configuration before CI/CD
- CI/CD before verification steps
- Scripts can be created in parallel
- Verification steps must be last

### Parallel Opportunities

- **Phase 1**: Tasks T003, T004, T005, T006 can run in parallel
- **Phase 2**: Most tasks within each package can run in parallel (marked with [P])
  - All 5 shared packages (universo-types, universo-utils, universo-api-client, universo-i18n, universo-ui-components) can be developed in parallel by different team members
  - Within each package, module files can be created in parallel
- **Phase 3 (US1)**: 
  - Tasks T042-T051 (English README sections) can run in parallel
  - Tasks T054-T058 (GitHub instructions) can run in parallel
  - Tasks T059-T061 (project configuration) can run in parallel
- **Phase 4 (US2)**: All label creation tasks (T062-T081) can run in parallel
- **Phase 5 (US3)**:
  - Tasks T082-T088 (English documentation) can run in parallel
  - Tasks T090-T094 (package documentation) can run in parallel
  - Tasks T095-T098 (contribution guide) can run in parallel
- **Phase 6 (US4)**:
  - Tasks T102-T108 (workspace dependencies) can run in parallel
  - Tasks T110-T117 (build and dev tools) can run in parallel
  - Tasks T127-T131 (scripts) can run in parallel
- **Phase 7**: Most documentation tasks (T139-T156) can run in parallel

---

## Parallel Example: Phase 2 (Foundational)

If you have a team of 5 developers, the shared infrastructure packages can be built in parallel:

```bash
# Developer A: universo-types package
Task T007-T018: Complete universo-types package

# Developer B: universo-utils package
Task T019-T023: Complete universo-utils package

# Developer C: universo-api-client package
Task T024-T027: Complete universo-api-client package

# Developer D: universo-i18n package
Task T028-T032: Complete universo-i18n package

# Developer E: universo-ui-components package
Task T033-T038: Complete universo-ui-components package

# Then one developer (or automated):
Task T039-T041: Update root workspace and verify build
```

---

## Parallel Example: User Stories (Phase 3-6)

Once Phase 2 is complete, all user stories can proceed in parallel:

```bash
# Developer A: User Story 1 (Repository Setup)
Task T042-T061: Complete all US1 tasks

# Developer B: User Story 2 (Issue Labels)
Task T062-T081: Complete all US2 tasks

# Developer C: User Story 3 (Package Structure)
Task T082-T099: Complete all US3 tasks

# Developer D: User Story 4 (Dev Environment)
Task T100-T138: Complete all US4 tasks
```

---

## Implementation Strategy

### MVP First (Critical Path)

**Goal**: Get to a working repository with clear documentation ASAP

1. Complete **Phase 1: Setup** (T001-T006) - ~1 hour
2. Complete **Phase 2: Foundational** (T007-T041) - ~2-3 days
   - This creates all shared infrastructure packages
   - CRITICAL: Nothing else can proceed until this is done
3. Complete **Phase 3: User Story 1** (T042-T061) - ~1 day
   - This creates all essential documentation
   - Makes the repository understandable to new developers
4. **STOP and VALIDATE**: 
   - Can a new developer clone and understand the project?
   - Does `cargo build --workspace` succeed?
5. Optional: Complete Phase 4 (US2 - Labels) for better issue tracking

**MVP Scope**: Phases 1, 2, and 3 only (Setup + Foundational + Repository Documentation)

### Incremental Delivery

**Full Feature Set**: All 4 user stories

1. **Foundation** (Phases 1-2): Setup + All shared infrastructure → Foundation complete
2. **+ User Story 1** (Phase 3): Documentation → Repository ready for contributors
3. **+ User Story 2** (Phase 4): Labels → Issue tracking organized
4. **+ User Story 3** (Phase 5): Package templates → New feature development enabled
5. **+ User Story 4** (Phase 6): Dev environment → Full development workflow ready
6. **+ Polish** (Phase 7): Final touches → Production ready

Each addition builds on previous work without breaking existing functionality.

### Parallel Team Strategy

**Optimal Team Size**: 4-6 developers

**Phase 2 (Foundational)** - Parallel by package:
- Dev 1: universo-types
- Dev 2: universo-utils  
- Dev 3: universo-api-client
- Dev 4: universo-i18n
- Dev 5: universo-ui-components
- Dev 6: Workspace integration (T039-T041)

**Phases 3-6 (User Stories)** - Parallel by story:
- Dev 1: User Story 1 (Repository Setup)
- Dev 2: User Story 2 (Issue Labels)
- Dev 3: User Story 3 (Package Structure)
- Dev 4: User Story 4 (Dev Environment)

**Phase 7 (Polish)** - Parallel by area:
- Dev 1: Health check implementation
- Dev 2: Additional documentation
- Dev 3: Repository metadata
- Dev 4: Quality assurance

---

## Validation Checkpoints

### After Phase 1 (Setup)
- [ ] Root Cargo.toml exists with workspace configuration
- [ ] rust-toolchain.toml specifies Rust 1.75+
- [ ] packages/ directory exists

### After Phase 2 (Foundational)
- [ ] All 5 shared packages exist with proper structure
- [ ] `cargo build --workspace` completes successfully
- [ ] All packages have proper Cargo.toml with dependencies
- [ ] Each package exports types/functions correctly

### After Phase 3 (User Story 1)
- [ ] README.md and README-RU.md exist with identical structure
- [ ] All .github/instructions/ files exist
- [ ] New developer can understand project from documentation
- [ ] Line count matches between EN and RU documentation

### After Phase 4 (User Story 2)
- [ ] All 20+ labels exist in GitHub Issues
- [ ] Labels have appropriate colors and descriptions
- [ ] Labels cover types, areas, project context, and priorities

### After Phase 5 (User Story 3)
- [ ] PACKAGE-STRUCTURE.md documents naming conventions
- [ ] packages/README.md explains organization
- [ ] CONTRIBUTING.md has new package creation guide
- [ ] All documentation has EN and RU versions

### After Phase 6 (User Story 4)
- [ ] Workspace dependencies configured correctly
- [ ] CI/CD pipeline runs successfully
- [ ] All development scripts work
- [ ] `cargo build --workspace` completes in <5 minutes
- [ ] `cargo clippy` shows zero warnings
- [ ] `cargo fmt` shows clean formatting

### After Phase 7 (Polish)
- [ ] Health check endpoint implemented and functional
- [ ] All documentation complete and synchronized
- [ ] All quality gates passing
- [ ] Repository ready for feature development

---

## Notes

- **[P] tasks**: Can run in parallel (different files, no dependencies on incomplete work)
- **[Story] labels**: Map tasks to user stories for traceability
  - [US1]: Repository Setup and Structure
  - [US2]: Issue Labels and Workflow Foundation
  - [US3]: Package Structure Template
  - [US4]: Development Environment Configuration
- **No tests**: Tests are not explicitly requested in the specification, so no test tasks are included
- **Independent stories**: Each user story can be completed and tested independently
- **Commit frequently**: Commit after each task or logical group of related tasks
- **Stop at checkpoints**: Validate independently before proceeding to next phase
- **Bilingual requirement**: All documentation must have EN and RU versions with identical structure

---

## Summary

**Total Tasks**: 165
- Phase 1 (Setup): 6 tasks
- Phase 2 (Foundational): 35 tasks (5 shared infrastructure packages)
- Phase 3 (User Story 1 - Repository Setup): 20 tasks
- Phase 4 (User Story 2 - Issue Labels): 20 tasks
- Phase 5 (User Story 3 - Package Structure): 18 tasks
- Phase 6 (User Story 4 - Dev Environment): 39 tasks
- Phase 7 (Polish): 27 tasks

**Parallel Opportunities**: 
- Phase 2: All 5 shared packages can be developed in parallel
- Phases 3-6: All 4 user stories can be developed in parallel after Phase 2
- Within each phase: Tasks marked [P] can run in parallel

**MVP Scope**: Phases 1-3 (41 tasks)
- Establishes foundation and essential documentation
- Enables immediate contributor onboarding

**Full Delivery**: All 7 phases (165 tasks)
- Complete foundation for Universo Platformo Rust
- Ready for feature development (Authentication, Clusters, Metaverses, Spaces, etc.)

**Estimated Effort**:
- MVP (Phases 1-3): 3-5 days (serial) or 2-3 days (parallel team)
- Full (All phases): 3-4 weeks (serial) or 1-2 weeks (parallel team)

**Next Phases After Completion** (see IMPLEMENTATION-ROADMAP.md):
- **Phase 2**: UPDL System and Clusters Feature (5-7 weeks)
- **Phase 3**: Authentication and Profile Management (3-4 weeks)
- **Phase 4**: Uniks (Workspaces) Feature (4-5 weeks)
- **Phase 5**: Metaverses Feature (4-5 weeks)
- **Phase 6**: Spaces/Canvases with Node System (6-8 weeks)
  - Visual flow editor integration
  - LangChain node library
  - UPDL node library
  - Node execution engine
- **Phase 7**: Template System (AR.js, PlayCanvas) (5-6 weeks)
- **Phase 8**: Publication System (4-5 weeks)
  - Publish flows as public experiences
  - Sharing and discovery features
- **Phase 9+**: Advanced Features
  - Space Builder (AI-assisted flow generation)
  - Multiplayer infrastructure
  - Analytics dashboard
  - OAuth2 authentication
  - Additional template platforms

## Package Structure Reference

This initialization creates the foundation for a well-organized package ecosystem:

### Shared Infrastructure (Created in Phase 1)
```
packages/
├── universo-types/base/         # Shared type definitions
├── universo-utils/base/         # Common utilities, UPDL processor
├── universo-api-client/base/    # Centralized HTTP client
├── universo-i18n/base/          # Internationalization (EN/RU)
└── universo-ui-components/base/ # Reusable Yew components
```

### Domain Features (Future Phases)
```
packages/
├── auth-frt/base/               # Authentication frontend (Phase 2)
├── auth-srv/base/               # Authentication backend (Phase 2)
├── profile-frt/base/            # User profile frontend (Phase 2)
├── profile-srv/base/            # User profile backend (Phase 2)
├── clusters-frt/base/           # Clusters frontend (Phase 2)
├── clusters-srv/base/           # Clusters backend (Phase 2)
├── organizations-frt/base/      # Organizations frontend (Phase 3)
├── organizations-srv/base/      # Organizations backend (Phase 3)
├── uniks-frt/base/              # Uniks/Workspaces frontend (Phase 4)
├── uniks-srv/base/              # Uniks/Workspaces backend (Phase 4)
├── metaverses-frt/base/         # Metaverses frontend (Phase 5)
├── metaverses-srv/base/         # Metaverses backend (Phase 5)
├── spaces-frt/base/             # Spaces frontend (Phase 6)
├── spaces-srv/base/             # Spaces backend (Phase 6)
├── storages-frt/base/           # Storages frontend (Phase 7)
├── storages-srv/base/           # Storages backend (Phase 7)
├── projects-frt/base/           # Projects frontend (Phase 7)
├── projects-srv/base/           # Projects backend (Phase 7)
├── publish-frt/base/            # Publication system frontend (Phase 8)
├── publish-srv/base/            # Publication system backend (Phase 8)
├── analytics-frt/base/          # Analytics dashboard (Phase 9+)
└── analytics-srv/base/          # Analytics backend (Phase 9+)
```

### Node Libraries (Phase 6)
```
packages/
├── updl-nodes/base/             # UPDL node definitions
├── langchain-nodes/base/        # LangChain integration nodes
└── node-executor/base/          # Node execution engine
```

**Note**: Unlike the React version which has `flowise-components`, the Rust implementation will NOT port Flowise code. Instead, node functionality will be implemented from scratch in `langchain-nodes` and `updl-nodes` packages using idiomatic Rust patterns.

### Template System (Phase 7)
```
packages/
├── template-quiz/base/          # AR.js Quiz template exporter (equivalent to React template-quiz)
├── template-mmoomm/base/        # PlayCanvas MMOOMM template exporter (equivalent to React template-mmoomm)
├── template-babylonjs/base/     # Babylon.js template (future)
└── template-threejs/base/       # Three.js template (future)
```

**Template Naming Convention**: Template packages are named after their primary functionality (quiz, mmoomm) rather than the underlying technology (arjs, playcanvas), matching the React repository's naming convention for consistency.

### Support Packages (Phase 9+)
```
packages/
├── universo-rest-docs/base/     # API documentation server (OpenAPI/Swagger)
├── multiplayer-srv/base/        # Multiplayer server (WebSocket/game server)
├── space-builder-frt/base/      # AI-assisted flow generation UI
├── space-builder-srv/base/      # LLM integration backend
└── health-srv/base/             # Health check service
```

## Best Practices for Future Development

When implementing features in subsequent phases, follow these patterns established in this initialization:

### 1. Package Separation
- **Always separate frontend and backend** into `-frt` and `-srv` packages
- **Share types** through `universo-types` package
- **Reuse components** from `universo-ui-components`
- **Use centralized HTTP client** from `universo-api-client`

### 2. UPDL Integration (from ARCHITECTURAL-COMPARISON.md)
- All visual nodes should produce UPDL output
- UPDL processor converts flow graphs to UPDL structures
- Templates consume UPDL and generate platform-specific code
- Maintain platform-agnostic abstractions

### 3. Technology Stack (from research.md and constitution v1.6.0)
- **Frontend**: Yew function components with hooks (use_state, use_effect, use_reducer)
- **Backend**: Actix Web with async handlers, typed extractors, middleware
- **State Management**: Yewdux for global state, use_reducer for local state
- **Database**: SQLx with repository trait pattern for type-safe queries
- **Build**: Trunk for frontend dev server, cargo-watch for backend hot reload

### 4. Code Quality Standards
- **Zero clippy warnings** policy
- **rustfmt** formatted code
- **Comprehensive documentation** with examples
- **Bilingual docs** (EN/RU) with identical structure
- **Unit and integration tests** for all packages

### 5. Lessons from React Implementation (ARCHITECTURAL-COMPARISON.md)
- **Component extraction** saved 7,692 lines of duplication - plan sharing from start
- **Centralized type system** prevented type conflicts - use universo-types everywhere
- **UPDL as abstraction** enabled multi-platform export - design for extensibility
- **Template system** kept platform logic separate - maintain clear boundaries
