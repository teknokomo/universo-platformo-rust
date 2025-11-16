<!--
Sync Impact Report - Constitution v1.2.0
========================================
Version Change: 1.1.0 → 1.2.0 (Non-functional requirements and integration contracts)
Date: 2025-11-16

Principles Added in v1.2.0:
- IX. Non-Functional Requirements Priority (NEW)
- X. Integration Contracts (NEW)
- XI. Risk Management (NEW)

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
- None - all placeholders filled
-->

# Universo Platformo Rust Constitution

## Core Principles

### I. Monorepo with Rust Workspace

The project MUST be organized as a monorepo using Cargo workspace management. All packages MUST be defined in the root `Cargo.toml` workspace configuration. This enables unified dependency management, consistent tooling, and efficient cross-package development.

**Rationale**: Monorepo structure facilitates code sharing, simplifies dependency management, and enables atomic cross-package changes. Cargo workspaces provide native Rust tooling support for this pattern.

### II. Package Structure Convention

All packages MUST be organized in the `packages/` directory following these rules:
- Packages requiring both frontend and backend MUST be split into separate packages with `-frt` (frontend) and `-srv` (server/backend) suffixes
- Example: `packages/clusters-frt` and `packages/clusters-srv`
- Each package MUST contain a root `base/` directory to accommodate future alternative implementations
- Package naming MUST be lowercase with hyphens as separators

**Rationale**: Clear separation of frontend and backend code enables independent deployment and testing. The `base/` directory pattern prepares the codebase for future technology stack alternatives while maintaining a consistent interface.

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

**Version**: 1.2.0 | **Ratified**: 2025-11-15 | **Last Amended**: 2025-11-16
