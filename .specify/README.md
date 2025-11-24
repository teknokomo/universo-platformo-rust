# Spec Kit Directory Structure

This directory contains the Spec Kit workflow infrastructure for managing feature specifications, documentation, and development workflows in the Universo Platformo Rust project.

## Directory Structure

```
.specify/
├── memory/                # Project memory and constitution
│   └── constitution.md    # Project constitution and core principles
├── scripts/               # Automation scripts
│   └── bash/             # Shell scripts for workflow automation
│       ├── check-prerequisites.sh    # Verify feature branch and spec files
│       ├── common.sh                 # Shared functions and variables
│       ├── create-new-feature.sh     # Create new feature branch and spec
│       ├── setup-plan.sh             # Initialize implementation plan
│       └── update-agent-context.sh   # Update agent context files
├── specs/                 # Feature specifications (organized by feature number)
│   └── NNN-feature-name/  # Individual feature directories
│       ├── spec.md        # Feature specification
│       ├── tasks.md       # Implementation tasks
│       ├── plan.md        # Implementation plan
│       ├── data-model.md  # Data model (optional)
│       ├── research.md    # Research notes (optional)
│       ├── quickstart.md  # Quick start guide (optional)
│       └── contracts/     # API contracts (optional)
└── templates/             # Document templates
    ├── spec-template.md       # Specification template
    ├── tasks-template.md      # Tasks template
    ├── plan-template.md       # Implementation plan template
    ├── checklist-template.md  # Checklist template
    └── agent-file-template.md # Agent file template
```

## Purpose

### memory/
Contains the project's **constitution** - the foundational principles, conventions, and rules that govern the project's architecture, development practices, and quality standards. The constitution is the ultimate authority for all development decisions.

Key file:
- **constitution.md**: Core principles including:
  - Monorepo structure with Cargo workspaces
  - Package structure conventions (NON-NEGOTIABLE)
  - Bilingual documentation requirements
  - Technology stack standards
  - Development workflow guidelines

### scripts/
Automation scripts that support the Spec Kit workflow. These scripts help create new features, validate project structure, and manage the specification lifecycle.

Key scripts:
- **create-new-feature.sh**: Creates a new feature branch and initializes spec directory
- **check-prerequisites.sh**: Validates feature branch setup and available documents
- **setup-plan.sh**: Initializes implementation planning phase
- **update-agent-context.sh**: Updates agent configuration with project context

### specs/
Feature specifications organized by feature number (NNN-feature-name). Each feature has its own directory containing all related documentation.

Directory naming convention: `NNN-feature-name` where:
- `NNN` is a zero-padded three-digit number (e.g., 001, 002, 003)
- `feature-name` is a short, descriptive name in kebab-case

Each feature directory typically contains:
- **spec.md**: Complete feature specification with user stories, requirements, and acceptance criteria
- **tasks.md**: Ordered list of implementation tasks
- **plan.md**: Technical implementation plan with architecture decisions
- **data-model.md**: Data structures and entity definitions (if applicable)
- **research.md**: Research notes and technical investigations (optional)
- **contracts/**: API contract definitions (optional)

### templates/
Standard templates for creating new documentation. These ensure consistency across all feature specifications and documentation.

Available templates:
- **spec-template.md**: Feature specification structure
- **tasks-template.md**: Task list organization
- **plan-template.md**: Implementation plan structure
- **checklist-template.md**: Checklist format
- **agent-file-template.md**: AI agent configuration

## Workflow Integration

The `.specify` directory integrates with GitHub Copilot agents in `.github/agents/`:
- **speckit.specify**: Create new feature specifications
- **speckit.clarify**: Clarify ambiguities in specifications
- **speckit.plan**: Generate implementation plans
- **speckit.tasks**: Create task lists from specifications
- **speckit.analyze**: Analyze consistency across artifacts
- **speckit.implement**: Execute implementation tasks
- **speckit.constitution**: Update project constitution

## Usage

### Creating a New Feature

```bash
# From repository root
.specify/scripts/bash/create-new-feature.sh "Add user authentication system"
```

This will:
1. Create a new feature branch (e.g., `001-user-auth-system`)
2. Create spec directory at `.specify/specs/001-user-auth-system/`
3. Initialize `spec.md` from template

### Working with an Existing Feature

The scripts automatically detect the current feature branch and locate the corresponding spec directory.

```bash
# Check prerequisites for current feature
.specify/scripts/bash/check-prerequisites.sh

# Setup implementation plan
.specify/scripts/bash/setup-plan.sh
```

## Agent Access

GitHub Copilot agents have read access to the `.specify` directory and can:
- Read specifications, plans, and tasks
- Access the constitution for validation
- Use templates for generating new documents
- Execute workflow scripts

The **tasks** and **analytics** agents specifically use `.specify` content to:
- Generate implementation tasks based on specifications
- Analyze consistency across specification artifacts
- Validate against constitutional principles
- Track feature progress and completeness

## Relationship to Root `specs/` (Legacy)

**NOTE**: The legacy `specs/` directory in the repository root has been migrated to `.specify/specs/`. All new features should be created in `.specify/specs/`.

## Best Practices

1. **Always start with a specification**: Use `/speckit.specify` to create a new feature spec before coding
2. **Follow the workflow**: Specify → Clarify → Plan → Tasks → Implement → Analyze
3. **Reference the constitution**: All decisions must align with constitutional principles
4. **Keep specs updated**: Update specifications when requirements change
5. **Document decisions**: Use research.md to capture technical decisions and trade-offs
6. **Maintain consistency**: Follow templates and naming conventions

## Migration Notes

This structure follows the Spec Kit pattern and consolidates all specification-related files under `.specify/`:
- ✅ Memory (constitution) → `.specify/memory/`
- ✅ Specs → `.specify/specs/`
- ✅ Templates → `.specify/templates/`
- ✅ Scripts → `.specify/scripts/`

The organization ensures:
- Clear separation of concerns
- Easy access for agents
- Version control of all specification artifacts
- Consistent workflow across features
