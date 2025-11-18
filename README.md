# Important! Grave! Важно!

-   Workers of the world, unite!
-   Proletoj el ĉiuj landoj, unuiĝu!
-   Пролетарии всех стран, соединяйтесь!

# Universo Platformo Rust

[![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue)](https://github.com/teknokomo/universo-platformo-rust)
[![License: Omsk Open License](https://img.shields.io/badge/license-Omsk%20Open%20License-green)](LICENSE.md)

## Basic Information

Implementation of Universo Platformo / Universo MMOOMM / Universo Kiberplano built on Actix Web, Yew, and the Rust ecosystem. This project is a Rust-native reimplementation of the concepts from [Universo Platformo React](https://github.com/teknokomo/universo-platformo-react), focusing on performance, safety, and WebAssembly-powered frontend capabilities.

**In this repository, public efforts are currently underway to create Universo Platformo / Universo MMOOMM, in order to launch a global teknokomization and save humanity from final enslavement and complete destruction by creating special mass multi-user virtual worlds, such as Universo MMOOMM, and a platform for their creation - Universo Platformo, initially with gaming functionality, and then with the addition of the Cyberplan functionality.**

Universo Platformo Rust serves as the foundation for implementing **Universo Kiberplano** - a global planning and implementation system that unifies plans, tasks, and resources while controlling robots. This Rust implementation prioritizes type safety, performance, and reliability for mission-critical operations.

More details about all this are written in "The Book of The Future" and various other materials of ours, most of which are still poorly structured and not in English, but right now work is underway to create new detailed documentation, which will be presented in many languages.

## Inspiration

Our wonderful project, which will help create a global teknokomization and save humanity from final enslavement and total destruction, is expanding to the Rust ecosystem. We are implementing a Rust-based version of Universo Platformo that will serve as a high-performance, type-safe foundation for creating interactive 3D/AR/VR experiences.

This implementation focuses on leveraging Rust's safety guarantees and performance characteristics to create a robust platform for building cross-platform 3D applications through a visual node-based interface compiled to WebAssembly.

## Where Am I and What Should I Do?

The near future, Omsk is the capital of the world, in the Olympus-1 tower, scientists explain to you that it is possible to connect your consciousness to a robot in another part of the Universe, in a parallel reality, controlled by robots we call Robocubans, through the recently discovered Great Ring system.

In Universo Platformo Rust, you are at the control panel of this revolutionary technology, now powered by the safety and performance of Rust. Through the visual node editor compiled to WebAssembly, you can create interactive 3D scenes, AR experiences, and VR worlds that bridge our reality with parallel universes.

Your mission is to help build and expand this platform using Rust's powerful ecosystem, creating new exporters, enhancing the node system, and contributing to the publication mechanism that will allow these experiences to be shared across the multiverse with unprecedented reliability and performance.

## Contact Information

For questions or collaboration, please contact:

-   VK: [https://vk.com/vladimirlevadnij](https://vk.com/vladimirlevadnij)
-   Telegram: [https://t.me/Vladimir_Levadnij](https://t.me/Vladimir_Levadnij)
-   Email: [universo.pro@yandex.com](mailto:universo.pro@yandex.com)

Our website: [https://universo.pro](https://universo.pro)

## Overview

Universo Platformo Rust is a project that reimplements the core concepts from Universo Platformo React using:

-   **Rust + WebAssembly** for high-performance, type-safe frontend (via Yew)
-   **Actix Web** for a fast, concurrent backend
-   **Supabase integration** for multi-user functionality with abstraction for future DBMS support
-   **Monorepo structure** using Cargo workspaces
-   **Package-based architecture** with clear frontend/backend separation

The project aims to create a unified platform for developing interactive 3D applications that can be exported to various technologies including AR.js, PlayCanvas, Babylon.js, Three.js, and A-Frame, all powered by Rust's performance and safety guarantees.

## Relationship to Universo Platformo React

This repository implements the same conceptual foundation as [Universo Platformo React](https://github.com/teknokomo/universo-platformo-react) but using the Rust technology stack:

**Conceptual Equivalents:**
-   React → Yew (WebAssembly-based frontend framework)
-   Express → Actix Web (high-performance web framework)
-   PNPM workspaces → Cargo workspaces
-   JavaScript/TypeScript → Rust
-   Passport.js → Rust-native authentication solutions

**Important:** This is NOT a direct port. We take the proven concepts from the React implementation and apply Rust ecosystem best practices, avoiding any architectural issues or incomplete features from the React version.

## Current Status

**Current Phase**: 0.1.0-alpha (November 2025)

**Primary Focus:**

-   Repository initialization and structure setup
-   Cargo workspace configuration
-   Package structure template (frontend/backend separation)
-   Documentation establishment (bilingual English/Russian)
-   First feature implementation: Clusters (Clusters/Domains/Resources entities)

## Tech Stack

**Frontend:**
-   Rust (stable)
-   Yew (WebAssembly framework)
-   Material Design components (Rust ecosystem equivalents)
-   Build: Trunk for development, wasm-pack for library builds
-   State Management: Yewdux for global state, use_reducer for local state

**Backend:**
-   Rust (stable)
-   Actix Web
-   Supabase (primary database)
-   Database abstraction layer for future DBMS support
-   SQLx for type-safe database queries
-   actix-web-httpauth for authentication

**Development:**
-   Cargo workspaces (monorepo management)
-   rustfmt (code formatting)
-   clippy (linting)
-   cargo-watch (development automation and hot reload)
-   wasm-bindgen-test (frontend testing)
-   actix-web::test (backend testing)

**Inter-Package Communication:**
-   Shared types in `universo-types` crate with serde traits
-   Compile-time type safety for frontend-backend communication
-   HTTP client in `universo-api-client` crate using reqwest
-   Repository pattern with traits for database abstraction

## Best Practices for Rust Fullstack

This project follows modern Rust fullstack development patterns. For detailed examples and guidance, see:

- **Constitution Principle VII**: Complete Rust ecosystem best practices ([.specify/memory/constitution.md](./.specify/memory/constitution.md))
- **Architectural Comparison**: Rust technology stack integration guide ([specs/001-initialize-rust-platformo/ARCHITECTURAL-COMPARISON.md](./specs/001-initialize-rust-platformo/ARCHITECTURAL-COMPARISON.md))

**Key Patterns**:

1. **Frontend (Yew)**: Function components with hooks (use_state, use_reducer), typed props with Properties trait, HTML macro for templates
2. **Backend (Actix Web)**: Async handlers, typed extractors (Json<T>, Path<T>), middleware via .wrap()
3. **Shared Types**: All API contracts and entities in universo-types crate with Serialize/Deserialize traits
4. **Database**: Repository trait pattern with async_trait, SQLx for compile-time query verification
5. **Build Tools**: Trunk for frontend dev server with hot reload, cargo-watch for backend
6. **Testing**: wasm-bindgen-test for Yew components, actix-web::test for API endpoints

**Learning from React Implementation**:
- Adopt proven shared infrastructure packages (types, utils, api-client, i18n, ui-components)
- Follow -frt/-srv naming convention with base/ subdirectories
- Leverage Rust's type system for better compile-time guarantees than TypeScript
- Use Cargo workspace.dependencies for centralized version management

## Project Structure

**CRITICAL: Modular Architecture (NON-NEGOTIABLE)**

ALL application functionality MUST be implemented as packages in the `packages/` directory. This is a fundamental architectural requirement that cannot be violated.

**What MUST be in packages/**:
- ✅ All feature implementations (frontend and backend)
- ✅ All business logic and application code
- ✅ All UI components
- ✅ All API endpoints
- ✅ Shared infrastructure (types, utils, i18n, UI components)

**What is allowed in repository root**:
- ✅ Cargo.toml (workspace configuration)
- ✅ rust-toolchain.toml (toolchain specification)
- ✅ .gitignore, .github/ (repository configuration)
- ✅ .specify/ (specification workflow)
- ✅ README files and LICENSE

**What is PROHIBITED in repository root**:
- ❌ Application source code (NO src/ directory)
- ❌ Feature implementations
- ❌ Business logic
- ❌ UI components
- ❌ API endpoints

This modular structure ensures that:
1. Packages can be extracted to separate repositories in the future
2. Teams can work independently on different packages
3. Deployment can be done per-package
4. Code boundaries are clear and enforceable

```
universo-platformo-rust/
├── packages/                  # ALL APPLICATION CODE MUST BE HERE
│   ├── universo-types/        # Shared type definitions (PRIORITY: Create first)
│   │   └── base/
│   ├── universo-utils/        # Common utilities (PRIORITY: Create first)
│   │   └── base/
│   ├── universo-api-client/   # HTTP client (PRIORITY: Create first)
│   │   └── base/
│   ├── universo-i18n/         # Internationalization (PRIORITY: Create first)
│   │   └── base/
│   ├── universo-ui-components/# Shared UI components (PRIORITY: Create first)
│   │   └── base/
│   ├── clusters-frt/          # Clusters frontend (Phase 2)
│   │   └── base/              # Primary Rust/Yew implementation
│   ├── clusters-srv/          # Clusters backend (Phase 2)
│   │   └── base/              # Primary Rust/Actix implementation
│   ├── [feature]-frt/         # Other frontend packages
│   └── [feature]-srv/         # Other backend packages
├── specs/                     # Feature specifications
│   └── 001-initialize-rust-platformo/
├── .specify/                  # Specification workflow tools
│   ├── memory/                # Constitution and project memory
│   ├── scripts/               # Automation scripts
│   └── templates/             # Document templates
├── .github/
│   └── instructions/          # Development guidelines
│       ├── github-issues.md   # Issue creation standards
│       ├── github-labels.md   # Label conventions
│       ├── github-pr.md       # Pull request guidelines
│       └── i18n-docs.md       # Bilingual documentation rules
├── Cargo.toml                 # Workspace configuration (NO application code)
└── README.md                  # This file
```

This structure allows for:

-   **Mandatory Modularity**: Each feature MUST be contained within its own package(s)
-   **Clear Separation**: Frontend and backend packages are distinct (-frt/-srv suffixes)
-   **Scalability**: New features MUST be added as new packages in packages/
-   **Future Repository Extraction**: Packages are designed to become separate repositories
-   **Future-Proofing**: `base/` subdirectories allow for alternative implementations

## Key Features

### Three-Entity Pattern

Many features in Universo Platformo follow a three-entity hierarchical pattern:

-   **Clusters**: Clusters / Domains / Resources
-   **Metaverses**: Metaverses / Sections / Entities
-   **Uniks**: (Extended entity structure)

This pattern provides a consistent approach to organizing hierarchical data and relationships across different feature domains.

### Spaces and Canvases

Future development will include the Spaces/Canvases functionality for creating visual node-based workflows:

-   LangChain graph nodes integration
-   UPDL (Universal Platform Description Language) nodes
-   Visual workflow editor
-   Cross-platform export capabilities

## Development Workflow

1. **Planning Phase:**
   - Create GitHub Issue (following `.github/instructions/github-issues.md`)
   - Write feature specification in `specs/[###-feature-name]/spec.md`
   - Get specification approval

2. **Implementation Phase:**
   - Create feature branch named `###-feature-name`
   - Implement according to specification
   - Write tests alongside code
   - Update documentation (English first, then Russian)

3. **Quality Gates:**
   - All code must pass `cargo clippy` with no warnings
   - All code must be formatted with `rustfmt`
   - All tests must pass with `cargo test`
   - Documentation must be bilingual with identical structure

4. **Review & Merge:**
   - Create Pull Request (following `.github/instructions/github-pr.md`)
   - PR must reference the corresponding Issue
   - PR must include both English and Russian descriptions
   - PR must pass all CI checks before merge

## Getting Started

### Prerequisites

-   Rust (stable) - Install via [rustup](https://rustup.rs/)
-   Cargo (comes with Rust)
-   Node.js (for some build tools)
-   Supabase account (for database features)

### Installation

```bash
# Clone the repository
git clone https://github.com/teknokomo/universo-platformo-rust.git
cd universo-platformo-rust

# Build all packages
cargo build

# Run tests
cargo test

# Run with hot-reload during development
cargo watch -x run
```

### Creating a New Feature Package

When creating a new feature that requires both frontend and backend:

1. Create two packages in `packages/`:
   ```bash
   mkdir -p packages/myfeature-frt/base
   mkdir -p packages/myfeature-srv/base
   ```

2. Add them to the workspace in `Cargo.toml`

3. Place your Rust code in the respective `base/` directories

4. Follow the specification-first approach documented in `.specify/`

## Documentation

All documentation must be provided in both English and Russian:

-   **English** is the primary standard (created first)
-   **Russian** translation must have identical structure and line count
-   Pattern: `README.md` (English) and `README-RU.md` (Russian)

See `.github/instructions/i18n-docs.md` for detailed bilingual documentation guidelines.

## Contributing

We welcome contributions! Please follow these guidelines:

1. Read the constitution in `.specify/memory/constitution.md`
2. Create issues using `.github/instructions/github-issues.md`
3. Follow the specification-first workflow
4. Ensure bilingual documentation
5. Pass all quality gates (clippy, rustfmt, tests)

## License

This project is licensed under the Omsk Open License - see the LICENSE.md file for details.

## Related Projects

-   [Universo Platformo React](https://github.com/teknokomo/universo-platformo-react) - Original implementation using React/Express
-   More Universo projects coming soon across different technology stacks

---

**Together we build the future! // Вместе мы строим будущее!**
