# 20 — Technical Architecture [OPTIONAL]

> Stack choices, infrastructure, API design, and deployment.
> Include this when the spec needs to inform or constrain implementation decisions.
>
> **Dependencies**: [02-data-model.md](02-data-model.md), [09-external-interactions.md](09-external-interactions.md)

---

## Stack

| Layer | Technology | Version | Rationale |
|---|---|---|---|
| _[e.g., Frontend]_ | _[React]_ | _[18+]_ | _[Team expertise, component ecosystem]_ |
| _[e.g., Backend]_ | _[Node.js / Express]_ | _[20+]_ | _[Shared language with frontend]_ |
| _[e.g., Database]_ | _[PostgreSQL]_ | _[15+]_ | _[Relational data, JSONB support]_ |
| _[e.g., Cache]_ | _[Redis]_ | _[7+]_ | _[Session storage, rate limiting]_ |
| _[e.g., Queue]_ | _[RabbitMQ]_ | _[3.12+]_ | _[Background job processing]_ |

---

## Database Schema

_Physical schema mapping from the conceptual data model. Include only where the physical schema diverges from the conceptual model (normalisation, denormalisation, indexes, constraints)._

### _[Table name]_

| Column | Type | Constraints | Maps to |
|---|---|---|---|
| _[column]_ | _[VARCHAR(100)]_ | _[NOT NULL]_ | _[Entity.field]_ |
| _[column]_ | _[TIMESTAMP]_ | _[DEFAULT NOW()]_ | _[Entity.field]_ |
| _[column]_ | _[INTEGER]_ | _[FOREIGN KEY → table(id)]_ | _[Relationship]_ |

### Indexes

| Table | Columns | Type | Rationale |
|---|---|---|---|
| _[table]_ | _[column(s)]_ | _[B-tree / GIN / unique]_ | _[Query pattern it supports]_ |

---

## API Design

_Internal API endpoints exposed by this system (not external services — those are in 09)._

### Conventions

- **Style**: _[REST / GraphQL / gRPC]_
- **Base URL**: _[e.g., /api/v1]_
- **Authentication**: _[Bearer token / session cookie / API key]_
- **Versioning**: _[URL path / header / query parameter]_
- **Error format**: _[Standard error response shape]_

### Endpoints

#### _[METHOD]_ _[/path]_

- **Description**: _[What this does]_
- **Auth required**: _[yes / no]_
- **Request body**:
  ```json
  {}
  ```
- **Response body**:
  ```json
  {}
  ```
- **Error responses**: _[List of possible error codes and meanings]_

---

_Add more endpoints as needed._

---

## Infrastructure

### Environments

| Environment | Purpose | URL | Notes |
|---|---|---|---|
| _[e.g., Development]_ | _[Local dev]_ | _[localhost:3000]_ | _[Uses local DB, mocked external services]_ |
| _[e.g., Staging]_ | _[Pre-production testing]_ | _[staging.example.com]_ | _[Real external services, test data]_ |
| _[e.g., Production]_ | _[Live system]_ | _[example.com]_ | _[Full monitoring, real data]_ |

### Deployment

- **Method**: _[CI/CD pipeline / manual / GitOps]_
- **Platform**: _[AWS / GCP / Azure / Vercel / self-hosted]_
- **Containerised**: _[yes (Docker) / no]_
- **Rollback strategy**: _[Blue-green / canary / immediate rollback / database migration rollback]_

### Session Management

- **Storage**: _[Server-side (Redis) / JWT / cookie-based]_
- **Duration**: _[e.g., 30 minutes idle, 24 hours absolute]_
- **Refresh**: _[Sliding window / fixed expiry / refresh token]_
- **Multi-device**: _[Allowed / single session only]_

---

## Security Architecture

| Concern | Approach |
|---|---|
| _[e.g., CSRF protection]_ | _[Token-based, SameSite cookies]_ |
| _[e.g., XSS prevention]_ | _[Content Security Policy, output encoding]_ |
| _[e.g., SQL injection]_ | _[Parameterised queries, ORM]_ |
| _[e.g., Rate limiting]_ | _[RATE-XXX from constraints, implemented via Redis]_ |
| _[e.g., Input sanitisation]_ | _[Server-side validation, HTML encoding]_ |
| _[e.g., Secrets management]_ | _[Environment variables, vault, not in code]_ |
