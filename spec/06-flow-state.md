# 06 — Flow / State Model [REQUIRED]

> How the system progresses — states and transitions, not pages.
> A state is "what's true right now." A transition is "what must happen to move forward."
>
> **Dependencies**: [02-data-model.md](02-data-model.md), [04-validation-rules.md](04-validation-rules.md), [05-business-rules.md](05-business-rules.md)
> **Guide**: [docs/GUIDE.md — Flow / State](../docs/GUIDE.md#06--flow--state-model-required)
> **Format**: [docs/FORMAT.md — Flow / State Format](../docs/FORMAT.md#flow--state-format)

---

## States

### STATE-001: _[State name]_

- **Description**: _[What's true when the system is in this state]_
- **Entry conditions**: _[What must be true to enter this state]_
- **Required data**: _[Which fields must be present/valid]_
- **Exit conditions**: _[What must be true to leave this state]_
- **Transitions to**: _[STATE-XXX](06-flow-state.md#STATE-XXX) — [condition]_
- **Resumable**: _[yes/no — if yes, what persists and for how long]_

---

### STATE-002: _[State name]_

- **Description**: _[What's true in this state]_
- **Entry conditions**: _[What must be true]_
- **Required data**: _[Which fields]_
- **Exit conditions**: _[What must be true to leave]_
- **Transitions to**: _[STATE-XXX] — [condition]_
- **Resumable**: _[yes/no]_

---

_Add more states as needed._

---

## Terminal States

_States where the flow ends._

| State | Type | Description |
|---|---|---|
| _[STATE-XXX]_ | success | _[Flow completed successfully]_ |
| _[STATE-XXX]_ | failure | _[Flow cannot continue]_ |
| _[STATE-XXX]_ | abandoned | _[User left without completing]_ |

---

## Flow Diagram

_Optional: describe the flow as a simple text diagram._

```
[STATE-001] → [STATE-002] → [STATE-003] → [SUCCESS]
                   ↓
              [STATE-004] → [FAILURE]
```
