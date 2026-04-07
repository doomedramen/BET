# BET — Behaviour Extraction Template

A structured specification framework for software projects. BET produces specs so complete that any developer could build the system from them without asking questions.

Works for both **rewrites** (extracting behaviour from an existing system) and **new projects** (defining behaviour before code is written).

## The Principle

> "If I deleted the frontend completely, could I rebuild the system from this spec?"
>
> If the answer is yes, your spec is complete.

## What BET Is

BET is a **specification format** — a set of structured markdown files that capture everything about how a system behaves:

- What the system does (capabilities and goals)
- What data it touches (entities, fields, ownership, transformations)
- What rules govern it (validation, business logic, constraints, rate limits)
- How it progresses (states, transitions, conditions, field visibility)
- What can go wrong (edge cases, errors, failure modes, multi-session)
- What it says (messages, help text, legal copy, contextual content)
- How it responds (loading, error display, auto-save, navigation)
- How it's accessible (keyboard nav, focus management, screen readers)
- How data lives and dies (consent, retention, deletion, compliance)

Optional sections extend into UI design, technical architecture, SEO, localization, animation, monitoring, onboarding, and multi-channel behaviour.

## What BET Is Not

- Not a project management tool
- Not a testing framework (though it produces test-ready specs)
- Not opinionated about tech stack or architecture (unless you use the optional section)

## Quick Start

1. Copy the `spec/` folder into your project
2. Read `docs/FORMAT.md` for writing conventions
3. Fill in sections following the recommended order (each file notes its dependencies)
4. Use `docs/GUIDE.md` for tips on each section
5. Validate with `docs/COMPLETENESS-CHECKLIST.md`
6. Generate BDD scenarios using `docs/BDD-GENERATION.md`

## Structure

```
BET/
├── README.md                          — You are here
├── docs/
│   ├── FORMAT.md                      — Writing conventions and syntax rules
│   ├── GUIDE.md                       — Section-by-section walkthrough
│   ├── COMPLETENESS-CHECKLIST.md      — How to validate your spec is done
│   └── BDD-GENERATION.md             — Turning specs into Given/When/Then scenarios
│
├── spec/                              — Copy this folder to start a new spec
│   │
│   │  ── Core Behaviour ──
│   ├── 01-capabilities.md            — [REQUIRED] System goals and actors
│   ├── 02-data-model.md              — [REQUIRED] Entities, fields, types, ownership, transformations
│   ├── 03-permissions.md             — [OPTIONAL] Roles and access control
│   ├── 04-validation-rules.md        — [REQUIRED] Input correctness rules
│   ├── 05-business-rules.md          — [REQUIRED] Core logic, presentation rules (sort/filter)
│   ├── 06-flow-state.md              — [REQUIRED] States, transitions, resumability
│   ├── 07-conditional-logic.md       — [REQUIRED] Dynamic behaviour, cascades, field visibility
│   ├── 08-constraints.md             — [REQUIRED] Blocking conditions, recovery, rate limits
│   ├── 09-external-interactions.md   — [OPTIONAL] APIs, services, failure modes, API contracts
│   ├── 10-events-side-effects.md     — [OPTIONAL] Analytics, webhooks, notifications
│   ├── 11-messages.md                — [REQUIRED] All user-facing text: messages, help, legal, content
│   ├── 12-error-taxonomy.md          — [OPTIONAL] Error categories and handling
│   ├── 13-edge-cases.md              — [REQUIRED] What could go wrong, incl. multi-session
│   ├── 14-dependencies.md            — [OPTIONAL] Assumptions and prerequisites
│   ├── 15-non-functional.md          — [OPTIONAL] Performance, availability, retention
│   │
│   │  ── Interaction & Accessibility ──
│   ├── 16-interaction-behaviour.md   — [REQUIRED] Error display, loading, auto-save, navigation
│   ├── 17-accessibility.md           — [REQUIRED] Keyboard, focus, screen readers, WCAG
│   ├── 18-data-lifecycle.md          — [REQUIRED] Consent, retention, deletion, compliance
│   │
│   │  ── Optional Extensions ──
│   ├── 19-ui-specification.md        — [OPTIONAL] Visual design, layout, components, responsive
│   ├── 20-technical-architecture.md  — [OPTIONAL] Stack, database, API design, infrastructure
│   ├── 21-seo.md                     — [OPTIONAL] URLs, metadata, indexing, social sharing
│   ├── 22-localization.md            — [OPTIONAL] Dates, currency, numbers, i18n
│   ├── 23-animation.md              — [OPTIONAL] Transitions, micro-interactions, motion
│   ├── 24-monitoring.md              — [OPTIONAL] Dashboards, alerts, logging, health checks
│   ├── 25-onboarding.md              — [OPTIONAL] First-run, empty states, feature discovery
│   └── 26-multi-channel.md           — [OPTIONAL] Cross-device, cross-platform behaviour
│
└── examples/
    └── insurance-quote/               — Fully worked example
```

## Recommended Fill-in Order

Sections have dependencies. Follow this order:

1. **Capabilities** — define goals first (everything else serves these)
2. **Data Model** — what data exists, including transformations
3. **Permissions** — who interacts with the data
4. **Validation Rules** — correctness rules on the data
5. **Business Rules** — logic applied to the data, including presentation rules
6. **Flow / State** — how the system progresses
7. **Conditional Logic** — what varies based on context, including field visibility
8. **Constraints** — hard stops, including rate limits
9. **External Interactions** — system boundaries, including API contracts
10. **Events & Side Effects** — what the system emits
11. **Content & Messages** — all user-facing text (messages, help, legal, content)
12. **Error Taxonomy** — how failures are handled
13. **Edge Cases** — what could go wrong, including multi-session
14. **Dependencies** — what must be true
15. **Non-Functional** — performance, availability
16. **Interaction Behaviour** — how the system responds (errors, loading, navigation)
17. **Accessibility** — keyboard, focus, screen readers
18. **Data Lifecycle** — consent, retention, deletion, compliance
19–26. **Optional Extensions** — UI, architecture, SEO, localization, animation, monitoring, onboarding, multi-channel (fill in any order, as needed)

## The Payoff

Once a BET spec is complete, BDD scenarios become almost mechanical:

```gherkin
Scenario: Underage user cannot get a quote
  Given a user provides a date of birth indicating age < 17
  When they attempt to proceed to quote calculation
  Then the system must block progression
  And display the message "You must be at least 17 years old to get a quote."
```

Every business rule, validation, constraint, edge case, interaction pattern, accessibility requirement, and data lifecycle rule maps directly to testable scenarios. See `docs/BDD-GENERATION.md` for the full conversion rules.
