# BET Completeness Checklist

Use this checklist to validate that your spec is done. A spec is complete when a developer could build the system from it without asking questions.

---

## Master Test

- [ ] Could a developer build this system without asking me questions?
- [ ] Could a tester write tests without seeing the code?
- [ ] If I deleted the frontend, could I rebuild it from this spec?
- [ ] Are all failure modes accounted for?
- [ ] Do I know what happens at every system boundary?

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

### 08 — Constraints

- [ ] Every constraint specifies a recovery path (even if "none")
- [ ] Every constraint links to a message
- [ ] No constraint is actually a validation rule
- [ ] Every constraint references the state(s) it blocks

### 09 — External Interactions (if applicable)

- [ ] Every service has input/output documented
- [ ] Every service has failure modes (timeout, down, bad data)
- [ ] Every service has fallback behaviour
- [ ] Timeout values are specified

### 10 — Events & Side Effects (if applicable)

- [ ] Every state transition has been considered for events
- [ ] Event timing is specified (before/after action)
- [ ] Failure notifications are documented

### 11 — Messages

- [ ] Every message has a trigger condition
- [ ] Every message has a severity (error, warning, info, success)
- [ ] Dynamic content uses parameters (`{paramName}`)
- [ ] Every business rule and constraint has a linked message
- [ ] Empty states and loading states have messages

### 12 — Error Taxonomy (if applicable)

- [ ] Error categories are defined (user, system, external)
- [ ] Each category has a handling strategy
- [ ] Retry logic is documented for transient errors
- [ ] "Show user" vs "log for ops" distinction is clear

### 13 — Edge Cases

- [ ] At least 5 edge cases documented (if fewer, you haven't looked hard enough)
- [ ] Edge cases are categorised (data, timing, concurrency, partial state, permissions, boundary)
- [ ] Every external interaction has at least one edge case
- [ ] Partial state / abandoned flow is addressed
- [ ] Each edge case specifies expected behaviour

### 14 — Dependencies (if applicable)

- [ ] All external systems are listed
- [ ] Seed / reference data requirements are documented
- [ ] Infrastructure assumptions are explicit

### 15 — Non-Functional (if applicable)

- [ ] Every requirement has a measurable number
- [ ] Performance expectations are defined per capability
- [ ] Data retention / deletion rules are specified

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
