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

---

## Field Visibility & Editability

_For each state, document which fields are visible, hidden, read-only, or conditionally required. This captures the behavioural rules that determine what the user can see and interact with._

### _[STATE-XXX]_: _[State name]_

| Field | Default | Condition | Becomes |
|---|---|---|---|
| _Entity.field_ | _visible + editable_ | _[none — always this way]_ | — |
| _Entity.field_ | _hidden_ | _[COND-XXX is true]_ | _visible + required_ |
| _Entity.field_ | _visible + editable_ | _[COND-XXX is true]_ | _visible + read-only_ |
| _Entity.field_ | _visible + editable_ | _[Entity.otherField == "value"]_ | _hidden_ |

**Editability values**: `editable`, `read-only`, `disabled`, `hidden`

---

_Repeat for each state that has user-facing fields. Every field from the data model should appear in at least one state's visibility table._
