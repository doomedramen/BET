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
| Data Model (Transformation) | `TRANS-` | `TRANS-001` |
| Permissions | `PERM-` | `PERM-001` |
| Validation Rules | `VR-` | `VR-001` |
| Business Rules | `BR-` | `BR-001` |
| Presentation Rules | `PR-` | `PR-001` |
| Flow States | `STATE-` | `STATE-001` |
| Conditional Logic | `COND-` | `COND-001` |
| Constraints | `CON-` | `CON-001` |
| Rate Limits | `RATE-` | `RATE-001` |
| External Interactions | `EXT-` | `EXT-001` |
| Events | `EVT-` | `EVT-001` |
| Messages | `MSG-` | `MSG-001` |
| Help Text | `HELP-` | `HELP-001` |
| Legal Text | `LEGAL-` | `LEGAL-001` |
| Content | `CONTENT-` | `CONTENT-001` |
| Errors | `ERR-` | `ERR-001` |
| Edge Cases | `EDGE-` | `EDGE-001` |
| Interaction Behaviour (Loading) | `LOADING-` | `LOADING-001` |
| Accessibility (Announcement) | `ANNOUNCE-` | `ANNOUNCE-001` |
| Consent | `CONSENT-` | `CONSENT-001` |
| User Rights | `RIGHT-` | `RIGHT-001` |
| Dependencies | `DEP-` | `DEP-001` |
| Non-Functional | `NFR-` | `NFR-001` |
| UI Components | `COMP-` | `COMP-001` |
| Alerts | `ALERT-` | `ALERT-001` |
| Dashboards | `DASH-` | `DASH-001` |
| Onboarding Steps | `ONBOARD-` | `ONBOARD-001` |
| Animations | `ANIM-` | `ANIM-001` |
| Testing Decisions | `TEST-` | `TEST-001` |

## Cross-References

Reference items in other sections using their ID and file:

```
Depends on: [VR-003](04-validation-rules.md)
References: [Driver.dateOfBirth](02-data-model.md#Driver)
Transformation: [TRANS-001](02-data-model.md#TRANS-001)
Rate limit: [RATE-001](08-constraints.md#RATE-001)
Help text: [HELP-001](11-messages.md#HELP-001)
Legal: [LEGAL-001](11-messages.md#LEGAL-001)
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

**Category** values: `data`, `timing`, `concurrency`, `multi-session`, `partial state`, `permissions`, `boundary`

## Data Transformations Format

```markdown
### TRANS-001: Quote Request Assembly

- **Source**: Driver, Vehicle, QuoteRequest (cover details)
- **Target**: Insurance Provider API payload
- **Mapping**:
  | Source field | Target field | Transform |
  |---|---|---|
  | Driver.firstName | applicant.first_name | direct |
  | Driver.dateOfBirth | applicant.dob | formatted to ISO 8601 |
  | Driver.age | (omitted) | not sent — provider calculates from dob |
- **Notes**: Omit null optional fields rather than sending empty values
```

**Transform** values: `direct`, `renamed`, `formatted`, `combined`, `calculated`, `omitted`

## Presentation Rules Format

```markdown
### PR-001: Quote Results List

- **Context**: Quote results displayed after calculation
- **Default sort**: annualPrice ascending
- **Available sort options**: annualPrice, monthlyPrice, provider name, excess
- **Filtering**: coverLevel, excess range
- **Pagination**: All results shown (max 20 providers)
- **Empty state**: [MSG-010](11-messages.md#MSG-010)
```

## Field Visibility Format

```markdown
### STATE-001: CollectDriverDetails

| Field | Default | Condition | Becomes |
|---|---|---|---|
| Driver.firstName | visible + editable | — | — |
| Driver.businessType | hidden | Vehicle.usage == "business" | visible + required |
```

**Editability** values: `editable`, `read-only`, `disabled`, `hidden`

## Rate Limits Format

```markdown
### RATE-001: Quote Request Limit

- **Action**: Submit quote request
- **Limit**: 10 per hour
- **Scope**: per user
- **When exceeded**: Show error message, block submission
- **Recovery**: After time window resets
- **Message**: [MSG-XXX](11-messages.md)
```

## API Contract Format

API contracts are added to external interactions (09). Each service includes:

```markdown
- **API Contract**:
  - **Method**: POST
  - **Endpoint**: /api/v1/vehicle-lookup
  - **Authentication**: API key in header
  - **Request schema**: { "registration": "string" }
  - **Response schema**: { "make": "string", "model": "string", ... }
  - **Error codes**:
    | Code | Meaning | Maps to |
    |---|---|---|
    | 404 | Vehicle not found | COND-001 |
  - **Rate limits**: 100 requests per minute per API key
  - **Data transformation**: [TRANS-001](02-data-model.md#TRANS-001)
```

## Content Format

Section 11 now covers all user-facing text, not just reactive messages:

```markdown
### HELP-001: Cover Type Explainer

- **Context**: Alongside QuoteRequest.coverType field
- **Text**: "Comprehensive cover protects against damage to your vehicle and others."
- **Display**: expandable

### LEGAL-001: Data Processing Consent

- **Location**: STATE-001 (CollectDriverDetails)
- **Text**: "By proceeding, you agree to our processing of your personal data..."
- **Interaction**: must check box
- **Required by**: GDPR

### CONTENT-001: Quote Results Introduction

- **Context**: Top of STATE-005 (QuoteResults)
- **Text**: "Here are your quotes, sorted by price."
- **Purpose**: Orients the user to the results page
```

## Interaction Behaviour Format

```markdown
### LOADING-001: Quote Calculation

- **Trigger**: User submits complete quote request
- **Immediate feedback**: Submit button disabled, spinner shown
- **Threshold**: After 3s, show "Getting your quotes..."
- **Timeout**: 15s — show error message
- **Cancellable**: no
- **Success feedback**: Redirect to quote results
```

## Accessibility Format

```markdown
### ANNOUNCE-001: Validation Error

- **Trigger**: Form submission fails validation
- **Text**: "{count} errors found. Please correct and try again."
- **Priority**: assertive
- **Mechanism**: role="alert"
```

## Data Lifecycle Format

```markdown
### CONSENT-001: Data Processing

- **Data covered**: All Driver and Vehicle fields
- **Purpose**: Quote generation
- **Collection method**: Checkbox before first submission
- **Required**: yes (blocking)
- **Revocable**: yes
- **Text**: [LEGAL-001](11-messages.md#LEGAL-001)
- **Stored as**: timestamp, consent version, IP address

### RIGHT-001: Right to Access

- **What**: User can request all personal data
- **Response format**: JSON download
- **Response time**: Immediate (self-service)
- **Authentication**: Logged-in user session
```

## Testing Strategy Format

```markdown
### TEST-001: [Title]

- **Layer**: [unit | integration | contract | e2e | bdd]
- **Covers**: [spec IDs, e.g. BR-001, VR-003, EXT-001]
- **Decision / Gap**: [What was decided, or why coverage is deferred]
- **Status**: [ ] Implemented  [ ] Deferred  [ ] Out of scope
```

**Layer** values: `unit`, `integration`, `contract`, `e2e`, `bdd`
