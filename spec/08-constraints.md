# 08 — Constraints & Blocking Conditions [REQUIRED]

> Hard stops that prevent progress entirely.
> Different from business rules: a business rule might flag something, a constraint blocks entirely.
>
> **Dependencies**: [05-business-rules.md](05-business-rules.md), [06-flow-state.md](06-flow-state.md)
> **Guide**: [docs/GUIDE.md — Constraints](../docs/GUIDE.md#08--constraints-required)
> **Format**: [docs/FORMAT.md — Constraints Format](../docs/FORMAT.md#constraints-format)

---

## Constraints

### CON-001: _[Constraint name]_

- **Condition**: _[What triggers the block]_
- **Effect**: _[What is prevented — reference the state(s) blocked]_
- **Recovery**: _[What can the user do? Options: adjust inputs, contact support, retry later, none]_
- **Message**: _[MSG-XXX](11-messages.md#MSG-XXX)_

---

### CON-002: _[Constraint name]_

- **Condition**: _[What triggers the block]_
- **Effect**: _[What is prevented]_
- **Recovery**: _[What can the user do]_
- **Message**: _[MSG-XXX](11-messages.md#MSG-XXX)_

---

_Add more constraints as needed._

---

## Rate Limits & Abuse Prevention

_Rules that limit how frequently actions can be performed. These protect the system and its external dependencies from abuse._

### RATE-001: _[Rate limit name]_

- **Action**: _[What is being limited — e.g., quote requests, vehicle lookups, login attempts]_
- **Limit**: _[Number of allowed actions per time window — e.g., 10 per minute, 50 per hour]_
- **Scope**: _[per user / per session / per IP / global]_
- **When exceeded**: _[What happens — block, queue, degrade, show message]_
- **Recovery**: _[When can the user try again — after time window resets, after CAPTCHA, etc.]_
- **Message**: _[MSG-XXX](11-messages.md#MSG-XXX)_

---

_Add more rate limits as needed. Consider limits for: form submissions, API calls to external services, authentication attempts, data exports._
