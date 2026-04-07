# 05 — Business Rules [REQUIRED]

> Core system logic in IF/THEN/ELSE format.
> Every rule needs a priority, an origin, and references to the data it operates on.
>
> **Dependencies**: [02-data-model.md](02-data-model.md), [04-validation-rules.md](04-validation-rules.md)
> **Guide**: [docs/GUIDE.md — Business Rules](../docs/GUIDE.md#05--business-rules-required)
> **Format**: [docs/FORMAT.md — Business Rules Format](../docs/FORMAT.md#business-rules-format)

---

## Rules

### BR-001: _[Rule name]_

- **Priority**: _[1 (blocking) / 2 (high) / 3 (medium) / 4 (low)]_
- **Origin**: _[legal / business / technical / ux]_
- **Rule**:
  IF _[condition]_
  THEN _[outcome]_
  ELSE _[alternative]_
- **References**: _[Entity.field](02-data-model.md#ENT-XXX)_

---

### BR-002: _[Rule name]_

- **Priority**: _[1 / 2 / 3 / 4]_
- **Origin**: _[legal / business / technical / ux]_
- **Rule**:
  IF _[condition]_
  THEN _[outcome]_
- **References**: _[Entity.field](02-data-model.md#ENT-XXX)_

---

_Add more rules as needed._

---

## Priority Resolution

_When multiple rules apply to the same situation, they are evaluated in priority order (1 first). Document any specific conflicts here._

| Conflict | Rule A | Rule B | Resolution |
|---|---|---|---|
| _[Description]_ | _[BR-XXX]_ | _[BR-XXX]_ | _[Which wins and why]_ |

---

## Presentation Rules

_How lists, collections, and results are displayed — default ordering, available sorting, filtering, and pagination. These are behavioural decisions, not UI decisions._

### PR-001: _[List/collection name]_

- **Context**: _[Where this list appears — e.g., quote results, search results, user list]_
- **Default sort**: _[Field and direction — e.g., annualPrice ascending]_
- **Available sort options**: _[Which fields can the user sort by, if any]_
- **Filtering**: _[What can be filtered, and by which criteria]_
- **Pagination**: _[Page size, infinite scroll, load more, or all-at-once. Max items.]_
- **Empty state**: _[What happens when the list has zero items — link to message]_
- **References**: _[Link to relevant entities and states]_

---

_Add more presentation rules as needed. Every state that presents a list or collection needs one._
