# 11 — Messages & Copy

## Messages

### MSG-001: Invalid Date of Birth

- **Severity**: error
- **Trigger**: [VR-001](04-validation-rules.md#VR-001)
- **Text**: "Please enter a valid date of birth."
- **Parameters**: none

### MSG-002: Future Date of Birth

- **Severity**: error
- **Trigger**: [VR-002](04-validation-rules.md#VR-002)
- **Text**: "Date of birth cannot be in the future."
- **Parameters**: none

### MSG-003: Invalid Registration

- **Severity**: error
- **Trigger**: [VR-003](04-validation-rules.md#VR-003)
- **Text**: "Please enter a valid UK registration number (e.g., AB12 CDE)."
- **Parameters**: none

### MSG-004: Invalid Vehicle Value

- **Severity**: error
- **Trigger**: [VR-004](04-validation-rules.md#VR-004)
- **Text**: "Vehicle value must be greater than 0."
- **Parameters**: none

### MSG-005: Required Field Missing

- **Severity**: error
- **Trigger**: [VR-005](04-validation-rules.md#VR-005), [VR-006](04-validation-rules.md#VR-006)
- **Text**: "{fieldName} is required."
- **Parameters**: fieldName = display name of the missing field

### MSG-006: Start Date Out of Range

- **Severity**: error
- **Trigger**: [VR-007](04-validation-rules.md#VR-007), [CON-002](08-constraints.md#CON-002)
- **Text**: "Cover start date must be between today and {maxDate}."
- **Parameters**: maxDate = today + 30 days

### MSG-007: Years Held Inconsistent

- **Severity**: error
- **Trigger**: [VR-008](04-validation-rules.md#VR-008)
- **Text**: "Years held cannot exceed {maxYears} based on your date of birth."
- **Parameters**: maxYears = Driver.age - 17

### MSG-008: Underage Error

- **Severity**: error
- **Trigger**: [BR-001](05-business-rules.md#BR-001), [CON-001](08-constraints.md#CON-001)
- **Text**: "You must be at least {minAge} years old to get a quote."
- **Parameters**: minAge = 17

### MSG-009: All Providers Failed

- **Severity**: error
- **Trigger**: [CON-003](08-constraints.md#CON-003)
- **Text**: "We're unable to get quotes at the moment. Please try again later or contact us for help."
- **Parameters**: none

### MSG-010: No Quotes Available

- **Severity**: info
- **Trigger**: [STATE-006](06-flow-state.md#STATE-006)
- **Text**: "No quotes are available for your details. You can adjust your information and try again."
- **Parameters**: none

### MSG-011: Vehicle Lookup Failed

- **Severity**: warning
- **Trigger**: [EXT-001](09-external-interactions.md#EXT-001) timeout or service down
- **Text**: "We couldn't look up your vehicle automatically. Please enter the details manually."
- **Parameters**: none

---

## State Messages

| State | Message | Severity |
|---|---|---|
| QuoteCalculation (loading) | "Getting your quotes..." | info |
| QuoteResults (empty after filter) | "No quotes match your filters. Try adjusting your preferences." | info |
| Resume prompt | "Welcome back. We've saved your progress from {savedDate}." | info |
