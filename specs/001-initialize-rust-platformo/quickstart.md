# Quick Start Guide: Universo Platformo Rust

**Last Updated**: 2025-11-17  
**Applies To**: Version 0.1.0 (Foundation Phase)

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** 1.75 or later (stable toolchain)
- **Cargo** (comes with Rust)
- **PostgreSQL** 14+ (for database)
- **Node.js** 18+ and **npm** (for Trunk installation)
- **Git** for version control

### Installing Rust

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --version
cargo --version
```

### Installing Additional Tools

```bash
# Install Trunk (for frontend builds)
cargo install trunk

# Install cargo-watch (for development hot-reload)
cargo install cargo-watch

# Install cargo-tarpaulin (for code coverage)
cargo install cargo-tarpaulin

# Install sqlx-cli (for database migrations)
cargo install sqlx-cli --no-default-features --features postgres
```

---

## Quick Start (5 minutes)

### 1. Clone the Repository

```bash
git clone https://github.com/teknokomo/universo-platformo-rust.git
cd universo-platformo-rust
```

### 2. Set Up Environment Variables

Create a `.env` file in the project root:

```bash
# Database
DATABASE_URL=postgresql://postgres:password@localhost:5432/universo_platformo

# Supabase (get these from your Supabase project)
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=your-anon-key
SUPABASE_SERVICE_KEY=your-service-key

# Server
HOST=127.0.0.1
PORT=8080

# Frontend
FRONTEND_URL=http://localhost:8081
```

### 3. Set Up Database

```bash
# Create database
createdb universo_platformo

# Run migrations (when available in Phase 2)
# sqlx migrate run
```

### 4. Build the Workspace

```bash
# Build all packages
cargo build --workspace

# This should succeed even in Foundation Phase
# Individual packages may not be functional yet
```

### 5. Run Tests

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture
```

---

## Development Workflow

### Working on Backend Packages

```bash
# Navigate to a backend package
cd packages/universo-types/base

# Build the package
cargo build

# Run tests
cargo test

# Run with hot-reload (when applicable)
cargo watch -x run
```

### Working on Frontend Packages

```bash
# Navigate to a frontend package (when available in Phase 2)
cd packages/clusters-frt/base

# Start development server with hot-reload
trunk serve --port 8081

# Build for production
trunk build --release
```

### Running the Full Stack

```bash
# Terminal 1: Backend server (Phase 2+)
cd packages/universo-api-srv/base
cargo watch -x run

# Terminal 2: Frontend dev server (Phase 2+)
cd packages/universo-web-frt/base
trunk serve --port 8081

# Access at: http://localhost:8081
```

---

## Project Structure Overview

```
universo-platformo-rust/
├── packages/                    # All packages (frontend, backend, shared)
│   ├── universo-types/         # Shared type definitions
│   ├── universo-utils/         # Common utilities
│   ├── universo-api-client/    # HTTP client
│   ├── universo-i18n/          # Internationalization
│   └── universo-ui-components/ # Yew UI components
├── specs/                       # Feature specifications
│   └── 001-initialize-rust-platformo/
│       ├── spec.md
│       ├── plan.md
│       ├── research.md
│       ├── data-model.md
│       ├── quickstart.md (this file)
│       └── contracts/
├── .github/                     # GitHub workflows and instructions
│   ├── instructions/
│   └── workflows/
├── Cargo.toml                   # Workspace configuration
├── rust-toolchain.toml          # Rust version specification
├── README.md                    # English documentation
└── README-RU.md                 # Russian documentation
```

---

## Common Commands

### Building

```bash
# Build entire workspace
cargo build --workspace

# Build specific package
cargo build -p universo-types

# Build for release (optimized)
cargo build --workspace --release
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific package
cargo test -p universo-types

# Run specific test
cargo test test_name

# Run tests with coverage
cargo tarpaulin --workspace --out Html
```

### Linting and Formatting

```bash
# Format code
cargo fmt --all

# Check formatting without modifying
cargo fmt --all -- --check

# Run Clippy (linter)
cargo clippy --workspace

# Clippy with strict mode (deny warnings)
cargo clippy --workspace -- -D warnings
```

### Documentation

```bash
# Generate and open documentation
cargo doc --workspace --open

# Generate docs without dependencies
cargo doc --workspace --no-deps
```

---

## IDE Setup

### Visual Studio Code

Recommended extensions:

- **rust-analyzer**: Rust language support
- **CodeLLDB**: Debugging support
- **crates**: Cargo.toml management
- **Better TOML**: TOML syntax highlighting

Settings (`.vscode/settings.json`):

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### IntelliJ IDEA / CLion

1. Install the Rust plugin
2. Open the project
3. Enable Cargo check on save
4. Configure rustfmt as default formatter

---

## Database Setup (Detailed)

### Using Docker for PostgreSQL

```bash
# Start PostgreSQL container
docker run --name universo-postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=universo_platformo \
  -p 5432:5432 \
  -d postgres:15

# Check if running
docker ps
```

### Using Local PostgreSQL

```bash
# Install PostgreSQL (Ubuntu/Debian)
sudo apt-get install postgresql postgresql-contrib

# Install PostgreSQL (macOS)
brew install postgresql@15

# Start PostgreSQL
sudo systemctl start postgresql  # Linux
brew services start postgresql@15  # macOS

# Create database and user
sudo -u postgres psql
```

In PostgreSQL shell:

```sql
CREATE DATABASE universo_platformo;
CREATE USER universo WITH ENCRYPTED PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE universo_platformo TO universo;
```

### Running Migrations

```bash
# Once migrations exist (Phase 2+)
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

---

## Troubleshooting

### Rust Toolchain Issues

```bash
# Update Rust to latest stable
rustup update stable

# Check current version
rustc --version

# If wrong version, set override
rustup override set stable
```

### Cargo Build Errors

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

### Database Connection Issues

```bash
# Test database connection
psql -U universo -d universo_platformo -h localhost

# Check if PostgreSQL is running
sudo systemctl status postgresql  # Linux
brew services list  # macOS
```

### WASM Build Issues

```bash
# Ensure WASM target is installed
rustup target add wasm32-unknown-unknown

# Reinstall Trunk if needed
cargo install trunk --force
```

---

## Next Steps

After completing the quick start:

1. **Read the Documentation**: Check `README.md` and `README-RU.md` for project overview
2. **Explore Specifications**: Review `specs/001-initialize-rust-platformo/spec.md`
3. **Understand Architecture**: Read `specs/001-initialize-rust-platformo/ARCHITECTURAL-COMPARISON.md`
4. **Check Constitution**: Review `.specify/memory/constitution.md` for project principles
5. **Create an Issue**: Follow `.github/instructions/github-issues.md` to start contributing

---

## Getting Help

- **GitHub Issues**: https://github.com/teknokomo/universo-platformo-rust/issues
- **Documentation**: Inline documentation via `cargo doc --open`
- **Community**: Check the repository Discussions tab

---

## Contributing

1. Read `.github/instructions/github-issues.md` for issue creation
2. Read `.github/instructions/github-pr.md` for pull request guidelines
3. Follow `.github/instructions/i18n-docs.md` for documentation changes
4. Ensure all tests pass and code is formatted before submitting PR

---

## Foundation Phase Status

**Current Status**: Foundation Phase (Phase 1)

The project is currently in the foundation setup phase. The following are available:

- ✅ Project structure and organization
- ✅ Cargo workspace configuration
- ✅ Shared infrastructure package definitions
- ✅ Documentation and specifications
- ✅ Development workflow guidelines

**Coming in Phase 2** (Clusters Feature):
- Backend API implementation
- Frontend application
- Database migrations
- Full CRUD operations
- UPDL system

For detailed phase planning, see `specs/001-initialize-rust-platformo/IMPLEMENTATION-ROADMAP.md`.

---

## License

This project is licensed under the terms specified in the repository LICENSE file.
