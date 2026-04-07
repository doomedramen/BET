# 11 — Content & Messages

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

### MSG-012: Quote Rate Limit Exceeded

- **Severity**: warning
- **Trigger**: [RATE-001](08-constraints.md#RATE-001)
- **Text**: "You've requested too many quotes recently. Please try again in {waitTime}."
- **Parameters**: waitTime = time remaining in the rate limit window

### MSG-013: Vehicle Lookup Rate Limit

- **Severity**: info
- **Trigger**: [RATE-002](08-constraints.md#RATE-002)
- **Text**: "Please enter your vehicle details manually."
- **Parameters**: none

---

## State Messages

| State | Message | Severity |
|---|---|---|
| QuoteCalculation (loading) | "Getting your quotes..." | info |
| QuoteCalculation (extended, >5s) | "Still working — checking with our providers..." | info |
| QuoteResults (empty after filter) | "No quotes match your filters. Try adjusting your preferences." | info |
| Resume prompt | "Welcome back. We've saved your progress from {savedDate}." | info |

---

## Help Text & Field Descriptions

### HELP-001: Date of Birth

- **Context**: Alongside Driver.dateOfBirth field
- **Text**: "We need your date of birth to check eligibility and calculate your quote."
- **Display**: always visible (below field)

### HELP-002: Licence Type

- **Context**: Alongside Driver.licenceType field
- **Text**: "Select the type of driving licence you currently hold. A provisional licence limits the cover types available to you."
- **Display**: always visible (below field)

### HELP-003: Estimated Vehicle Value

- **Context**: Alongside Vehicle.estimatedValue field
- **Text**: "Enter the amount you'd expect to receive if you sold your vehicle today."
- **Display**: always visible (below field)

### HELP-004: Cover Type

- **Context**: Alongside QuoteRequest.coverType field
- **Text**: "The level of protection for your vehicle. Comprehensive covers damage to your vehicle and others. Third-party covers damage you cause to others only."
- **Display**: expandable (collapsed by default)

### HELP-005: Vehicle Usage

- **Context**: Alongside Vehicle.usage field
- **Text**: "How the vehicle is primarily used. Business use may affect your premium and the providers available to you."
- **Display**: always visible (below field)

### HELP-006: Start Date

- **Context**: Alongside QuoteRequest.startDate field
- **Text**: "When you'd like your cover to begin. Must be within the next 30 days."
- **Display**: always visible (below field)

---

## Legal & Compliance Text

### LEGAL-001: Data Processing Consent

- **Location**: [STATE-001](06-flow-state.md#STATE-001) (CollectDriverDetails) — shown before first submission
- **Text**: "By proceeding, you consent to us processing your personal data to generate insurance quotes. Your data will be shared with our insurance provider partners. See our [Privacy Policy] for details."
- **Interaction**: must check box
- **Required by**: GDPR — Article 6(1)(a) consent for data processing

### LEGAL-002: Terms of Use

- **Location**: [STATE-003](06-flow-state.md#STATE-003) (CollectCoverDetails) — shown before quote submission
- **Text**: "By requesting quotes, you confirm that the information you've provided is accurate and complete to the best of your knowledge. Providing false information may invalidate your policy. You agree to our [Terms of Service]."
- **Interaction**: must check box
- **Required by**: Business policy — accuracy declaration

### LEGAL-003: Cookie Notice

- **Location**: All states — shown on first visit
- **Text**: "We use essential cookies to keep your session active and save your progress. We also use analytics cookies to improve our service. [Manage preferences]"
- **Interaction**: can dismiss (accept all) or manage preferences
- **Required by**: UK PECR / ePrivacy

---

## Contextual Content

### CONTENT-001: Quote Results Introduction

- **Context**: Top of [STATE-005](06-flow-state.md#STATE-005) (QuoteResults)
- **Text**: "Here are your quotes, sorted by annual price. All prices shown include Insurance Premium Tax (IPT). Quotes are valid for 24 hours."
- **Purpose**: Orients the user, sets expectations about pricing and validity

### CONTENT-002: No Quotes Guidance

- **Context**: [STATE-006](06-flow-state.md#STATE-006) (NoQuotes), below MSG-010
- **Text**: "This can happen if your vehicle or driver profile doesn't match what our providers currently cover. You could try adjusting your cover type, vehicle usage, or start date. If you continue to have difficulty, contact us for assistance."
- **Purpose**: Helps the user understand why and what to do next

### CONTENT-003: Step Indicator

- **Context**: All data collection states (STATE-001 through STATE-003)
- **Text**: "Step {currentStep} of {totalSteps}: {stepName}"
- **Purpose**: Tells the user where they are in the flow
