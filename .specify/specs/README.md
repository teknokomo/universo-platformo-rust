# Specifications Directory

This directory contains all feature specifications for the Universo Platformo Rust project.

## Structure

Each specification is organized in its own subdirectory with a numeric prefix:

```
.specify/specs/
├── 001-initialize-rust-platformo/
│   ├── spec.md              # Main specification document
│   ├── plan.md              # Implementation plan
│   ├── tasks.md             # Task breakdown
│   ├── data-model.md        # Data model (if applicable)
│   ├── research.md          # Research notes (if applicable)
│   ├── quickstart.md        # Quick start guide (if applicable)
│   ├── contracts/           # API contracts (if applicable)
│   └── checklists/          # Review checklists (if applicable)
├── 002-feature-name/
│   └── ...
└── README.md                # This file
```

## Naming Convention

- Directories MUST be named with a 3-digit numeric prefix followed by a descriptive name: `NNN-feature-name`
- The numeric prefix determines the feature order and is used for branch naming
- Multiple branches can reference the same specification by using the same numeric prefix

## Required Files

Each specification directory MUST contain:
- `spec.md` - The main specification document following `.specify/templates/spec-template.md`
- `plan.md` - Implementation plan following `.specify/templates/plan-template.md`
- `tasks.md` - Task breakdown following `.specify/templates/tasks-template.md`

## Optional Files

Specifications MAY include:
- `data-model.md` - Detailed data model and entity relationships
- `research.md` - Research notes and decision rationale
- `quickstart.md` - Quick start guide for testing
- `contracts/` - API contracts and interface definitions
- `checklists/` - Review and validation checklists

## Workflow

1. Create a new feature specification using `.specify/scripts/bash/create-new-feature.sh`
2. Fill in the specification following the templates in `.specify/templates/`
3. Run clarifications using `/speckit.clarify` to refine requirements
4. Generate implementation plan using `/speckit.plan`
5. Generate tasks using `/speckit.tasks`
6. Analyze for consistency using `/speckit.analyze`
7. Implement using `/speckit.implement`

## See Also

- `.specify/memory/constitution.md` - Project constitution and core principles
- `.specify/templates/` - Templates for specifications, plans, and tasks
- `.specify/scripts/` - Utility scripts for managing specifications
