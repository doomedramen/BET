# 13 — Edge Cases & Exceptions

## Edge Cases

### EDGE-001: Partial Data on Browser Close

- **Category**: partial state
- **Scenario**: User closes browser mid-flow at any state
- **Expected behaviour**: Data persists for 30 days. On return, user resumes from last valid state with all entered data preserved.
- **References**: All states in [06-flow-state.md](06-flow-state.md) — resumability

### EDGE-002: No Quotes Returned

- **Category**: boundary
- **Scenario**: All providers return valid responses but none include quotes for this user/vehicle combination
- **Expected behaviour**: Transition to [STATE-006](06-flow-state.md#STATE-006). Display [MSG-010](11-messages.md#MSG-010). Allow user to adjust inputs and retry.
- **References**: [STATE-006](06-flow-state.md#STATE-006)

### EDGE-003: Some Providers Timeout

- **Category**: timing
- **Scenario**: 2 of 5 providers respond, 3 timeout
- **Expected behaviour**: Display quotes from responding providers. Do not wait for timed-out providers. No error shown — partial results are acceptable.
- **References**: [EXT-002](09-external-interactions.md#EXT-002) failure modes

### EDGE-004: Vehicle Lookup Returns Partial Data

- **Category**: data
- **Scenario**: Vehicle lookup returns make and model but not engineSize or fuelType
- **Expected behaviour**: Auto-populate available fields, prompt user to fill in missing fields manually.
- **References**: [EXT-001](09-external-interactions.md#EXT-001), [COND-003](07-conditional-logic.md#COND-003)

### EDGE-005: User Changes Registration After Lookup

- **Category**: data
- **Scenario**: User gets auto-populated vehicle data, then changes the registration number
- **Expected behaviour**: Clear all auto-populated fields. Trigger a new lookup for the new registration.
- **References**: [COND-002](07-conditional-logic.md#COND-002), [COND-003](07-conditional-logic.md#COND-003)

### EDGE-006: Age Boundary — Turns 17 Today

- **Category**: boundary
- **Scenario**: User's date of birth means they turn 17 today
- **Expected behaviour**: Age is calculated as 17. User is eligible. [BR-001](05-business-rules.md#BR-001) does not block.
- **References**: [BR-001](05-business-rules.md#BR-001), [Driver.age](02-data-model.md#ENT-001) calculation

### EDGE-007: Session Expires During Quote Calculation

- **Category**: timing
- **Scenario**: User's session expires while STATE-004 (QuoteCalculation) is in progress
- **Expected behaviour**: Quote calculation continues server-side. Results are stored. When user returns (within 24 hours), results are displayed. After 24 hours, recalculation required.
- **References**: [STATE-004](06-flow-state.md#STATE-004), [STATE-005](06-flow-state.md#STATE-005)

### EDGE-008: Estimated Value of Zero

- **Category**: data
- **Scenario**: User enters 0 as vehicle estimated value
- **Expected behaviour**: Blocked by [VR-004](04-validation-rules.md#VR-004). Display [MSG-004](11-messages.md#MSG-004).
- **References**: [VR-004](04-validation-rules.md#VR-004)

### EDGE-009: Occupation With Special Characters

- **Category**: data
- **Scenario**: User enters occupation with apostrophes, hyphens, or accented characters (e.g., "nurse's aide", "self-employed")
- **Expected behaviour**: Accept all standard punctuation and Unicode characters. No sanitisation errors.
- **References**: [Driver.occupation](02-data-model.md#ENT-001)

### EDGE-010: User Goes Back and Changes Driver Details

- **Category**: partial state
- **Scenario**: User completes vehicle details, then goes back to change driver date of birth
- **Expected behaviour**: Driver details are updated. Vehicle details are preserved. But quote must be recalculated — any cached quotes are invalidated.
- **References**: [STATE-001](06-flow-state.md#STATE-001), [STATE-004](06-flow-state.md#STATE-004)
