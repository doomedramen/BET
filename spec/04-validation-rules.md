# 04 — Validation Rules [REQUIRED]

> Input correctness rules — separated from business logic.
> A validation rule checks that data is well-formed. A business rule decides what to do with valid data.
>
> **Dependencies**: [02-data-model.md](02-data-model.md)
> **Guide**: [docs/GUIDE.md — Validation Rules](../docs/GUIDE.md#04--validation-rules-required)
> **Format**: [docs/FORMAT.md — Validation Rules Format](../docs/FORMAT.md#validation-rules-format)

---

## Rules

### VR-001: _[Rule name]_

- **Field**: _[Entity.fieldName](02-data-model.md#ENT-XXX)_
- **Rule**: _[What makes this field valid]_
- **Timing**: _[on input / on submit / on state transition / on save]_
- **Message**: _[MSG-XXX](11-messages.md#MSG-XXX)_

---

### VR-002: _[Rule name]_

- **Field**: _[Entity.fieldName](02-data-model.md#ENT-XXX)_
- **Rule**: _[What makes this field valid]_
- **Timing**: _[on input / on submit / on state transition / on save]_
- **Message**: _[MSG-XXX](11-messages.md#MSG-XXX)_

---

## Cross-Field Validations

_Rules that depend on multiple fields._

### VR-XXX: _[Rule name]_

- **Fields**: _[Entity.fieldA](02-data-model.md), [Entity.fieldB](02-data-model.md)_
- **Rule**: _[e.g., endDate must be after startDate]_
- **Timing**: _[when is this checked]_
- **Message**: _[MSG-XXX](11-messages.md#MSG-XXX)_
