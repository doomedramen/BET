# 03 — Permissions & Roles [OPTIONAL]

> Who can do what. What's visible vs actionable per role.
> Skip this section for single-role systems.
>
> **Dependencies**: [01-capabilities.md](01-capabilities.md), [02-data-model.md](02-data-model.md)
> **Guide**: [docs/GUIDE.md — Permissions](../docs/GUIDE.md#03--permissions-optional)

---

## Roles

| Role | Description |
|---|---|
| _[e.g., Customer]_ | _[What this role does in the system]_ |
| _[e.g., Admin]_ | _[What this role does]_ |

---

## Capability Access

_Which roles can perform which capabilities._

| Capability | _Role 1_ | _Role 2_ | _Role 3_ |
|---|---|---|---|
| [CAP-001](01-capabilities.md#CAP-001) | yes | no | yes |
| [CAP-002](01-capabilities.md#CAP-002) | no | yes | yes |

---

## Data Access

_Who can see and modify whose data._

### PERM-001: _[Permission name]_

- **Role**: _[Which role]_
- **Entity**: _[Which entity]_
- **Can view**: _[own / all / none]_
- **Can modify**: _[own / all / none]_
- **Can delete**: _[own / all / none]_

---

## Transition Permissions

_Who can trigger which state transitions._

| Transition | Allowed roles |
|---|---|
| _[e.g., Submit for review]_ | _[Customer]_ |
| _[e.g., Approve]_ | _[Admin]_ |
