# BET Format Conventions

Rules for writing consistent, unambiguous BET specs.

---

## General Rules

- One concept per file. Each spec section lives in its own numbered file.
- Use the exact headings and structure provided in each template. Add entries under them, don't restructure.
- Every item must have a unique **ID** within its section (e.g., `BR-001`, `VR-003`, `MSG-012`). These IDs are used for cross-references.
- Mark items as `[REQUIRED]` or `[OPTIONAL]` where the template indicates.

## ID Prefixes

| Section | Prefix | Example |
|---|---|---|
| Capabilities | `CAP-` | `CAP-001` |
| Data Model (Entity) | `ENT-` | `ENT-001` |
| Data Model (Field) | `entity.fieldName` | `Driver.dateOfBirth` |
| Permissions | `PERM-` | `PERM-001` |
| Validation Rules | `VR-` | `VR-001` |
| Business Rules | `BR-` | `BR-001` |
| Flow States | `STATE-` | `STATE-001` |
| Conditional Logic | `COND-` | `COND-001` |
| Constraints | `CON-` | `CON-001` |
| External Interactions | `EXT-` | `EXT-001` |
| Events | `EVT-` | `EVT-001` |
| Messages | `MSG-` | `MSG-001` |
| Errors | `ERR-` | `ERR-001` |
| Edge Cases | `EDGE-` | `EDGE-001` |
| Dependencies | `DEP-` | `DEP-001` |
| Non-Functional | `NFR-` | `NFR-001` |

## Cross-References

Reference items in other sections using their ID and file:

```
Depends on: [VR-003](04-validation-rules.md)
References: [Driver.dateOfBirth](02-data-model.md#Driver)
```

## Data Model Format

Entities use tables:

```markdown
### ENT-001: Driver

| Field | Type | Required | Source | Description |
|---|---|---|---|---|
| firstName | string | yes | user input | Driver's first name |
| dateOfBirth | date | yes | user input | Used for age calculation |
| age | number | yes | calculated | Derived from dateOfBirth |
```

**Source** values: `user input`, `api`, `calculated`, `system-generated`

### Relationships

```markdown
#### Relationships

- Driver HAS MANY Vehicles
- Vehicle BELONGS TO Driver
- Quote REFERENCES Driver, Vehicle
```

## Validation Rules Format

```markdown
### VR-001: Date of Birth Valid Date

- **Field**: [Driver.dateOfBirth](02-data-model.md#Driver)
- **Rule**: Must be a valid date in the past
- **Timing**: on input
- **Message**: [MSG-001](11-messages.md)
```

**Timing** values: `on input`, `on submit`, `on state transition`, `on save`

## Business Rules Format

Use IF/THEN/ELSE:

```markdown
### BR-001: Minimum Driver Age

- **Priority**: 1 (blocking)
- **Origin**: legal
- **Rule**:
  IF Driver.age < 17
  THEN reject quote request
  AND trigger [CON-001](08-constraints.md)
- **References**: [Driver.age](02-data-model.md#Driver)
```

**Priority** values: `1 (blocking)`, `2 (high)`, `3 (medium)`, `4 (low)`

**Origin** values: `legal`, `business`, `technical`, `ux`

## Flow / State Format

```markdown
### STATE-001: CollectDriverDetails

- **Entry conditions**: User has started a new quote
- **Required data**: Driver.firstName, Driver.lastName, Driver.dateOfBirth
- **Exit conditions**: All required Driver fields pass validation
- **Transitions to**: [STATE-002](06-flow-state.md#STATE-002) (CollectVehicleDetails)
- **Resumable**: yes — data persists for 30 days
```

## Conditional Logic Format

```markdown
### COND-001: Vehicle Not Found

- **Trigger**: Vehicle registration lookup returns no results
- **Outcome**: Require manual vehicle entry (show fields: make, model, year, value)
- **Cascades**: Disables auto-populated vehicle fields
- **References**: [EXT-001](09-external-interactions.md)
```

## Constraints Format

```markdown
### CON-001: Ineligible Driver (Underage)

- **Condition**: Driver.age < 17
- **Effect**: Prevent progression past [STATE-001](06-flow-state.md#STATE-001)
- **Recovery**: None — user cannot proceed
- **Message**: [MSG-001](11-messages.md)
```

## External Interactions Format

```markdown
### EXT-001: Vehicle Lookup Service

- **Input**: Driver.registrationNumber
- **Output**: make, model, year, engineSize, fuelType
- **Failure modes**:
  - Timeout (>5s): Show manual entry form, log warning
  - Not found: Trigger [COND-001](07-conditional-logic.md#COND-001)
  - Service down: Show manual entry form, alert ops team
- **Fallback**: Manual vehicle entry
```

## Messages Format

```markdown
### MSG-001: Underage Error

- **Severity**: error
- **Trigger**: [BR-001](05-business-rules.md#BR-001)
- **Text**: "You must be at least {minAge} years old to get a quote."
- **Parameters**: minAge = 17
```

**Severity** values: `error`, `warning`, `info`, `success`

Use `{parameterName}` for dynamic content.

## Edge Cases Format

```markdown
### EDGE-001: Partial Data on Exit

- **Category**: partial state
- **Scenario**: User closes browser mid-flow
- **Expected behaviour**: Data persists, user can resume from last valid state
- **References**: [STATE-001](06-flow-state.md#STATE-001) resumability
```

**Category** values: `data`, `timing`, `concurrency`, `partial state`, `permissions`, `boundary`
