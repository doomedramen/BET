# 14 — Dependencies & Assumptions [OPTIONAL]

> What must be true for this system to work.
> External systems, seed data, infrastructure, and things you're taking for granted.
>
> **Dependencies**: All previous sections
> **Guide**: [docs/GUIDE.md — Dependencies](../docs/GUIDE.md#14--dependencies-optional)

---

## External System Dependencies

_Systems that must exist and be available._

### DEP-001: _[System name]_

- **What it provides**: _[What your system needs from it]_
- **Required availability**: _[always / during business hours / on demand]_
- **What happens if unavailable**: _[Link to fallback in 09-external-interactions.md or describe]_

---

## Data Dependencies

_Data that must be pre-populated or seeded before the system can function._

### DEP-XXX: _[Data name]_

- **Description**: _[What data must exist]_
- **Source**: _[Where does it come from — manual seed, migration, external import]_
- **Update frequency**: _[static / daily / real-time]_

---

## Infrastructure Assumptions

_Infrastructure the system assumes is in place._

| Assumption | Details |
|---|---|
| _[e.g., Database]_ | _[PostgreSQL 15+, available at all times]_ |
| _[e.g., Cache]_ | _[Redis, used for session storage]_ |
| _[e.g., Queue]_ | _[RabbitMQ, used for background jobs]_ |

---

## Environment Assumptions

_Conditions about the runtime environment._

| Assumption | Details |
|---|---|
| _[e.g., Authentication]_ | _[Users are authenticated via SSO before reaching this system]_ |
| _[e.g., Network]_ | _[System has outbound internet access for API calls]_ |
