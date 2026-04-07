# 11 — Messages & Copy [REQUIRED]

> Every piece of user-facing text — error messages, success confirmations, warnings, info text.
> Each message needs a trigger and a severity level.
>
> **Dependencies**: [04-validation-rules.md](04-validation-rules.md), [05-business-rules.md](05-business-rules.md), [08-constraints.md](08-constraints.md)
> **Guide**: [docs/GUIDE.md — Messages](../docs/GUIDE.md#11--messages-required)
> **Format**: [docs/FORMAT.md — Messages Format](../docs/FORMAT.md#messages-format)

---

## Messages

### MSG-001: _[Message name]_

- **Severity**: _[error / warning / info / success]_
- **Trigger**: _[What causes this message — link to rule, constraint, or state]_
- **Text**: "_[The exact message text. Use {paramName} for dynamic content.]_"
- **Parameters**: _[paramName = description of value]_

---

### MSG-002: _[Message name]_

- **Severity**: _[error / warning / info / success]_
- **Trigger**: _[What causes this message]_
- **Text**: "_[Message text]_"
- **Parameters**: _[if any]_

---

_Add more messages as needed._

---

## State Messages

_Messages shown during specific system states (empty, loading, etc.)._

| State | Message | Severity |
|---|---|---|
| _[e.g., Empty results]_ | _"No results found. Try adjusting your search."_ | info |
| _[e.g., Loading]_ | _"Loading your results..."_ | info |
