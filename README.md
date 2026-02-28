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
-   **Supabase integration** for authentication, accessed exclusively through the backend
-   **Monorepo structure** using Cargo workspaces
-   **Package-based architecture** with clear frontend/backend separation

The project aims to create a unified platform for developing interactive 3D applications that can be exported to various technologies including AR.js, PlayCanvas, Babylon.js, Three.js, and A-Frame, all powered by Rust's performance and safety guarantees.

## Security Architecture

**Supabase is accessed exclusively through the backend.** The frontend never communicates with Supabase directly — all authentication and data operations go through the Actix Web backend:

```
Browser (Yew/WASM) ──► Actix Web Backend ──► Supabase
       │                      │
       └── session cookie     └── JWT (server-side only)
```

This design ensures:

-   The Supabase JWT is never exposed to browser JavaScript
-   All authentication logic is enforced server-side
-   Session data is stored in encrypted, HttpOnly, SameSite=Lax cookies
-   The Supabase `anon_key` remains on the server only

## Relationship to Universo Platformo React

This repository implements the same conceptual foundation as [Universo Platformo React](https://github.com/teknokomo/universo-platformo-react) but using the Rust technology stack:

**Conceptual Equivalents:**
-   React → Yew (WebAssembly-based frontend framework)
-   Express → Actix Web (high-performance web framework)
-   PNPM workspaces → Cargo workspaces
-   JavaScript/TypeScript → Rust
-   Supabase JS client → reqwest HTTP client (server-side only, never in the browser)

**Important:** This is NOT a direct port. We take the proven concepts from the React implementation and apply Rust ecosystem best practices, avoiding any architectural issues or incomplete features from the React version.

## Current Status

**Current Phase**: 0.1.0-alpha (February 2026)

**Implemented:**

-   Repository structure and Cargo workspace configuration
-   Start page for unauthenticated users: hero, testimonials, login modal (`packages/start-frt`)
-   Start page for authenticated users: three-step onboarding wizard (`packages/start-frt`)
-   Supabase authentication backend with encrypted session cookies (`packages/start-srv`)
-   Onboarding API: Projects / Campaigns / Clusters item selection and completion tracking

**Planned Next:**

-   Clusters feature (Clusters / Domains / Resources entities)
-   Shared types package (`universo-types`)
-   Metaverses feature (Metaverses / Sections / Entities)

## Tech Stack

**Frontend (start-frt):**
-   Rust (stable) compiled to WebAssembly
-   Yew 0.21 (component-based UI framework)
-   Yew Router 0.18 (client-side routing)
-   gloo-net (HTTP fetch API with credentials support for WASM)
-   wasm-bindgen / wasm-bindgen-futures (JavaScript interop)
-   Trunk (development server with hot reload and API proxy)

**Backend (start-srv):**
-   Rust (stable)
-   Actix Web 4 (async HTTP server framework)
-   actix-session 0.9 with CookieSessionStore (encrypted server-side sessions)
-   actix-cors 0.7 (configurable CORS middleware)
-   reqwest 0.12 (async HTTP client for Supabase REST API calls)
-   dotenvy (environment variable loading from .env)

**Authentication Flow:**
1. Browser → `POST /api/v1/auth/login` → Backend → Supabase `/auth/v1/token`
2. Backend stores JWT in encrypted, HttpOnly, SameSite=Lax session cookie
3. Browser → `GET /api/v1/auth/me` → Backend → Supabase `/auth/v1/user`
4. Browser → `POST /api/v1/auth/logout` → Backend → Supabase `/auth/v1/logout` + purge session

**Development Tools:**
-   Cargo workspaces (monorepo management)
-   rustfmt (code formatting)
-   clippy (linting)
-   wasm-bindgen-test (WASM unit tests)
-   actix-web::test (backend integration tests)

## Project Structure

**CRITICAL: Modular Architecture (NON-NEGOTIABLE)**

ALL application functionality MUST be implemented as packages in the `packages/` directory. This is a fundamental architectural requirement that cannot be violated.

```
universo-platformo-rust/
├── packages/                  # ALL APPLICATION CODE MUST BE HERE
│   ├── start-frt/             # Start page frontend package
│   │   └── base/
│   │       ├── src/
│   │       │   ├── api/       # HTTP client (gloo-net, fetch with credentials)
│   │       │   ├── auth/      # AuthContext, AuthProvider, use_reducer state
│   │       │   ├── components/# Hero, LoginForm, OnboardingWizard, Footer, etc.
│   │       │   ├── pages/     # StartPage, GuestStartPage, AuthenticatedStartPage
│   │       │   ├── app.rs     # Root App component with Yew Router
│   │       │   └── lib.rs     # WASM entry point (wasm_bindgen start)
│   │       ├── index.html     # HTML entry point for Trunk
│   │       ├── styles.css     # Dark theme stylesheet
│   │       └── Trunk.toml     # Trunk config with API proxy to backend
│   └── start-srv/             # Start page backend package
│       └── base/
│           ├── src/
│           │   ├── handlers/  # auth.rs (login/logout/me), onboarding.rs
│           │   ├── config.rs  # AppConfig loaded from environment variables
│           │   ├── error.rs   # AppError enum with HTTP response conversion
│           │   ├── models.rs  # Shared request/response data types
│           │   ├── supabase.rs# Supabase REST API client (sign_in/get_user/sign_out)
│           │   └── main.rs    # HttpServer setup, routing, middleware stack
│           └── .env.example   # Environment variable documentation template
├── Cargo.toml                 # Workspace definition and shared dependencies
└── README.md                  # This file
```

**Architecture Constraints:**
- ✅ All feature code lives in `packages/`
- ✅ Frontend only calls `/api/v1/*` endpoints on the backend
- ✅ Backend is the sole client of the Supabase REST API
- ❌ No direct Supabase calls from the frontend (browser never touches Supabase)
- ❌ No application code in the repository root (no `src/` directory)

## Key Features

### Authentication and Start Page

The first implemented feature provides a complete authentication cycle:

-   **Guest landing page**: Hero section, testimonials, "Sign In" / "Get Started" buttons
-   **Login modal**: Email/password form calling the backend auth endpoint
-   **Session management**: JWT stored server-side in encrypted HttpOnly cookie
-   **Authenticated dashboard**: Three-step onboarding wizard displayed after login
-   **Session restoration**: On every load, `GET /api/v1/auth/me` restores the session
-   **Logout**: Revokes Supabase token and purges the server-side session

### Three-Entity Onboarding Wizard

The onboarding wizard follows the three-entity hierarchical pattern used across Universo Platformo:

-   **Step 1 — Projects** (Global Goals): Sustainability and development initiatives
-   **Step 2 — Campaigns** (Personal Interests): Education, healthcare, environment topics
-   **Step 3 — Clusters** (Platform Features): 3D worlds, AR experiences, VR simulations
-   **Completion screen**: Shown after wizard submission, with option to update preferences

### Planned Future Features

-   **Clusters**: Full CRUD for Clusters / Domains / Resources entities
-   **Metaverses**: Metaverses / Sections / Entities management
-   **Spaces/Canvases**: Visual node-based workflow editor compiled to WebAssembly
-   **Shared packages**: `universo-types`, `universo-utils`, `universo-api-client`

## Development Workflow

1. **Planning Phase:**
   - Create GitHub Issue (following `.github/instructions/github-issues.md`)
   - Write feature specification in `.specify/specs/[###-feature-name]/spec.md`
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

-   Rust (stable) — Install via [rustup](https://rustup.rs/)
-   Trunk — `cargo install trunk`
-   wasm32 target — `rustup target add wasm32-unknown-unknown`
-   Supabase account — [supabase.com](https://supabase.com) with an existing user account

### Running the Backend

```bash
# Go to the backend package directory
cd packages/start-srv/base

# Copy the environment template and fill in your values
cp .env.example .env
# Edit .env: set SUPABASE_URL and SUPABASE_ANON_KEY

# Start the Actix Web server on http://127.0.0.1:8080
cargo run
```

### Running the Frontend

```bash
# Go to the frontend package directory
cd packages/start-frt/base

# Start the Trunk development server (proxies /api requests to localhost:8080)
trunk serve
```

Open `http://127.0.0.1:8080` in your browser. The backend and Trunk server must both be running.

### Building for Production

```bash
# Build the backend binary
cargo build --release -p start-srv

# Build the frontend WASM bundle
cd packages/start-frt/base && trunk build --release
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

This project is licensed under the Omsk Open License — see the LICENSE.md file for details.

## Related Projects

-   [Universo Platformo React](https://github.com/teknokomo/universo-platformo-react) — Original implementation using React/Express
-   More Universo projects coming soon across different technology stacks

---

**Together we build the future! // Вместе мы строим будущее!**
