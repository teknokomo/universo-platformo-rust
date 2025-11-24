# Data Model: Initialize Universo Platformo Rust Project

**Feature**: 001-initialize-rust-platformo  
**Date**: 2025-11-17  
**Status**: Foundation Phase

## Overview

This document defines the data models for the foundational shared infrastructure packages. Since this is the initialization phase, we focus on the structure of shared type packages and their organization. Domain-specific models (Clusters, Metaverses) will be added in subsequent phases.

## Shared Type Packages

### universo-types Package Structure

The `universo-types` package serves as the central repository for all type definitions used across the platform.

#### Core Type Categories

1. **API Types** (`api/` module)
2. **Entity Types** (`entities/` module)  
3. **UPDL Types** (`updl/` module) - Phase 2
4. **Common Types** (`common/` module)

---

## 1. API Types

Located in `packages/universo-types/base/src/api/`

### Pagination

Used across all list endpoints for consistent pagination behavior.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 50,
        }
    }
}

impl Pagination {
    pub const MAX_PAGE_SIZE: u32 = 100;
    
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.page == 0 {
            return Err("Page must be >= 1".to_string());
        }
        if self.page_size == 0 || self.page_size > Self::MAX_PAGE_SIZE {
            return Err(format!("Page size must be 1-{}", Self::MAX_PAGE_SIZE));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total_count: u32,
    pub total_pages: u32,
}
```

### API Response Envelope

Standard response wrapper for all API endpoints.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ApiResponse<T> {
    #[serde(rename = "success")]
    Success { data: T },
    
    #[serde(rename = "error")]
    Error { 
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse::Success { data }
    }
    
    pub fn error(message: impl Into<String>) -> Self {
        ApiResponse::Error {
            message: message.into(),
            code: None,
            details: None,
        }
    }
    
    pub fn error_with_code(message: impl Into<String>, code: impl Into<String>) -> Self {
        ApiResponse::Error {
            message: message.into(),
            code: Some(code.into()),
            details: None,
        }
    }
}
```

### API Error Types

```rust
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiErrorCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    ValidationError,
    InternalServerError,
    ServiceUnavailable,
}

impl fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiErrorCode::BadRequest => write!(f, "BAD_REQUEST"),
            ApiErrorCode::Unauthorized => write!(f, "UNAUTHORIZED"),
            ApiErrorCode::Forbidden => write!(f, "FORBIDDEN"),
            ApiErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ApiErrorCode::Conflict => write!(f, "CONFLICT"),
            ApiErrorCode::ValidationError => write!(f, "VALIDATION_ERROR"),
            ApiErrorCode::InternalServerError => write!(f, "INTERNAL_SERVER_ERROR"),
            ApiErrorCode::ServiceUnavailable => write!(f, "SERVICE_UNAVAILABLE"),
        }
    }
}
```

---

## 2. Entity Types

Located in `packages/universo-types/base/src/entities/`

### Common Entity Traits

All entities share common fields and behaviors.

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common fields for all entities with timestamps
pub trait Timestamped {
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
}

/// Common fields for soft-deletable entities
pub trait SoftDeletable {
    fn deleted_at(&self) -> Option<DateTime<Utc>>;
    fn is_deleted(&self) -> bool {
        self.deleted_at().is_some()
    }
}

/// Common fields for entities with ownership
pub trait Owned {
    fn owner_id(&self) -> Uuid;
}
```

### Base Entity Structure

Template for domain entities (to be used in Phase 2 for Clusters, etc.)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Timestamped for BaseEntity {
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl SoftDeletable for BaseEntity {
    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
}
```

---

## 3. Common Types

Located in `packages/universo-types/base/src/common/`

### User Identity

Basic user identity structure for authentication context.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdentity {
    pub user_id: Uuid,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
```

### Language

Supported languages for internationalization.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "ru")]
    Russian,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Russian => "ru",
        }
    }
    
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Language::English),
            "ru" => Some(Language::Russian),
            _ => None,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}
```

### Validation Result

Standard validation result type.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub type ValidationResult<T> = Result<T, Vec<ValidationError>>;
```

---

## 4. UPDL Types (Phase 2)

Located in `packages/universo-types/base/src/updl/`

**Note**: These types will be fully defined in Phase 2 when implementing the Clusters feature and UPDL processor. Placeholder structure provided here for reference.

### UPDL Node Base

```rust
// Placeholder - to be expanded in Phase 2
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UPDLNode {
    pub id: String,
    pub node_type: String,
    pub properties: Value,
    pub connections: Vec<NodeConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnection {
    pub source_port: String,
    pub target_node: String,
    pub target_port: String,
}

// Additional UPDL node types (Scene, Entity, Transform, etc.) 
// will be added in Phase 2
```

---

## Package Organization

### universo-types/base/src/lib.rs

```rust
//! Universo Platform Shared Types
//! 
//! This crate provides all shared type definitions used across the Universo
//! Platformo Rust project, including API contracts, entity definitions, and
//! UPDL structures.

pub mod api;
pub mod entities;
pub mod common;
pub mod updl;

// Re-export commonly used types
pub use api::{ApiResponse, Pagination, PaginatedResponse};
pub use common::{Language, UserIdentity, ValidationError, ValidationResult};
pub use entities::{Timestamped, SoftDeletable, Owned, BaseEntity};
```

### Cargo.toml Dependencies

```toml
[package]
name = "universo-types"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.6", features = ["serde", "v4"] }
chrono = { version = "0.4", features = ["serde"] }
```

---

## Database Schema Considerations

While database schemas are implementation details, the entity types inform schema design:

### Naming Conventions
- Table names: snake_case, plural (e.g., `clusters`, `domains`, `resources`)
- Column names: snake_case (e.g., `created_at`, `owner_id`)
- Primary keys: `id` (UUID)
- Foreign keys: `{entity}_id` (e.g., `cluster_id`, `domain_id`)

### Common Columns
All tables include:
- `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`
- `created_at TIMESTAMPTZ DEFAULT NOW()`
- `updated_at TIMESTAMPTZ DEFAULT NOW()`
- `deleted_at TIMESTAMPTZ` (for soft delete)

### Indexes
- Primary key on `id`
- Index on `created_at` for chronological queries
- Index on `deleted_at` for filtering active records
- Foreign key indexes for relationships

---

## Validation Rules

All types implement validation where appropriate:

### API Request Validation
- Required fields must be present
- String lengths within acceptable ranges
- Numeric values within valid ranges
- Email format validation
- UUID format validation

### Business Logic Validation
- Uniqueness constraints (e.g., email uniqueness)
- Referential integrity (foreign key relationships)
- State transitions (e.g., cannot delete non-existent entity)

### Example Validation Implementation

```rust
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateClusterRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,
}
```

---

## Type Safety Guarantees

### Compile-Time Guarantees
- Serde ensures serialization/deserialization correctness
- Type system prevents invalid state representation
- Generic types enable reusable patterns

### Runtime Guarantees
- Validation methods prevent invalid data from entering system
- Error types provide structured error handling
- Option types make null safety explicit

---

## Testing Strategy

### Unit Tests
Each type module includes tests for:
- Serialization/deserialization round-trips
- Validation logic correctness
- Default value behavior
- Edge cases (empty strings, boundary values)

### Example Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_offset() {
        let page1 = Pagination { page: 1, page_size: 10 };
        assert_eq!(page1.offset(), 0);
        
        let page2 = Pagination { page: 2, page_size: 10 };
        assert_eq!(page2.offset(), 10);
    }

    #[test]
    fn test_pagination_validation() {
        let invalid = Pagination { page: 0, page_size: 10 };
        assert!(invalid.validate().is_err());
        
        let valid = Pagination { page: 1, page_size: 50 };
        assert!(valid.validate().is_ok());
    }
}
```

---

## Future Considerations

### Phase 2 Additions (Clusters Feature)
- Cluster, Domain, Resource entity types
- CRUD request/response types for Clusters API
- Full UPDL node type definitions
- Flow graph structures

### Phase 3 Additions (Template System)
- Template configuration types
- Export format types
- AR.js and PlayCanvas specific types

### Scalability
- Types designed for evolution without breaking changes
- Versioning strategy via serde field attributes
- Backward compatibility with `#[serde(default)]` where appropriate

---

## Summary

The data model establishes a solid foundation with shared types for API contracts, entity management, and common utilities. All types use serde for serialization, include comprehensive validation, and follow Rust best practices for type safety. Domain-specific models will build on these foundations in subsequent phases.
