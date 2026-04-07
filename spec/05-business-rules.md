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
