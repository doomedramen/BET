# BET Section Guide

How to fill in each section — with tips for both rewrites and new projects.

---

## How to Use This Guide

Each section below covers:
- **What to capture** — what belongs in this section
- **Rewrite mode** — how to extract this from an existing system
- **Greenfield mode** — how to define this from scratch
- **Common mistakes** — what people typically get wrong
- **Red flags** — signs that the section is incomplete

---

## 01 — Capabilities [REQUIRED]

**What to capture**: The top-level goals of the system. Not features, not screens — goals. What does the system allow people to accomplish? Who are the actors?

**Rewrite mode**: Step back from the existing UI. Don't write "there is a form with 5 fields." Write "the system allows a user to provide their personal details for quote generation." Look at the system from the outside.

**Greenfield mode**: Start with user stories, but strip out implementation details. "As a user, I can get an insurance quote based on my personal and vehicle data" — that's a capability. "As a user, I can fill in a form" — that's UI.

**Common mistakes**:
- Listing features instead of goals ("search bar" vs "find products by keyword")
- Forgetting non-human actors (background jobs, scheduled tasks, external systems that call your API)
- Making capabilities too granular (each should be meaningful and independent)

**Red flags**:
- No actors defined
- Capabilities describe UI elements instead of goals
- Everything is one big capability (it should be broken down)

---

## 02 — Data Model [REQUIRED]

**What to capture**: Every piece of data the system touches. Entities, their fields, types, whether required, and crucially — where does each field come from (user input, API, calculated, system-generated)?

**Rewrite mode**: Go through every form, every API response, every database table. If a piece of data is ever entered, displayed, stored, or sent — it belongs here. Don't forget derived data (fields that are calculated from other fields).

**Greenfield mode**: Work from your capabilities. For each capability, ask: "What data does this need?" Build entities from the answers.

**Common mistakes**:
- Missing calculated/derived fields (age from date of birth, total from line items)
- Not specifying data source/ownership — critical for knowing what to validate vs trust
- Forgetting relationships between entities
- Including UI state as data (isFormOpen, currentStep — these are flow concerns, not data)

**Red flags**:
- Fields with no source specified
- No relationships defined between entities
- Data that appears in business rules but isn't in the data model

---

## 03 — Permissions [OPTIONAL]

**What to capture**: Who can do what. Roles, what's visible per role, what's actionable per role. Skip this for single-role systems.

**Rewrite mode**: Log in as different user types. What changes? What's hidden? What's disabled? Document the delta between roles.

**Greenfield mode**: Define roles first, then for each capability ask: "Which roles can do this?"

**Common mistakes**:
- Only documenting what each role can do (also document what they can't)
- Forgetting data-level permissions (can a user see other users' data?)
- Missing permission checks on transitions (who can approve, cancel, escalate?)

**Red flags**:
- Roles defined but no mapping to capabilities or data
- No mention of data-level access control

---

## 04 — Validation Rules [REQUIRED]

**What to capture**: Input correctness rules — separated from business logic. A validation rule checks that data is well-formed. A business rule decides what to do with valid data.

**Rewrite mode**: Submit every form with bad data. Leave fields blank, enter wrong formats, exceed limits. Every error message you see maps to a validation rule. Also check: when does validation trigger? Immediately on input? On submit? On blur?

**Greenfield mode**: For every field in your data model that comes from user input, ask: "What makes this field invalid?"

**Common mistakes**:
- Mixing validation with business rules ("age must be >= 17" is a business rule, "date of birth must be a valid date" is validation)
- Not specifying timing (on input vs on submit changes UX significantly)
- Forgetting cross-field validation ("end date must be after start date")

**Red flags**:
- User-input fields with no validation rules
- No timing specified
- Validation rules that reference business concepts (these belong in business rules)

---

## 05 — Business Rules [REQUIRED]

**What to capture**: The core logic — what the system does with valid data. IF/THEN format. Every rule needs a priority (what if rules conflict?) and an origin (why does this rule exist?).

**Rewrite mode**: This is the hardest section for rewrites. Business rules are often buried in code, not documented anywhere. Look at: conditional logic in controllers/services, database constraints, email triggers, pricing logic, eligibility checks.

**Greenfield mode**: For each capability, ask: "What decisions does the system make?" Each decision is a business rule.

**Common mistakes**:
- Rules without origins — if you don't know why a rule exists, you can't judge edge cases
- No priority — when two rules apply, which wins?
- Hidden rules — logic that "everyone knows" but isn't written down
- Rules that are actually validation (format/presence checks belong in validation)

**Red flags**:
- No blocking rules (every system has hard stops somewhere)
- Rules that reference fields not in the data model
- No origins specified

---

## 06 — Flow / State Model [REQUIRED]

**What to capture**: How the system progresses — states and transitions. Not pages. A state is "what's true right now." A transition is "what must happen to move forward."

**Rewrite mode**: Walk through the system end-to-end. At each point ask: "What's true right now? What must be true to move on? Can I go back?" Map states, not screens.

**Greenfield mode**: For each capability, define the journey from start to completion. What states does the data pass through?

**Common mistakes**:
- Mapping pages instead of states (a page might contain multiple states, or one state might span multiple pages)
- Forgetting resumability — what happens if the user leaves and comes back?
- Not defining entry/exit conditions
- Missing error/cancelled/abandoned states

**Red flags**:
- No resumability defined
- Linear-only flow (real systems always have branches, loops, or dead ends)
- States with no exit conditions

---

## 07 — Conditional Logic [REQUIRED]

**What to capture**: What changes based on context. If a user's answers change what happens next, what fields appear, or what rules apply — that's conditional logic.

**Rewrite mode**: Fill in forms with different values. What changes? Do new fields appear? Do options change? Do different paths open? Each change is a condition.

**Greenfield mode**: For each state and each field, ask: "Does anything change depending on this value?"

**Common mistakes**:
- Not documenting cascades (changing field A affects field B, which affects condition C)
- Conditions that contradict business rules
- Orphaned conditions (triggers that reference fields or states that don't exist)

**Red flags**:
- No cascades documented (most real systems have them)
- Conditions with no clear trigger

---

## 08 — Constraints [REQUIRED]

**What to capture**: Hard stops — things that completely prevent progress. Different from business rules: a business rule might flag something as high risk, a constraint blocks entirely.

**Rewrite mode**: Try to break the system. What completely stops you? Age limits, geo-restrictions, missing required data, expired sessions.

**Greenfield mode**: For each capability, ask: "What would make this impossible?"

**Common mistakes**:
- Constraints without recovery paths (the user needs to know what they can do about it)
- Confusing constraints with validation (validation = bad input, constraint = valid input but ineligible)
- Not linking constraints to messages

**Red flags**:
- Constraints with no recovery path
- No messages linked to constraints

---

## 09 — External Interactions [OPTIONAL]

**What to capture**: Every system boundary — APIs you call, services you depend on. Input/output contracts and crucially: what happens when they fail.

**Rewrite mode**: Find every HTTP call, every queue message, every third-party SDK. Document what goes in and what comes out. Then ask: "What happens when this is slow? Down? Returns unexpected data?"

**Greenfield mode**: For each capability that depends on external data, define the service contract and failure modes.

**Common mistakes**:
- Only documenting the happy path (service responds correctly)
- No timeout values
- No fallback behaviour
- Forgetting internal services (your own microservices are still external interactions)

**Red flags**:
- External services with no failure modes documented
- No fallback behaviour for any service

---

## 10 — Events & Side Effects [OPTIONAL]

**What to capture**: What the system emits — analytics events, audit logs, webhooks, notifications, emails. Things that happen as a consequence of actions.

**Rewrite mode**: Check analytics dashboards, email templates, notification systems, audit logs. Every event the system produces is a side effect.

**Greenfield mode**: For each state transition and each business rule outcome, ask: "Should anything else happen?"

**Common mistakes**:
- Forgetting audit/compliance events
- Not specifying when events fire (before or after the action?)
- Missing failure notifications (system errors that should alert someone)

**Red flags**:
- No events defined for any state transitions (unlikely in a real system)

---

## 11 — Messages [REQUIRED]

**What to capture**: Every piece of user-facing text. Error messages, success confirmations, warnings, info text. Each message needs a trigger condition and severity level.

**Rewrite mode**: Screenshot every message you see. Error states, empty states, success states, loading states. Every piece of text that reacts to system state is a message.

**Greenfield mode**: For every validation rule, business rule, constraint, and edge case, ask: "What does the user see?"

**Common mistakes**:
- Messages without triggers (when does this show?)
- Hardcoded values instead of parameters ("17" instead of "{minAge}")
- Missing severity levels
- Forgetting empty states and loading states

**Red flags**:
- Business rules or constraints with no linked messages
- Messages with no trigger condition

---

## 12 — Error Taxonomy [OPTIONAL]

**What to capture**: Categories of errors and how each is handled. User errors (bad input), system errors (bugs, crashes), external errors (API failures). Retry logic, fallback behaviour.

**Rewrite mode**: Check error handling code, error boundaries, try/catch blocks, error logging. How does the system categorise and handle failures?

**Greenfield mode**: Define your error categories upfront. For each category, define the handling strategy.

**Common mistakes**:
- Treating all errors the same
- No retry logic for transient failures
- No distinction between "show the user" and "log for ops"

**Red flags**:
- No error categories defined
- No retry or fallback strategies

---

## 13 — Edge Cases [REQUIRED]

**What to capture**: Everything that could go wrong, categorised. Data edge cases (null, empty, max length), timing (race conditions, expired sessions), concurrency (two users editing the same thing), partial states (interrupted flows), permission boundaries (role changes mid-flow).

**Rewrite mode**: This is where rewrites fail most often. The original system handles edge cases in code that nobody remembers writing. Go through every state, every transition, every external call and ask: "What if this goes wrong?"

**Greenfield mode**: For each section you've filled in, ask: "What's the weird case?" Every rule has an edge. Every flow has an interruption.

**Common mistakes**:
- Only thinking about data edge cases (forgetting timing, concurrency, permissions)
- Not categorising (makes it hard to ensure coverage)
- "We'll handle that later" — you won't

**Red flags**:
- No edge cases for external interactions (they always fail eventually)
- No partial state handling (users always abandon flows)
- Fewer than 5 edge cases total (you haven't looked hard enough)

---

## 14 — Dependencies [OPTIONAL]

**What to capture**: What must be true for the system to work. External systems that must exist, data that must be pre-populated, infrastructure assumptions.

**Rewrite mode**: What breaks if you move this system to a new environment? What data was seeded manually? What services are assumed to be running?

**Greenfield mode**: List everything you're taking for granted. Auth system exists? Payment provider is set up? Reference data is loaded?

**Common mistakes**:
- Assuming infrastructure (database, cache, queue) without documenting it
- Forgetting seed data / reference data
- Not specifying versions or compatibility requirements

**Red flags**:
- No dependencies listed (every system depends on something)

---

## 15 — Non-Functional Requirements [OPTIONAL]

**What to capture**: Performance expectations, availability, data retention, security requirements. The "how well" rather than the "what."

**Rewrite mode**: Check SLAs, monitoring dashboards, performance budgets. What are the current benchmarks? What's the expected load?

**Greenfield mode**: Define acceptable performance for each capability. How fast? How available? How long is data kept?

**Common mistakes**:
- Vague requirements ("must be fast" — how fast?)
- Missing data retention / deletion requirements (especially for GDPR/compliance)
- No load expectations

**Red flags**:
- No specific numbers (every NFR should be measurable)
