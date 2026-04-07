# 07 — Conditional Logic [REQUIRED]

> What changes based on context — dynamic behaviour triggered by data or state.
>
> **Dependencies**: [02-data-model.md](02-data-model.md), [06-flow-state.md](06-flow-state.md)
> **Guide**: [docs/GUIDE.md — Conditional Logic](../docs/GUIDE.md#07--conditional-logic-required)
> **Format**: [docs/FORMAT.md — Conditional Logic Format](../docs/FORMAT.md#conditional-logic-format)

---

## Conditions

### COND-001: _[Condition name]_

- **Trigger**: _[What causes this condition to be evaluated]_
- **Outcome**: _[What changes when the condition is true]_
- **Cascades**: _[Does this affect other conditions or fields? If so, which?]_
- **References**: _[Link to relevant data model fields, states, or external interactions]_

---

### COND-002: _[Condition name]_

- **Trigger**: _[What causes evaluation]_
- **Outcome**: _[What changes]_
- **Cascades**: _[Downstream effects]_
- **References**: _[Links]_

---

_Add more conditions as needed._

---

## Cascade Map

_If conditions cascade (A triggers B triggers C), document the chain here._

| Starting condition | Triggers | Which triggers | End result |
|---|---|---|---|
| _[COND-XXX]_ | _[COND-XXX]_ | _[COND-XXX]_ | _[Final outcome]_ |
