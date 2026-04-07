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

## Tagging Convention

Use spec IDs as tags for traceability:

```gherkin
@BR-001 @CAP-001 @STATE-001
Scenario: ...
```

This allows you to trace any failing test back to the exact spec item it validates.
