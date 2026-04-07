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
