# 09 — External Interactions

## Services

### EXT-001: Vehicle Lookup Service

- **Description**: Returns vehicle details from a UK registration number
- **Input**: Vehicle.registrationNumber
- **Output**: make, model, year, engineSize, fuelType
- **Failure modes**:
  - Timeout (>5s): Show manual entry form, log warning
  - Not found: Trigger [COND-002](07-conditional-logic.md#COND-002) — require manual entry
  - Service down: Show manual entry form, alert ops team
  - Invalid format: Handled before call by [VR-003](04-validation-rules.md#VR-003)
- **Fallback**: Manual vehicle entry (user provides all fields)

### EXT-002: Insurance Quote Providers

- **Description**: Third-party insurers that return price quotes based on a complete quote request
- **Input**: Complete QuoteRequest entity (driver + vehicle + cover details)
- **Output**: List of Quote entities (provider, price, cover level, excess)
- **Failure modes**:
  - Timeout (>15s per provider): Exclude that provider from results, continue with others
  - Provider returns error: Exclude that provider, continue with others
  - All providers fail: Trigger [CON-003](08-constraints.md#CON-003)
  - Unexpected response format: Log error, exclude provider, continue with others
- **Fallback**: If all providers fail, show error message and suggest retry later
