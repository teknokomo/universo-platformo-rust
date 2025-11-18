# Implementation Plan: Initialize Universo Platformo Rust Project

**Branch**: `001-initialize-rust-platformo` | **Date**: 2025-11-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-initialize-rust-platformo/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

**IMPORTANT UPDATE (2025-11-17)**: Constitution v1.5.0 and Spec v3.1.0 now UNCONDITIONALLY require ALL functionality in packages/ directory. Repository root restricted to configuration only.

## Summary

Initialize the Universo Platformo Rust project with MANDATORY modular monorepo structure using Cargo workspaces, bilingual documentation (EN/RU), shared infrastructure packages (universo-types, universo-utils, universo-api-client, universo-i18n, universo-ui-components), and comprehensive development workflow guidelines. ALL application functionality MUST be implemented as packages in packages/ directory. This foundation enables the Rust/Yew/Actix Web implementation of the platform as an alternative to the React version, with explicit preparation for future package extraction to separate repositories.

## Technical Context

**Language/Version**: Rust 1.75+ (stable channel)  
**Primary Dependencies**: Yew 0.21+ (frontend), Actix Web 4.x (backend), serde 1.x (serialization), reqwest 0.11+ (HTTP client)  
**Storage**: Supabase (PostgreSQL-based), with trait-based abstraction for future DBMS support  
**Testing**: cargo test, wasm-bindgen-test (WASM), actix-rt (async tests)  
**Target Platform**: Linux/macOS/Windows (backend), WASM32 (frontend browsers)
**Project Type**: Web application (monorepo with frontend and backend packages)  
**Performance Goals**: <5min full workspace build, <2MB compressed WASM bundle, <200ms API p95 response time, 1000+ concurrent connections  
**Constraints**: Bilingual documentation (EN/RU) required, WCAG 2.1 AA accessibility, zero clippy warnings policy  
**Scale/Scope**: Foundation for multi-feature platform (Clusters, Metaverses, Spaces, Templates), 5 shared infrastructure packages initially

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### ✅ Principle I: Monorepo with Rust Workspace
- **Status**: COMPLIANT
- **Action**: Root Cargo.toml will define workspace with all packages in packages/ directory
- **Validation**: `cargo build --workspace` must compile successfully

### ✅ Principle II: Package Structure Convention (NON-NEGOTIABLE)
- **Status**: COMPLIANT
- **Action**: ALL application functionality in packages/ directory ONLY. Repository root restricted to configuration files.
- **Critical Requirements**:
  - ✅ All packages in packages/ with -frt/-srv suffixes and base/ subdirectories
  - ✅ NO application code in repository root (NO src/ directory)
  - ✅ NO feature implementations outside packages/
  - ✅ Packages designed for future extraction to separate repositories
- **Validation**: 
  - Directory structure follows packages/[name]-{frt,srv}/base/ pattern
  - Repository root contains ONLY: Cargo.toml, rust-toolchain.toml, .gitignore, .github/, .specify/, README files
  - Zero application code files in repository root

### ✅ Principle III: Bilingual Documentation (NON-NEGOTIABLE)
- **Status**: COMPLIANT
- **Action**: All documentation in EN (README.md) and RU (README-RU.md) with identical structure
- **Validation**: Line count and section structure must match exactly between EN/RU versions

### ✅ Principle IV: Database Flexibility
- **Status**: COMPLIANT
- **Action**: Trait-based database abstraction with initial Supabase implementation
- **Validation**: Database access encapsulated through repository traits

### ✅ Principle V: Issue-Driven Development
- **Status**: COMPLIANT
- **Action**: GitHub Issues must be created following .github/instructions/github-issues.md
- **Validation**: All work has corresponding issue with proper labels

### ✅ Principle VI: Specification-First Approach
- **Status**: COMPLIANT (IN PROGRESS)
- **Action**: This spec and plan created before implementation
- **Validation**: spec.md and plan.md exist before coding begins

### ✅ Principle VII: Best Practices for Rust Fullstack
- **Status**: COMPLIANT
- **Action**: Use idiomatic Rust, Yew, Actix Web; avoid porting bad patterns from React version
- **Validation**: Code passes clippy with zero warnings, follows Rust API guidelines

### ✅ Principle VIII: Repository Boundaries and Exclusions
- **Status**: COMPLIANT
- **Action**: NO docs/ directory, NO .github/agents/ pre-created
- **Validation**: Only .github/instructions/ and .specify/ allowed

### ✅ Principle IX: Non-Functional Requirements Priority
- **Status**: COMPLIANT
- **Action**: Performance, security, accessibility, maintainability requirements explicitly defined in spec
- **Validation**: All NFR-001 through NFR-022 documented and measurable

### ✅ Principle X: Integration Contracts
- **Status**: COMPLIANT
- **Action**: Shared type packages with serde-compatible structs for API contracts
- **Validation**: universo-types package with all API request/response types

### ✅ Principle XI: Risk Management
- **Status**: COMPLIANT
- **Action**: Risks identified in spec with mitigation strategies
- **Validation**: Risk section in spec.md with probability/impact assessment

### ✅ Principle XII: Shared Infrastructure Priority
- **Status**: COMPLIANT
- **Action**: Create 5 shared packages FIRST (types, utils, api-client, i18n, ui-components)
- **Validation**: Shared packages created before any domain packages (clusters, metaverses)

### ✅ Principle XIII: Template System Architecture
- **Status**: PLANNED (Phase 3)
- **Action**: Template trait interface and UPDL processor planned for later phase
- **Validation**: Deferred to Phase 3 after Clusters feature complete

### ✅ Principle XIV: UPDL as Core Abstraction
- **Status**: PLANNED (Phase 2)
- **Action**: UPDL types and processor planned for Phase 2 with Clusters feature
- **Validation**: Deferred to Phase 2 implementation

### ✅ Principle XV: Build Tooling Strategy
- **Status**: COMPLIANT
- **Action**: Configure wasm-pack/trunk, cargo-watch, CI/CD pipeline with all quality gates
- **Validation**: All build scripts work, GitHub Actions configured

**GATE RESULT**: ✅ **PASS** - All current-phase principles compliant. Phase 2/3 principles appropriately deferred.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Monorepo Structure (Cargo Workspace)
packages/
├── universo-types/          # Shared type definitions (serde traits)
│   └── base/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── api/        # API request/response types
│       │   ├── entities/   # Database entity types
│       │   └── updl/       # UPDL node types (Phase 2)
│       └── Cargo.toml
├── universo-utils/          # Common utilities and UPDL processor
│   └── base/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── processors/ # UPDL processor (Phase 2)
│       │   └── helpers/    # General utilities
│       └── Cargo.toml
├── universo-api-client/     # Centralized HTTP client (reqwest)
│   └── base/
│       ├── src/
│       │   ├── lib.rs
│       │   └── client.rs
│       └── Cargo.toml
├── universo-i18n/           # Internationalization (EN/RU)
│   └── base/
│       ├── src/
│       │   ├── lib.rs
│       │   └── locales/
│       └── Cargo.toml
└── universo-ui-components/  # Yew components (Material Design)
    └── base/
        ├── src/
        │   ├── lib.rs
        │   └── components/
        └── Cargo.toml

# Root workspace configuration
Cargo.toml                   # Workspace definition
rust-toolchain.toml          # Rust version specification

# Documentation
README.md                    # English documentation
README-RU.md                 # Russian documentation (identical structure)

# GitHub configuration
.github/
├── instructions/
│   ├── github-issues.md     # Issue creation guidelines
│   ├── github-labels.md     # Label usage guidelines
│   ├── github-pr.md         # PR guidelines
│   └── i18n-docs.md         # i18n documentation standards
└── workflows/
    └── ci.yml               # CI/CD pipeline

# Specification workflow
.specify/
├── memory/
│   └── constitution.md      # Project constitution
├── scripts/
│   └── bash/
│       ├── setup-plan.sh
│       └── update-agent-context.sh
└── templates/
    ├── plan-template.md
    └── spec-template.md

# Build artifacts (gitignored)
target/                      # Cargo build output
```

**Structure Decision**: Chose monorepo with Cargo workspace for unified dependency management and efficient cross-package development. All packages follow the -frt/-srv suffix convention with base/ subdirectories to support future alternative implementations. Shared infrastructure packages (types, utils, api-client, i18n, ui-components) are created first to prevent code duplication across domain packages.

## Complexity Tracking

> **No constitution violations** - This feature fully complies with all applicable constitution principles. Shared infrastructure priority is followed by creating 5 foundational packages before domain features. UPDL and template systems are appropriately deferred to Phases 2-3 per the phased implementation plan.
