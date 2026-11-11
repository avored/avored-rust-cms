# API Route Versioning

## Policy

All resource APIs use the `/api/v1/` prefix. Public API-key endpoints live directly under `/api/v1/`; JWT-protected browser administration endpoints live under `/api/v1/admin/`. Authentication/bootstrap endpoints (`/api/auth/login`, `/api/misc/setup`) and the operational health check (`/health`) are outside the resource API and are explicitly unversioned.

JWT-protected admin routes require `Authorization: Bearer <JWT>`. Public entity-record routes require `X-Api-Key`. The API-key administration endpoints are admin-only and require a JWT. Admin routes do not accept API keys as a substitute for JWTs.

## Canonical routes

| Method | Path | Audience | Authentication |
|---|---|---|---|
| GET, POST | `/api/v1/admin/collection` | Admin browser | JWT |
| GET, PUT, DELETE | `/api/v1/admin/collection/{record_id}` | Admin browser | JWT |
| GET, POST | `/api/v1/admin/entities` | Admin browser | JWT |
| GET, PUT, DELETE | `/api/v1/admin/entities/{id}` | Admin browser | JWT |
| GET | `/api/v1/admin/entities/options` | Admin browser | JWT |
| PUT | `/api/v1/admin/entities/{id}/identifier` | Admin browser | JWT |
| GET, POST | `/api/v1/admin/email-templates` | Admin browser | JWT |
| GET, PUT, DELETE | `/api/v1/admin/email-templates/{id}` | Admin browser | JWT |
| GET, POST | `/api/v1/admin/workflows` | Admin browser | JWT |
| GET, PUT, DELETE | `/api/v1/admin/workflows/{id}` | Admin browser | JWT |
| PATCH | `/api/v1/admin/workflows/{id}/active` | Admin browser | JWT |
| GET | `/api/v1/admin/workflows/{id}/executions` | Admin browser | JWT |
| GET, POST | `/api/v1/admin/api-keys` | Admin browser | JWT |
| GET, PUT, DELETE | `/api/v1/admin/api-keys/{id}` | Admin browser | JWT |
| GET, PUT | `/api/v1/admin/me` | Admin browser | JWT |
| PUT | `/api/v1/admin/me/password` | Admin browser | JWT |
| GET, PUT | `/api/v1/admin/settings` | Admin browser | JWT |
| GET, POST | `/api/v1/entities` | Public API | `X-Api-Key` |
| GET, PATCH, DELETE | `/api/v1/entities/{id}` | Public API | `X-Api-Key` |
| POST | `/api/auth/login` | Authentication | None |
| POST | `/api/misc/setup` | Initial setup | None |
| GET | `/health` | Operations | None |

## Compatibility

Admin routes are consumed by this repository's own browser client. Legacy unversioned admin paths and `/api/v1/settings` are removed; clients in this repository use the canonical routes above. No deprecation or sunset window is maintained.
