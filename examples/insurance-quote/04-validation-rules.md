# 04 — Validation Rules

## Rules

### VR-001: Date of Birth Valid Date

- **Field**: [Driver.dateOfBirth](02-data-model.md#ENT-001)
- **Rule**: Must be a valid date in the past
- **Timing**: on input
- **Message**: [MSG-001](11-messages.md#MSG-001)

### VR-002: Date of Birth Not Future

- **Field**: [Driver.dateOfBirth](02-data-model.md#ENT-001)
- **Rule**: Must not be a future date
- **Timing**: on input
- **Message**: [MSG-002](11-messages.md#MSG-002)

### VR-003: Registration Number Format

- **Field**: [Vehicle.registrationNumber](02-data-model.md#ENT-002)
- **Rule**: Must match UK registration format (e.g., AB12 CDE)
- **Timing**: on submit
- **Message**: [MSG-003](11-messages.md#MSG-003)

### VR-004: Estimated Value Positive

- **Field**: [Vehicle.estimatedValue](02-data-model.md#ENT-002)
- **Rule**: Must be a positive number greater than 0
- **Timing**: on input
- **Message**: [MSG-004](11-messages.md#MSG-004)

### VR-005: First Name Required

- **Field**: [Driver.firstName](02-data-model.md#ENT-001)
- **Rule**: Must not be empty, max 100 characters
- **Timing**: on submit
- **Message**: [MSG-005](11-messages.md#MSG-005)

### VR-006: Last Name Required

- **Field**: [Driver.lastName](02-data-model.md#ENT-001)
- **Rule**: Must not be empty, max 100 characters
- **Timing**: on submit
- **Message**: [MSG-005](11-messages.md#MSG-005)

### VR-007: Start Date Future

- **Field**: [QuoteRequest.startDate](02-data-model.md#ENT-003)
- **Rule**: Must be today or a future date, no more than 30 days ahead
- **Timing**: on submit
- **Message**: [MSG-006](11-messages.md#MSG-006)

---

## Cross-Field Validations

### VR-008: Years Held Consistent With Age

- **Fields**: [Driver.age](02-data-model.md#ENT-001), [Driver.yearsHeld](02-data-model.md#ENT-001)
- **Rule**: yearsHeld must not exceed (age - 17). Cannot have held a licence longer than legally possible.
- **Timing**: on submit
- **Message**: [MSG-007](11-messages.md#MSG-007)
