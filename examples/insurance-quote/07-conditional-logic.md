# 07 — Conditional Logic

## Conditions

### COND-001: Business Use Additional Fields

- **Trigger**: Vehicle.usage is set to "business"
- **Outcome**: Require additional field: businessType (string, required, user input). Options: delivery, taxi, sales, other.
- **Cascades**: businessType value may affect available insurance providers
- **References**: [BR-004](05-business-rules.md#BR-004), [Vehicle.usage](02-data-model.md#ENT-002)

### COND-002: Vehicle Not Found

- **Trigger**: Vehicle registration lookup ([EXT-001](09-external-interactions.md#EXT-001)) returns no results
- **Outcome**: Require manual vehicle entry — user must provide make, model, year, engineSize, fuelType manually
- **Cascades**: None
- **References**: [EXT-001](09-external-interactions.md#EXT-001)

### COND-003: Vehicle Found

- **Trigger**: Vehicle registration lookup ([EXT-001](09-external-interactions.md#EXT-001)) returns results
- **Outcome**: Auto-populate make, model, year, engineSize, fuelType from lookup response. Fields become read-only.
- **Cascades**: None
- **References**: [EXT-001](09-external-interactions.md#EXT-001)

### COND-004: Provisional Licence

- **Trigger**: Driver.licenceType is set to "provisional"
- **Outcome**: Limit coverType options to third-party only
- **Cascades**: Affects available options in [STATE-003](06-flow-state.md#STATE-003)
- **References**: [Driver.licenceType](02-data-model.md#ENT-001)

---

## Cascade Map

| Starting condition | Triggers | End result |
|---|---|---|
| COND-001 (business use) | Provider filtering | Fewer quotes returned |
| COND-004 (provisional licence) | Cover type restriction | Only third-party cover available |

---

## Field Visibility & Editability

### STATE-001: CollectDriverDetails

| Field | Default | Condition | Becomes |
|---|---|---|---|
| Driver.firstName | visible + editable | — | — |
| Driver.lastName | visible + editable | — | — |
| Driver.dateOfBirth | visible + editable | — | — |
| Driver.occupation | visible + editable | — | — |
| Driver.licenceType | visible + editable | — | — |
| Driver.yearsHeld | visible + editable | — | — |

### STATE-002: CollectVehicleDetails

| Field | Default | Condition | Becomes |
|---|---|---|---|
| Vehicle.registrationNumber | visible + editable | — | — |
| Vehicle.make | visible + editable | [COND-003](#COND-003) (lookup found) | visible + read-only |
| Vehicle.model | visible + editable | [COND-003](#COND-003) (lookup found) | visible + read-only |
| Vehicle.year | visible + editable | [COND-003](#COND-003) (lookup found) | visible + read-only |
| Vehicle.engineSize | visible + editable | [COND-003](#COND-003) (lookup found) | visible + read-only |
| Vehicle.fuelType | visible + editable | [COND-003](#COND-003) (lookup found) | visible + read-only |
| Vehicle.estimatedValue | visible + editable | — | — |
| Vehicle.usage | visible + editable | — | — |
| Vehicle.businessType | hidden | [COND-001](#COND-001) (usage == "business") | visible + required |

### STATE-003: CollectCoverDetails

| Field | Default | Condition | Becomes |
|---|---|---|---|
| QuoteRequest.coverType | visible + editable (all options) | [COND-004](#COND-004) (provisional licence) | visible + editable (third-party only) |
| QuoteRequest.startDate | visible + editable | — | — |
