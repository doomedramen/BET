# BET Completeness Checklist

Use this checklist to validate that your spec is done. A spec is complete when a developer could build the system from it without asking questions.

---

## Master Test

- [ ] Could a developer build this system without asking me questions?
- [ ] Could a tester write tests without seeing the code?
- [ ] If I deleted the frontend, could I rebuild it from this spec?
- [ ] Are all failure modes accounted for?
- [ ] Do I know what happens at every system boundary?
- [ ] Do I know what every user-facing piece of text says?
- [ ] Do I know how the system behaves for keyboard-only and screen reader users?
- [ ] Do I know how data is collected, retained, and deleted?

---

## Per-Section Checklists

### 01 — Capabilities

- [ ] Every capability has a clear goal (not a feature description)
- [ ] All actors are named (users, roles, systems, background processes)
- [ ] Each capability is independent and testable
- [ ] No capability describes UI elements

### 02 — Data Model

- [ ] Every piece of data the system touches has an entity and field
- [ ] Every field has a type, required/optional, and source
- [ ] Calculated/derived fields are documented with their formula
- [ ] Relationships between entities are defined
- [ ] No field exists that isn't referenced somewhere else in the spec
- [ ] Data transformations are documented for every external interaction
- [ ] Data transformations are documented for every calculated entity

### 03 — Permissions (if applicable)

- [ ] All roles are defined
- [ ] Each role has explicit can/cannot mappings to capabilities
- [ ] Data-level access is documented (who sees whose data?)
- [ ] Permission checks on state transitions are documented

### 04 — Validation Rules

- [ ] Every user-input field has at least one validation rule
- [ ] Each validation rule specifies timing (on input, on submit, etc.)
- [ ] Cross-field validations are documented
- [ ] Each validation rule links to a message
- [ ] No validation rule contains business logic

### 05 — Business Rules

- [ ] Every rule uses IF/THEN/ELSE format
- [ ] Every rule has a priority
- [ ] Every rule has an origin (legal, business, technical, ux)
- [ ] Every rule references fields that exist in the data model
- [ ] Conflicting rules have clear priority ordering
- [ ] No rule is actually a validation rule (format/presence checks)
- [ ] Every state that presents a list has a presentation rule (sort, filter, pagination)
- [ ] Every presentation rule defines an empty state

### 06 — Flow / State Model

- [ ] Every state has entry conditions
- [ ] Every state has exit conditions
- [ ] Every state specifies resumability
- [ ] Terminal states are identified (success, failure, abandoned)
- [ ] The flow is not purely linear (branches, loops, or dead ends exist)
- [ ] Every transition references what must be true to proceed

### 07 — Conditional Logic

- [ ] Every condition has a clear trigger
- [ ] Cascading effects are documented (A changes B changes C)
- [ ] No condition contradicts a business rule
- [ ] Every condition references fields/states that exist
- [ ] Field visibility tables exist for every state with user-facing fields
- [ ] Every data model field appears in at least one visibility table
- [ ] Read-only fields are documented (e.g., API-sourced data)

### 08 — Constraints

- [ ] Every constraint specifies a recovery path (even if "none")
- [ ] Every constraint links to a message
- [ ] No constraint is actually a validation rule
- [ ] Every constraint references the state(s) it blocks
- [ ] Rate limits are defined for user-facing form submissions
- [ ] Rate limits are defined for external API calls
- [ ] Every rate limit has a message and recovery path

### 09 — External Interactions (if applicable)

- [ ] Every service has input/output documented
- [ ] Every service has failure modes (timeout, down, bad data)
- [ ] Every service has fallback behaviour
- [ ] Timeout values are specified
- [ ] Every service has an API contract (method, endpoint, auth, schemas)
- [ ] Every API error code maps to a system behaviour
- [ ] Every service has a data transformation linking it to the data model

### 10 — Events & Side Effects (if applicable)

- [ ] Every state transition has been considered for events
- [ ] Event timing is specified (before/after action)
- [ ] Failure notifications are documented

### 11 — Content & Messages

- [ ] Every message has a trigger condition
- [ ] Every message has a severity (error, warning, info, success)
- [ ] Dynamic content uses parameters (`{paramName}`)
- [ ] Every business rule and constraint has a linked message
- [ ] Empty states and loading states have messages
- [ ] Fields where the label alone is ambiguous have help text
- [ ] Legal/compliance text is documented for systems collecting personal data
- [ ] Contextual content (explainers) exists for domain-specific options
- [ ] Help text specifies display mode (always visible, on focus, expandable)
- [ ] Legal text specifies required interaction (acknowledge, checkbox, scroll)

### 12 — Error Taxonomy (if applicable)

- [ ] Error categories are defined (user, system, external)
- [ ] Each category has a handling strategy
- [ ] Retry logic is documented for transient errors
- [ ] "Show user" vs "log for ops" distinction is clear

### 13 — Edge Cases

- [ ] At least 5 edge cases documented (if fewer, you haven't looked hard enough)
- [ ] Edge cases are categorised (data, timing, concurrency, multi-session, partial state, permissions, boundary)
- [ ] Every external interaction has at least one edge case
- [ ] Partial state / abandoned flow is addressed
- [ ] Each edge case specifies expected behaviour
- [ ] Multi-session scenarios are covered (same user, multiple tabs/devices)
- [ ] Cross-device continuity is addressed (if applicable)

### 14 — Dependencies (if applicable)

- [ ] All external systems are listed
- [ ] Seed / reference data requirements are documented
- [ ] Infrastructure assumptions are explicit

### 15 — Non-Functional (if applicable)

- [ ] Every requirement has a measurable number
- [ ] Performance expectations are defined per capability
- [ ] Data retention / deletion rules are specified

### 16 — Interaction Behaviour

- [ ] Error display placement is defined (inline, summary, toast, modal)
- [ ] Multiple-error behaviour is defined (show all vs first)
- [ ] Error clear timing is defined (on input change, on re-submit)
- [ ] Every async operation has loading feedback defined
- [ ] Loading thresholds are specified (when does a spinner appear?)
- [ ] Timeout behaviour is defined for every async operation
- [ ] Auto-save behaviour is defined (or explicitly not used)
- [ ] Double-submission is prevented for form actions
- [ ] Navigation behaviour is defined (back, browser back, deep link)
- [ ] Unsaved changes prompt is defined (or explicitly not needed)
- [ ] Confirmation patterns are defined for destructive/irreversible actions

### 17 — Accessibility

- [ ] WCAG compliance target is specified
- [ ] Tab order is defined for every state with interactive elements
- [ ] Focus management is defined for every state transition
- [ ] Focus management is defined for every dynamic content change (errors, modals)
- [ ] Focus trapping is defined for modals and dialogs
- [ ] Screen reader announcements are defined for dynamic content changes
- [ ] Form fields have programmatic labels and error associations
- [ ] Required fields are indicated both visually and programmatically
- [ ] Skip navigation is defined
- [ ] Landmark regions are defined
- [ ] Alternative content is defined for non-text elements

### 18 — Data Lifecycle & Compliance

- [ ] Consent requirements are documented for data collection
- [ ] Retention periods are defined for every entity
- [ ] Deletion mechanism is defined (soft, hard, anonymise)
- [ ] Right to access is implemented (user can get their data)
- [ ] Right to deletion is implemented (user can delete their data)
- [ ] Cascading deletion is documented (what happens to related data)
- [ ] Audit trail covers creation, modification, deletion, and export
- [ ] Data is classified by sensitivity (PII, sensitive, internal)
- [ ] Handling rules exist per classification (encryption, masking, logging)
- [ ] Applicable regulations are identified and mapped to requirements

### 19 — UI Specification (if applicable)

- [ ] Design tokens are defined (colours, typography, spacing)
- [ ] Page layouts are defined for each state
- [ ] Component states are documented (default, hover, focus, disabled, error, loading)
- [ ] Responsive breakpoints and behaviour are defined

### 20 — Technical Architecture (if applicable)

- [ ] Technology stack is defined with versions
- [ ] Database schema maps to data model (divergences explained)
- [ ] API endpoints are defined with request/response schemas
- [ ] Session management strategy is documented
- [ ] Security architecture covers OWASP top 10

### 21 — SEO (if applicable)

- [ ] URL structure is defined for all public pages
- [ ] Page metadata (title, description) is defined
- [ ] Indexing rules distinguish public vs private pages
- [ ] Canonical URLs are defined for pages with variants

### 22 — Localization (if applicable)

- [ ] Default locale is defined with all formatting rules
- [ ] Field-specific formats are documented (input, display, storage)
- [ ] Pluralisation rules are defined for dynamic text

### 23 — Animation (if applicable)

- [ ] Reduced-motion behaviour is defined
- [ ] State transitions have animation specs (or explicitly none)
- [ ] Loading animations have appearance thresholds

### 24 — Monitoring (if applicable)

- [ ] Key metrics are defined with alert thresholds
- [ ] Alerts have severity and notification targets
- [ ] Logging levels are defined with examples
- [ ] Sensitive data is excluded from logs
- [ ] Health check endpoints are defined

### 25 — Onboarding (if applicable)

- [ ] First-time experience is defined (or explicitly same as returning)
- [ ] Empty states have content and calls to action
- [ ] Onboarding is skippable and dismissable

### 26 — Multi-Channel (if applicable)

- [ ] Supported channels are listed
- [ ] Behavioural differences per channel are documented
- [ ] Cross-channel state sync is defined
- [ ] Channel-specific entry points are documented

### 27 — Testing Strategy (if applicable)

- [ ] Test framework / tooling is decided for each layer (unit, integration, contract, e2e)
- [ ] Every required spec section is assigned to at least one test layer
- [ ] Every `VR-XXX` validation rule has a planned unit test
- [ ] Every `BR-XXX` business rule has a planned unit test (both branches)
- [ ] Every `EXT-XXX` external interaction has a planned contract or integration test
- [ ] Every `EDGE-XXX` edge case has a planned E2E test
- [ ] Known gaps are documented with `TEST-XXX` entries and a reason
- [ ] How to run tests locally and in CI is documented

---

## Cross-Section Consistency

These checks ensure sections reference each other correctly:

- [ ] Every field in business rules exists in the data model
- [ ] Every field in validation rules exists in the data model
- [ ] Every state referenced in conditions exists in the flow model
- [ ] Every message referenced in rules/constraints exists in messages
- [ ] Every external service failure has an edge case
- [ ] Every constraint references a state it blocks
- [ ] Every condition references fields or states that exist
- [ ] No orphaned items (entries in one section that nothing references)
- [ ] Every external interaction has a corresponding data transformation
- [ ] Every data model field appears in at least one field visibility table
- [ ] Every field visibility condition references a valid condition or field value
- [ ] Every rate limit has a corresponding message
- [ ] Every API contract error code maps to a condition, constraint, or fallback
- [ ] Every help text references a field that exists in the data model
- [ ] Every legal text references the state where it appears
- [ ] Every consent item references the data it covers
- [ ] Every loading pattern references the async operation it covers
- [ ] Every accessibility announcement references its trigger event
- [ ] Every presentation rule references the state where the list appears
