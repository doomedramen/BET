# 15 — Non-Functional Requirements [OPTIONAL]

> The "how well" — performance, availability, data retention, security.
> Every requirement should be measurable.
>
> **Dependencies**: [01-capabilities.md](01-capabilities.md)
> **Guide**: [docs/GUIDE.md — Non-Functional](../docs/GUIDE.md#15--non-functional-requirements-optional)

---

## Performance

| Capability / Action | Target | Measurement |
|---|---|---|
| _[e.g., Page load]_ | _[< 2 seconds]_ | _[Time to interactive]_ |
| _[e.g., Quote calculation]_ | _[< 5 seconds]_ | _[Request to response]_ |
| _[e.g., Search results]_ | _[< 1 second]_ | _[Query to render]_ |

---

## Availability

| Requirement | Target |
|---|---|
| _[e.g., Uptime]_ | _[99.9%]_ |
| _[e.g., Maintenance window]_ | _[Sundays 02:00-04:00 UTC]_ |
| _[e.g., Recovery time]_ | _[< 1 hour]_ |

---

## Load

| Metric | Expected | Peak |
|---|---|---|
| _[e.g., Concurrent users]_ | _[500]_ | _[2000]_ |
| _[e.g., Requests per second]_ | _[100]_ | _[500]_ |
| _[e.g., Data volume]_ | _[10GB]_ | _[50GB over 2 years]_ |

---

## Data Retention

| Data | Retention period | Deletion method |
|---|---|---|
| _[e.g., User data]_ | _[2 years after last login]_ | _[Soft delete, hard delete after 30 days]_ |
| _[e.g., Logs]_ | _[90 days]_ | _[Auto-purge]_ |
| _[e.g., Audit trail]_ | _[7 years]_ | _[Archive to cold storage]_ |

---

## Security

| Requirement | Details |
|---|---|
| _[e.g., Encryption at rest]_ | _[AES-256 for PII fields]_ |
| _[e.g., Encryption in transit]_ | _[TLS 1.3]_ |
| _[e.g., Authentication]_ | _[OAuth 2.0 / OIDC]_ |
| _[e.g., Data classification]_ | _[PII fields identified in data model]_ |
