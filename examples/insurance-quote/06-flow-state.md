# 06 — Flow / State Model

## States

### STATE-001: CollectDriverDetails

- **Description**: System is gathering driver personal information
- **Entry conditions**: User has started a new quote (or resumed an existing one)
- **Required data**: Driver.firstName, Driver.lastName, Driver.dateOfBirth, Driver.occupation, Driver.licenceType, Driver.yearsHeld
- **Exit conditions**: All required Driver fields present and pass validation ([VR-001](04-validation-rules.md#VR-001) through [VR-006](04-validation-rules.md#VR-006)), and [BR-001](05-business-rules.md#BR-001) does not block
- **Transitions to**: [STATE-002](#STATE-002) (CollectVehicleDetails)
- **Resumable**: yes — data persists for 30 days

### STATE-002: CollectVehicleDetails

- **Description**: System is gathering vehicle information
- **Entry conditions**: Driver details are complete and valid
- **Required data**: Vehicle.registrationNumber, Vehicle.make, Vehicle.model, Vehicle.year, Vehicle.estimatedValue, Vehicle.usage
- **Exit conditions**: All required Vehicle fields present and pass validation, conditional fields satisfied (e.g., businessType if usage is "business")
- **Transitions to**: [STATE-003](#STATE-003) (CollectCoverDetails)
- **Resumable**: yes — data persists for 30 days

### STATE-003: CollectCoverDetails

- **Description**: System is gathering coverage preferences
- **Entry conditions**: Driver and vehicle details are complete and valid
- **Required data**: QuoteRequest.coverType, QuoteRequest.startDate
- **Exit conditions**: Cover type selected and start date is valid ([VR-007](04-validation-rules.md#VR-007))
- **Transitions to**: [STATE-004](#STATE-004) (QuoteCalculation)
- **Resumable**: yes — data persists for 30 days

### STATE-004: QuoteCalculation

- **Description**: System is requesting quotes from providers
- **Entry conditions**: All required data is complete and valid
- **Required data**: Complete QuoteRequest entity
- **Exit conditions**: Quotes received from providers (or timeout/failure handled)
- **Transitions to**: [STATE-005](#STATE-005) (QuoteResults) or [STATE-006](#STATE-006) (NoQuotes)
- **Resumable**: no — recalculation required on return

### STATE-005: QuoteResults

- **Description**: Quotes are available for the user to review
- **Entry conditions**: At least one quote was returned
- **Required data**: List of Quote entities
- **Exit conditions**: User selects a quote (out of scope) or starts a new quote
- **Transitions to**: N/A (terminal for this capability)
- **Resumable**: yes — results persist for 24 hours

### STATE-006: NoQuotes

- **Description**: No quotes were returned from any provider
- **Entry conditions**: Quote calculation completed but returned zero results
- **Required data**: None
- **Exit conditions**: User adjusts inputs and retries, or abandons
- **Transitions to**: [STATE-001](#STATE-001), [STATE-002](#STATE-002), or [STATE-003](#STATE-003) (user adjusts inputs)
- **Resumable**: yes

---

## Terminal States

| State | Type | Description |
|---|---|---|
| STATE-005 | success | User has quotes to review |
| STATE-006 | failure (recoverable) | No quotes returned, user can adjust and retry |
| Abandoned | abandoned | User left without completing — data persists for 30 days |

---

## Flow Diagram

```
[STATE-001: DriverDetails] → [STATE-002: VehicleDetails] → [STATE-003: CoverDetails]
                                                                      ↓
                                                            [STATE-004: Calculation]
                                                              ↓              ↓
                                                    [STATE-005: Results]  [STATE-006: NoQuotes]
                                                                              ↓
                                                                    [Back to STATE-001/002/003]
```
