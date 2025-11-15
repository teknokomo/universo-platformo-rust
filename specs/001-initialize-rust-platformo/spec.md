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

### Assumptions

- Developers have basic familiarity with Rust, Cargo workspaces, and monorepo concepts
- The Universo Platformo React repository (https://github.com/teknokomo/universo-platformo-react) remains accessible as reference
- Supabase will be the sole database for initial implementation phases
- The project will prioritize Rust best practices over direct JavaScript/TypeScript pattern translation
- UI component library selection for Yew will be finalized during first frontend package implementation
- Authentication mechanism equivalent to Passport.js will be determined when implementing auth features
- Legacy Flowise code mentioned in React version will not be ported to Rust version
- Bilingual documentation is mandatory and will be maintained consistently across all files
- The workspace will use Cargo's native workspace features rather than requiring external tools like PNPM

### Key Entities

- **Repository**: The root monorepo containing all packages, configuration, and shared documentation
- **Package**: A self-contained module in `packages/` directory, may be frontend (-frt), backend (-srv), or shared (-common)
- **Base Implementation**: The primary Rust/Yew/Actix code within a package's `base/` subdirectory, allowing for future alternative implementations
- **Feature**: A complete functionality unit that may span multiple packages (e.g., Clusters feature includes clusters-frt and clusters-srv)
- **Label**: GitHub issue/PR categorization tags for type, area, and project context
- **Documentation**: Bilingual markdown files with English originals and Russian translations maintaining identical structure

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
