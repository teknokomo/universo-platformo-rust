# API Contracts Overview

**Feature**: 001-initialize-rust-platformo  
**Version**: 1.0.0  
**Date**: 2025-11-17

## Introduction

This directory contains API contract specifications for the Universo Platformo Rust backend. Each contract defines the request/response structure, authentication requirements, and examples for a specific API endpoint.

## Base URL

```
Production: https://api.universo.pro
Development: http://localhost:8080
```

## API Versioning

All API endpoints are versioned using URL path prefixes:

```
/api/v1/*    - Version 1 (current)
/api/v2/*    - Version 2 (future)
```

## Global Response Format

All API responses follow a standard envelope format:

### Success Response

```json
{
  "status": "success",
  "data": { /* response data */ }
}
```

### Error Response

```json
{
  "status": "error",
  "message": "Human-readable error message",
  "code": "ERROR_CODE",
  "details": { /* optional additional details */ }
}
```

## Authentication

Most endpoints require authentication via one of these methods:

### 1. Bearer Token (JWT)

```http
Authorization: Bearer <jwt_token>
```

### 2. Session Cookie

```http
Cookie: session_id=<session_token>
```

### 3. API Key (for service-to-service)

```http
X-API-Key: <api_key>
```

## Common HTTP Status Codes

| Status Code | Meaning | When Used |
|-------------|---------|-----------|
| 200 | OK | Successful request |
| 201 | Created | Resource successfully created |
| 204 | No Content | Successful deletion or update with no response body |
| 400 | Bad Request | Invalid request format or validation error |
| 401 | Unauthorized | Missing or invalid authentication |
| 403 | Forbidden | Authenticated but not authorized for this resource |
| 404 | Not Found | Resource does not exist |
| 409 | Conflict | Resource conflict (e.g., duplicate unique field) |
| 422 | Unprocessable Entity | Validation errors |
| 500 | Internal Server Error | Server-side error |
| 503 | Service Unavailable | Service temporarily unavailable |

## Common Request Headers

| Header | Required | Description |
|--------|----------|-------------|
| `Content-Type` | Yes (for POST/PUT/PATCH) | Must be `application/json` |
| `Authorization` | Yes (for protected endpoints) | Bearer token or API key |
| `Accept-Language` | No | Preferred language: `en` or `ru` (default: `en`) |
| `X-Request-ID` | No | Client-provided request ID for tracing |

## Common Response Headers

| Header | Description |
|--------|-------------|
| `Content-Type` | Always `application/json; charset=utf-8` |
| `X-Request-ID` | Request ID for tracing (echoed from request or generated) |
| `X-RateLimit-Limit` | Rate limit maximum requests |
| `X-RateLimit-Remaining` | Remaining requests in current window |
| `X-RateLimit-Reset` | Unix timestamp when rate limit resets |

## Pagination

List endpoints support pagination via query parameters:

```
GET /api/v1/clusters?page=1&page_size=50
```

### Parameters

| Parameter | Type | Default | Max | Description |
|-----------|------|---------|-----|-------------|
| `page` | integer | 1 | - | Page number (1-indexed) |
| `page_size` | integer | 50 | 100 | Items per page |

### Response

```json
{
  "status": "success",
  "data": {
    "data": [ /* array of items */ ],
    "page": 1,
    "page_size": 50,
    "total_count": 234,
    "total_pages": 5
  }
}
```

## Filtering and Sorting

List endpoints support filtering and sorting via query parameters:

```
GET /api/v1/clusters?filter[name]=example&sort=-created_at
```

### Filter Syntax

```
filter[field]=value          # Equals
filter[field][gte]=value     # Greater than or equal
filter[field][lte]=value     # Less than or equal
filter[field][contains]=val  # Contains substring
```

### Sort Syntax

```
sort=field          # Ascending
sort=-field         # Descending
sort=field1,-field2 # Multiple fields
```

## Error Codes

Standard error codes used across all endpoints:

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `BAD_REQUEST` | 400 | Invalid request format |
| `VALIDATION_ERROR` | 422 | Request validation failed |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `CONFLICT` | 409 | Resource conflict |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |
| `INTERNAL_SERVER_ERROR` | 500 | Server error |
| `SERVICE_UNAVAILABLE` | 503 | Service unavailable |

## Rate Limiting

API endpoints are rate-limited to prevent abuse:

- **Anonymous requests**: 60 requests per hour
- **Authenticated requests**: 1000 requests per hour
- **Burst limit**: 20 requests per second

Rate limit headers are included in all responses.

## Internationalization

Error messages and localized content support English and Russian:

- Set `Accept-Language: en` for English (default)
- Set `Accept-Language: ru` for Russian

## Available Contracts

### System Endpoints

- [Health Check](./health.md) - `GET /api/v1/health` - Service health status

### Future Endpoints (Phase 2+)

Additional contracts will be added as features are implemented:

- Clusters API (Phase 2)
- Metaverses API (Phase 2+)
- Spaces API (Phase 3)
- Templates API (Phase 3)
- Authentication API (Ongoing)

## Testing

All contracts can be tested using:

- **cURL**: Command-line HTTP client (examples in each contract)
- **Postman**: Import OpenAPI spec (to be generated)
- **HTTPie**: User-friendly HTTP client
- **Integration tests**: Rust tests in `tests/` directory

## OpenAPI Specification

A complete OpenAPI 3.0 specification will be generated from these contracts and code documentation. It will be available at:

```
GET /api/v1/openapi.json     # OpenAPI spec
GET /api/v1/docs             # Swagger UI
```

## Contract Maintenance

- Each contract is versioned independently
- Breaking changes require new API version (v2, v3, etc.)
- Non-breaking changes can be added to current version
- Deprecation notices must be provided 6 months before removal

## Support

For API support and questions:
- GitHub Issues: https://github.com/teknokomo/universo-platformo-rust/issues
- Documentation: https://docs.universo.pro (future)
