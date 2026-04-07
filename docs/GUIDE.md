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

### Data Transformations (subsection)

**What to capture**: How data maps between entities, or between entities and external service payloads. If data is reshaped, renamed, formatted, combined, or omitted when crossing a boundary, document the mapping.

**Common mistakes**:
- Assuming field names are the same everywhere (your `dateOfBirth` might be the API's `dob`)
- Not documenting omitted fields (what gets dropped and why)
- Not documenting formatting changes (date stored as ISO, sent as DD/MM/YYYY)

**Red flags**:
- External interactions with no corresponding transformation
- Calculated entities with no transformation documenting their assembly

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

### Presentation Rules (subsection)

**What to capture**: How lists, collections, and results are sorted, filtered, and paginated. These are behavioural decisions — "quotes are sorted by price ascending" is not UI, it's logic.

**Common mistakes**:
- No default sort defined (the developer has to guess)
- No empty state for when the list has zero items
- Not specifying pagination for potentially large lists

**Red flags**:
- Any state that presents a list with no presentation rule

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

### Field Visibility & Editability (subsection)

**What to capture**: For each state, which fields are visible, hidden, read-only, or conditionally required. This is the bridge between the data model (what fields exist), flow state (what state we're in), and conditional logic (what changes based on context).

**Common mistakes**:
- Only documenting visible fields (also document what's hidden and why)
- Not covering the read-only case (fields auto-populated from an API should be read-only)
- Fields that appear in the data model but aren't in any visibility table

**Red flags**:
- No visibility tables for any state
- Fields that are editable when they should be read-only (API-sourced data)
- Fields that appear in the data model but are not mapped to any state's visibility

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

### Rate Limits & Abuse Prevention (subsection)

**What to capture**: Limits on how frequently actions can be performed — per user, per session, per IP, or globally. Covers both user-facing rate limits and protection for external service dependencies.

**Common mistakes**:
- No rate limiting at all (every public-facing system needs it)
- Rate limits with no message (user doesn't know why they're blocked)
- No recovery path (when can they try again?)

**Red flags**:
- A system that calls external APIs with no rate limiting
- Form submission with no double-submit protection

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

### API Contracts (subsection)

**What to capture**: The actual shape of requests and responses — method, endpoint, authentication, request/response schemas, error codes, provider-imposed rate limits. This goes beyond "what data goes in and out" to "what does the actual API call look like?"

**Common mistakes**:
- Only documenting the happy path response (error responses need schemas too)
- Not documenting authentication method
- Not linking API error codes to system behaviour (what does a 429 trigger?)

**Red flags**:
- External services with no request/response schema
- No authentication method documented
- Error codes with no mapping to system behaviour

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

## 11 — Content & Messages [REQUIRED]

**What to capture**: ALL user-facing text — not just error messages. This includes: reactive messages (errors, warnings, confirmations), help text alongside fields, legal/compliance copy, contextual explainer content, state messages (loading, empty, resume).

**Rewrite mode**: Screenshot every piece of text you see, not just errors. Help text next to fields, tooltips, legal disclaimers at the bottom of forms, explainer text ("What does comprehensive cover mean?"), section introductions, page titles. Every piece of text that a user reads is content.

**Greenfield mode**: For every validation rule, business rule, constraint, and edge case, ask: "What does the user see?" Then for every field, ask: "Does the user need help understanding this?" Then for every state, ask: "Is there introductory or contextual text?"

**Common mistakes**:
- Only documenting error messages (forgetting help text, legal copy, explainers)
- Messages without triggers (when does this show?)
- Hardcoded values instead of parameters ("17" instead of "{minAge}")
- Missing severity levels
- Forgetting empty states and loading states
- Legal text that hasn't been reviewed by legal
- Help text that assumes domain knowledge the user may not have

**Red flags**:
- Business rules or constraints with no linked messages
- Messages with no trigger condition
- Fields with no help text where the label alone is ambiguous
- No legal or consent text in a system that collects personal data
- No contextual content (explainers) for domain-specific options

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

---

## 16 — Interaction Behaviour [REQUIRED]

**What to capture**: How the system responds to user actions — error display patterns, loading/processing feedback, auto-save behaviour, input debouncing, confirmation dialogs, navigation rules. This is about behaviour, not visual design.

**Rewrite mode**: Interact with the system slowly and deliberately. Submit forms with errors — where do errors appear? How do they clear? Submit forms that take time — what loading feedback exists? Navigate backward — is data preserved? Try to double-submit — is it prevented? Try leaving with unsaved changes — is there a prompt?

**Greenfield mode**: For each state and each action, ask: "What does the user experience between triggering the action and seeing the result?" Define the feedback loop for every action.

**Common mistakes**:
- Only defining the happy path (action → success) without the in-between states
- No loading feedback for asynchronous operations
- Not specifying error placement (inline vs summary vs toast)
- Forgetting to handle double-submission
- Not specifying what happens with the browser back button
- No behaviour for "unsaved changes" warning

**Red flags**:
- No loading patterns defined for any async operation
- No error display strategy documented
- No auto-save or explicit save behaviour defined

---

## 17 — Accessibility [REQUIRED]

**What to capture**: Behavioural accessibility requirements — keyboard navigation, focus management, screen reader announcements, form accessibility patterns. This is not about visual design (colour contrast, font sizes) — it's about how the system behaves for users of assistive technology.

**Rewrite mode**: Navigate the entire system using only a keyboard (Tab, Enter, Escape, Arrow keys). Turn on a screen reader (VoiceOver, NVDA) and go through the flow — what gets announced? What's confusing? What's missing? Check every modal, every dynamic update, every error.

**Greenfield mode**: For each state and each interaction, ask: "How does a keyboard-only user do this? How does a screen reader user know what happened?" Define focus management for every state transition and every dynamic content change.

**Common mistakes**:
- Only testing with a mouse (keyboard navigation is fundamentally different)
- Forgetting focus management on state transitions (where does focus go?)
- Not announcing dynamic content changes to screen readers
- Missing error association (errors not linked to their fields via aria-describedby)
- Not trapping focus in modals
- Forgetting skip navigation links

**Red flags**:
- No compliance target specified (WCAG level)
- No focus management defined for any state transition
- No screen reader announcements for dynamic content
- No keyboard shortcuts or tab order documented

---

## 18 — Data Lifecycle & Compliance [REQUIRED]

**What to capture**: How data moves through the system from creation to deletion — consent, retention, user rights (access, deletion, portability), audit trails, data classification. This goes beyond "how long is data kept" into the full lifecycle.

**Rewrite mode**: Ask: "What happens if a user requests all their data? What happens if they request deletion? What consent was collected when they signed up? What's logged for audit purposes?" Check privacy policies, cookie banners, data processing agreements. Look at the database — is there soft deletion? Audit columns? Consent records?

**Greenfield mode**: Start with applicable regulations (GDPR, CCPA, etc.) and work backward to requirements. For each entity in the data model, ask: "How is this created? How long is it kept? How is it deleted? Who can access it? Is consent required?"

**Common mistakes**:
- Treating data lifecycle as just retention periods (it's much broader)
- No consent mechanism for data collection
- No way for users to access or delete their data
- Audit trail that doesn't cover enough actions or retains for too short
- Not classifying data by sensitivity
- Forgetting cascading deletion (delete user → what happens to their quotes?)

**Red flags**:
- No consent documented for a system that collects personal data
- No user rights defined (access, deletion, portability)
- No audit trail
- No data classification
- Retention periods defined but no deletion mechanism

---

## 19 — UI Specification [OPTIONAL]

**What to capture**: Visual design — layout, components, design tokens, responsive behaviour, brand guidelines. Include this when the spec needs to capture how the system looks, not just how it behaves.

**Rewrite mode**: Screenshot every screen at multiple breakpoints. Document the design tokens (colours, typography, spacing) by inspecting the CSS. Catalogue reusable components. Note responsive breakpoints and what changes.

**Greenfield mode**: Define design tokens first, then page layouts, then component inventory. Work from a design system if one exists.

**Common mistakes**:
- Specifying component visuals without specifying component states (hover, focus, disabled, error, loading)
- No responsive behaviour documented
- Design tokens that don't match the actual implementation

**Red flags**:
- No responsive breakpoints defined
- No component states documented

---

## 20 — Technical Architecture [OPTIONAL]

**What to capture**: Stack choices, database schema, API design, infrastructure, deployment, security architecture. Include this when the spec needs to constrain or inform implementation.

**Rewrite mode**: Document the current stack, database schema, API endpoints, and infrastructure. Note where the physical schema diverges from the conceptual data model.

**Greenfield mode**: Make and document technology decisions. Design the API surface, plan the database schema, define the deployment strategy.

**Common mistakes**:
- Database schema that doesn't match the data model without explanation
- API design with no error format convention
- No session management strategy
- Missing security considerations (CSRF, XSS, etc.)

**Red flags**:
- No session management defined
- No security architecture documented

---

## 21 — SEO & Discoverability [OPTIONAL]

**What to capture**: URL structure, page metadata, structured data, indexing rules, social sharing. Include this for public-facing web applications.

**Rewrite mode**: Crawl the site. Document URL patterns, check meta tags, look at robots.txt and sitemap. Check social sharing previews.

**Greenfield mode**: Define URL structure from capabilities and states. Decide what's indexable and what isn't.

**Common mistakes**:
- User-specific pages that are accidentally indexable
- No canonical URLs for pages with query parameters
- Missing meta descriptions

**Red flags**:
- No URL structure defined
- Flow steps that are indexable (they shouldn't be)

---

## 22 — Localization & Formatting [OPTIONAL]

**What to capture**: Date/currency/number formatting, timezone handling, locale support. Include this even for single-locale systems when formatting matters.

**Rewrite mode**: Check how dates, currencies, and numbers are displayed everywhere. Check if there are locale-specific formats. Look for hardcoded format strings.

**Greenfield mode**: Define the default locale and its formatting rules. If multi-locale, define detection and fallback behaviour.

**Common mistakes**:
- Storing formatted values instead of raw values (store pence, display pounds)
- Inconsistent date formats across the system
- No timezone strategy

**Red flags**:
- No date format specified for a system that uses dates
- Currency displayed without consistent formatting

---

## 23 — Animation & Transitions [OPTIONAL]

**What to capture**: Page transitions, micro-interactions, loading animations, scroll behaviour. Include this when motion is a meaningful part of the experience.

**Rewrite mode**: Record screen captures and note every animation. Document durations, easing, and triggers.

**Greenfield mode**: Define motion principles, then specify transitions for state changes and feedback for interactions.

**Common mistakes**:
- No reduced-motion support (prefers-reduced-motion)
- Animations that block interaction
- Inconsistent durations and easing

**Red flags**:
- No mention of prefers-reduced-motion

---

## 24 — Monitoring & Observability [OPTIONAL]

**What to capture**: Key metrics, dashboards, alerts, logging strategy, health checks. Include this for systems that need production monitoring.

**Rewrite mode**: Check existing dashboards, alert configurations, and logging. Document what's monitored and what gaps exist.

**Greenfield mode**: Define key metrics from capabilities and NFRs. Design alerts for failure modes documented in external interactions and error taxonomy.

**Common mistakes**:
- Logging sensitive data (PII in logs)
- No alerts for external service failures
- Dashboards that nobody looks at

**Red flags**:
- No health check endpoints
- No alert for any failure scenario
- Sensitive data not excluded from logs

---

## 25 — Onboarding & Empty States [OPTIONAL]

**What to capture**: First-time user experience, empty states, progressive disclosure, contextual hints. Include this when first-run behaviour differs from returning-user behaviour.

**Rewrite mode**: Create a new account and go through the system fresh. What's different from a returning user? What do empty states look like? Are there tooltips or hints?

**Greenfield mode**: For each state, ask: "What does this look like with no data yet?" For the overall system, ask: "Does a first-time user need guidance?"

**Common mistakes**:
- Empty states with no call to action (user doesn't know what to do)
- Onboarding that can't be dismissed or skipped
- Onboarding that reappears after being dismissed

**Red flags**:
- No empty states defined for any list or collection
- First-time experience identical to returning user when it shouldn't be

---

## 26 — Multi-Channel Behaviour [OPTIONAL]

**What to capture**: How the system behaves across different devices, platforms, and entry points. Include this when the system is accessed via more than one channel.

**Rewrite mode**: Use the system on every channel it supports — desktop, mobile, native app, email links. Document where behaviour differs. Test cross-channel continuity (start on one, continue on another).

**Greenfield mode**: List supported channels. For each capability, ask: "Does this work the same on every channel?" Define cross-channel state synchronisation.

**Common mistakes**:
- Assuming mobile web and desktop web behave identically
- No cross-channel state sync (user loses progress switching devices)
- Deeplinks that break when sessions expire

**Red flags**:
- No supported channels listed
- No cross-channel continuity defined for a multi-channel system
