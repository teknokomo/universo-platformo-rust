# API Contract: Health Check

**Version**: 1.0.0  
**Status**: Foundation  
**Category**: System

## Endpoint

```
GET /api/v1/health
```

## Description

Health check endpoint for monitoring and load balancer health checks. Returns the service status and version information.

## Authentication

None required (public endpoint)

## Request

No request body or parameters.

## Response

### Success Response (200 OK)

```json
{
  "status": "success",
  "data": {
    "service": "universo-platformo-rust",
    "version": "0.1.0",
    "status": "healthy",
    "timestamp": "2025-11-17T09:26:00Z"
  }
}
```

### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `service` | string | Service name |
| `version` | string | Service version (semver) |
| `status` | string | Health status: "healthy", "degraded", "unhealthy" |
| `timestamp` | string | Current server timestamp (ISO 8601) |

### Error Responses

| Status Code | Description | Response Body |
|-------------|-------------|---------------|
| 503 | Service Unavailable | `{"status": "error", "message": "Service is starting up"}` |

## Example

### cURL

```bash
curl -X GET http://localhost:8080/api/v1/health
```

### Response

```json
{
  "status": "success",
  "data": {
    "service": "universo-platformo-rust",
    "version": "0.1.0",
    "status": "healthy",
    "timestamp": "2025-11-17T09:26:00.123Z"
  }
}
```

## Rust Types

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub service: String,
    pub version: String,
    pub status: HealthStatus,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}
```

## Notes

- This endpoint should be lightweight and fast (<10ms response time)
- Used by load balancers and monitoring systems
- Does not require authentication for accessibility
- Version number should match Cargo.toml version
