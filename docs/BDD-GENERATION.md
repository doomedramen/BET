# BDD Generation Rules

How to turn a complete BET spec into Given/When/Then scenarios. Once the spec is filled in, this process is almost mechanical.

---

## Principles

1. **One scenario per rule/case** — don't combine multiple behaviours into one scenario
2. **Use spec IDs in scenario tags** — `@BR-001` links the scenario back to its source
3. **Scenarios describe behaviour, not UI** — "the system blocks progression" not "the button is greyed out"
4. **Every REQUIRED section should produce at least one scenario**

---

## Conversion Patterns

### Business Rules → Scenarios

Every business rule in IF/THEN format maps directly to at least two scenarios: the positive case and the negative case.

**Source (05-business-rules.md):**
```
BR-001: Minimum Driver Age
IF Driver.age < 17 THEN reject quote request
```

**Output:**
```gherkin
@BR-001
Scenario: Underage driver is rejected
  Given a driver with age less than 17
  When the system evaluates eligibility
  Then the quote request is rejected
  And the message "You must be at least 17 years old to get a quote." is displayed

@BR-001
Scenario: Driver of eligible age proceeds
  Given a driver with age 17 or above
  When the system evaluates eligibility
  Then the quote request is not rejected
```

### Validation Rules → Scenarios

Each validation rule produces a failing scenario and a passing scenario.

**Source (04-validation-rules.md):**
```
VR-001: Date of Birth Valid Date
Field: Driver.dateOfBirth
Rule: Must be a valid date in the past
Timing: on input
```

**Output:**
```gherkin
@VR-001
Scenario: Invalid date of birth is rejected on input
  Given a driver enters an invalid date for dateOfBirth
  When the field loses focus
  Then a validation error is displayed
  And the user cannot proceed

@VR-001
Scenario: Valid date of birth is accepted
  Given a driver enters a valid past date for dateOfBirth
  When the field loses focus
  Then no validation error is displayed
```

### Flow States → Scenarios

Each state transition produces a scenario. Focus on entry conditions and exit conditions.

**Source (06-flow-state.md):**
```
STATE-001: CollectDriverDetails
Exit conditions: All required Driver fields pass validation
Transitions to: STATE-002 (CollectVehicleDetails)
Resumable: yes — data persists for 30 days
```

**Output:**
```gherkin
@STATE-001
Scenario: Proceed from driver details to vehicle details
  Given the user is in the CollectDriverDetails state
  And all required driver fields are valid
  When the user attempts to proceed
  Then the system transitions to CollectVehicleDetails

@STATE-001
Scenario: Cannot proceed with incomplete driver details
  Given the user is in the CollectDriverDetails state
  And required driver fields are missing or invalid
  When the user attempts to proceed
  Then the system remains in CollectDriverDetails
  And validation errors are displayed

@STATE-001
Scenario: Driver details are resumable
  Given the user has partially completed CollectDriverDetails
  When the user returns within 30 days
  Then the previously entered driver data is preserved
```

### Conditional Logic → Scenarios

Each condition produces a scenario for when the condition is true and when it's false.

**Source (07-conditional-logic.md):**
```
COND-001: Vehicle Not Found
Trigger: Vehicle registration lookup returns no results
Outcome: Require manual vehicle entry
```

**Output:**
```gherkin
@COND-001
Scenario: Vehicle not found triggers manual entry
  Given a user enters a registration number
  When the vehicle lookup returns no results
  Then the system requires manual vehicle entry
  And fields for make, model, year, and value are available

@COND-001
Scenario: Vehicle found auto-populates details
  Given a user enters a registration number
  When the vehicle lookup returns results
  Then the vehicle details are auto-populated
```

### Constraints → Scenarios

Each constraint produces a blocking scenario with recovery path (or lack thereof).

**Source (08-constraints.md):**
```
CON-001: Ineligible Driver (Underage)
Condition: Driver.age < 17
Effect: Prevent progression past CollectDriverDetails
Recovery: None
```

**Output:**
```gherkin
@CON-001
Scenario: Underage driver is blocked with no recovery
  Given a driver with age less than 17
  When they attempt to proceed past driver details
  Then progression is blocked
  And no alternative path is offered
  And the underage error message is displayed
```

### External Interactions → Scenarios

Each failure mode produces a scenario.

**Source (09-external-interactions.md):**
```
EXT-001: Vehicle Lookup Service
Failure modes:
  - Timeout (>5s): Show manual entry form
  - Not found: Trigger COND-001
  - Service down: Show manual entry form, alert ops
```

**Output:**
```gherkin
@EXT-001
Scenario: Vehicle lookup times out
  Given a user enters a registration number
  When the vehicle lookup takes longer than 5 seconds
  Then the manual entry form is shown
  And a warning is logged

@EXT-001
Scenario: Vehicle lookup service is down
  Given a user enters a registration number
  When the vehicle lookup service is unavailable
  Then the manual entry form is shown
  And the ops team is alerted
```

### Edge Cases → Scenarios

Each edge case maps to a scenario describing the expected behaviour.

**Source (13-edge-cases.md):**
```
EDGE-001: Partial Data on Exit
Category: partial state
Scenario: User closes browser mid-flow
Expected: Data persists, user can resume from last valid state
```

**Output:**
```gherkin
@EDGE-001
Scenario: Partial data persists on browser close
  Given a user has entered partial data in a flow
  When the browser is closed
  And the user returns later
  Then the previously entered data is preserved
  And the user resumes from the last valid state
```

### Field Visibility → Scenarios

Each field visibility rule produces a scenario for the default state and the conditional state.

**Source (07-conditional-logic.md):**
```
STATE-002: CollectVehicleDetails
| Field | Default | Condition | Becomes |
| Vehicle.businessType | hidden | Vehicle.usage == "business" | visible + required |
```

**Output:**
```gherkin
@COND-001 @STATE-002
Scenario: Business type field hidden by default
  Given the user is in CollectVehicleDetails
  And Vehicle.usage is not "business"
  Then the businessType field is not available

@COND-001 @STATE-002
Scenario: Business type field shown for business use
  Given the user is in CollectVehicleDetails
  And Vehicle.usage is "business"
  Then the businessType field is available and required
```

### Rate Limits → Scenarios

Each rate limit produces a scenario for normal use and exceeded use.

**Source (08-constraints.md):**
```
RATE-001: Quote Request Limit
Action: Submit quote request
Limit: 10 per hour per user
When exceeded: Block submission, show message
```

**Output:**
```gherkin
@RATE-001
Scenario: Quote requests within rate limit succeed
  Given a user has submitted fewer than 10 quote requests this hour
  When they submit a quote request
  Then the request is processed normally

@RATE-001
Scenario: Quote requests exceeding rate limit are blocked
  Given a user has submitted 10 quote requests this hour
  When they attempt to submit another quote request
  Then the submission is blocked
  And the rate limit message is displayed
```

### Presentation Rules → Scenarios

Each presentation rule produces scenarios for sorting, filtering, and empty states.

**Source (05-business-rules.md):**
```
PR-001: Quote Results List
Default sort: annualPrice ascending
Empty state: MSG-010
```

**Output:**
```gherkin
@PR-001
Scenario: Quote results are sorted by price ascending by default
  Given quote results have been returned
  When the results are displayed
  Then quotes are ordered by annual price from lowest to highest

@PR-001
Scenario: Quote results empty state
  Given no quotes match the current criteria
  When the results are displayed
  Then the no-quotes message is shown
```

### Interaction Behaviour → Scenarios

Loading patterns, error display, and navigation behaviour produce scenarios.

**Source (16-interaction-behaviour.md):**
```
LOADING-001: Quote Calculation
Trigger: User submits complete quote request
Immediate feedback: Submit button disabled, spinner shown
Timeout: 15s — show error message
```

**Output:**
```gherkin
@LOADING-001
Scenario: Quote calculation shows loading feedback
  Given the user has submitted a complete quote request
  When the submission is processing
  Then the submit button is disabled
  And a loading indicator is shown

@LOADING-001
Scenario: Quote calculation timeout shows error
  Given the user has submitted a complete quote request
  When the calculation takes longer than 15 seconds
  Then an error message is displayed
```

### Accessibility → Scenarios

Focus management, keyboard navigation, and screen reader announcements produce scenarios.

**Source (17-accessibility.md):**
```
Focus Management:
| Event | Focus moves to |
| Error on submit | Error summary |

ANNOUNCE-001: Validation Error
Trigger: Form submission fails validation
Text: "{count} errors found."
Priority: assertive
```

**Output:**
```gherkin
@ANNOUNCE-001
Scenario: Focus moves to error summary on failed submission
  Given the user is completing a form
  When they submit with validation errors
  Then focus moves to the error summary
  And a screen reader announces the number of errors

Scenario: All form fields are reachable via keyboard
  Given the user is in CollectDriverDetails
  When they navigate using only the Tab key
  Then every required field can be reached and edited
```

### Data Lifecycle → Scenarios

Consent, user rights, and retention produce scenarios.

**Source (18-data-lifecycle.md):**
```
CONSENT-001: Data Processing
Required: yes (blocking)
Collection method: Checkbox before first submission

RIGHT-002: Right to Deletion
Cascading: Delete Driver → delete all their QuoteRequests
```

**Output:**
```gherkin
@CONSENT-001
Scenario: User cannot proceed without data processing consent
  Given the user has not checked the data processing consent box
  When they attempt to submit their details
  Then submission is blocked

@RIGHT-002
Scenario: User deletion cascades to related data
  Given a user requests deletion of their data
  When the deletion is processed
  Then the user's Driver record is deleted
  And all associated QuoteRequests are deleted
```

---

## Coverage Rules

A complete BDD suite generated from BET should have:

- [ ] At least 2 scenarios per business rule (positive + negative)
- [ ] At least 2 scenarios per validation rule (valid + invalid)
- [ ] At least 1 scenario per state transition
- [ ] At least 2 scenarios per condition (true + false)
- [ ] At least 1 scenario per constraint
- [ ] At least 1 scenario per external service failure mode
- [ ] At least 1 scenario per edge case
- [ ] At least 2 scenarios per field visibility rule (default + conditional)
- [ ] At least 2 scenarios per rate limit (within limit + exceeded)
- [ ] At least 1 scenario per presentation rule
- [ ] At least 1 scenario per loading pattern
- [ ] At least 1 scenario per accessibility announcement
- [ ] At least 1 scenario per consent requirement
- [ ] At least 1 scenario per user right (access, deletion, portability)

## Tagging Convention

Use spec IDs as tags for traceability:

```gherkin
@BR-001 @CAP-001 @STATE-001
Scenario: ...
```

This allows you to trace any failing test back to the exact spec item it validates.
