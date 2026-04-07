# 05 — Business Rules

## Rules

### BR-001: Minimum Driver Age

- **Priority**: 1 (blocking)
- **Origin**: legal
- **Rule**:
  IF Driver.age < 17
  THEN reject quote request
  AND trigger [CON-001](08-constraints.md#CON-001)
- **References**: [Driver.age](02-data-model.md#ENT-001)

### BR-002: High Risk Occupation

- **Priority**: 3 (medium)
- **Origin**: business
- **Rule**:
  IF Driver.occupation IN ["racing driver", "delivery driver", "stunt performer"]
  THEN flag quote request as high risk
- **References**: [Driver.occupation](02-data-model.md#ENT-001)

### BR-003: High Value Vehicle

- **Priority**: 3 (medium)
- **Origin**: business
- **Rule**:
  IF Vehicle.estimatedValue > 50000
  THEN flag quote request as high value
  AND limit available providers to those accepting high-value vehicles
- **References**: [Vehicle.estimatedValue](02-data-model.md#ENT-002)

### BR-004: Business Use Requires Additional Data

- **Priority**: 2 (high)
- **Origin**: business
- **Rule**:
  IF Vehicle.usage == "business"
  THEN require additional field: businessType
  AND trigger [COND-001](07-conditional-logic.md#COND-001)
- **References**: [Vehicle.usage](02-data-model.md#ENT-002)

### BR-005: New Driver Premium Loading

- **Priority**: 3 (medium)
- **Origin**: business
- **Rule**:
  IF Driver.yearsHeld < 2
  THEN apply new driver premium loading factor
- **References**: [Driver.yearsHeld](02-data-model.md#ENT-001)

---

## Priority Resolution

| Conflict | Rule A | Rule B | Resolution |
|---|---|---|---|
| Age + occupation | BR-001 | BR-002 | BR-001 wins — if underage, occupation doesn't matter (blocked entirely) |

---

## Presentation Rules

### PR-001: Quote Results List

- **Context**: Quotes displayed in [STATE-005](06-flow-state.md#STATE-005) (QuoteResults)
- **Default sort**: annualPrice ascending (cheapest first)
- **Available sort options**: annualPrice (asc/desc), monthlyPrice (asc/desc), provider name (A-Z), excess (asc/desc)
- **Filtering**: coverLevel (comprehensive, third-party, third-party fire & theft), excess range (min/max slider)
- **Pagination**: All results shown at once (max 20 providers — no pagination needed)
- **Empty state**: [MSG-010](11-messages.md#MSG-010) — "No quotes are available for your details."
- **References**: [ENT-004](02-data-model.md#ENT-004), [STATE-005](06-flow-state.md#STATE-005)
