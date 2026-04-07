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
- What data it touches (entities, fields, ownership)
- What rules govern it (validation, business logic, constraints)
- How it progresses (states, transitions, conditions)
- What can go wrong (edge cases, errors, failure modes)
- What it says (messages, copy, error text)

Everything is **UI-agnostic**. BET describes behaviour, not screens.

## What BET Is Not

- Not a project management tool
- Not a UI wireframing system
- Not a testing framework (though it produces test-ready specs)
- Not opinionated about tech stack or architecture

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
│   ├── 01-capabilities.md            — [REQUIRED] System goals and actors
│   ├── 02-data-model.md              — [REQUIRED] Entities, fields, types, ownership
│   ├── 03-permissions.md             — [OPTIONAL] Roles and access control
│   ├── 04-validation-rules.md        — [REQUIRED] Input correctness rules
│   ├── 05-business-rules.md          — [REQUIRED] Core logic (IF/THEN)
│   ├── 06-flow-state.md              — [REQUIRED] States, transitions, resumability
│   ├── 07-conditional-logic.md       — [REQUIRED] Dynamic behaviour and cascades
│   ├── 08-constraints.md             — [REQUIRED] Blocking conditions and recovery paths
│   ├── 09-external-interactions.md   — [OPTIONAL] APIs, services, failure modes
│   ├── 10-events-side-effects.md     — [OPTIONAL] Analytics, webhooks, notifications
│   ├── 11-messages.md                — [REQUIRED] User-facing text, triggers, severity
│   ├── 12-error-taxonomy.md          — [OPTIONAL] Error categories and handling
│   ├── 13-edge-cases.md              — [REQUIRED] What could go wrong, categorised
│   ├── 14-dependencies.md            — [OPTIONAL] Assumptions and prerequisites
│   └── 15-non-functional.md          — [OPTIONAL] Performance, availability, retention
│
└── examples/
    └── insurance-quote/               — Fully worked example
```

## Recommended Fill-in Order

Sections have dependencies. Follow this order:

1. **Capabilities** — define goals first (everything else serves these)
2. **Data Model** — what data exists (everything references this)
3. **Permissions** — who interacts with the data
4. **Validation Rules** — correctness rules on the data
5. **Business Rules** — logic applied to the data
6. **Flow / State** — how the system progresses
7. **Conditional Logic** — what varies based on context
8. **Constraints** — hard stops
9. **External Interactions** — system boundaries
10. **Events & Side Effects** — what the system emits
11. **Messages** — user-facing text
12. **Error Taxonomy** — how failures are handled
13. **Edge Cases** — what could go wrong
14. **Dependencies** — what must be true
15. **Non-Functional** — performance, availability

## The Payoff

Once a BET spec is complete, BDD scenarios become almost mechanical:

```gherkin
Scenario: Underage user cannot get a quote
  Given a user provides a date of birth indicating age < 17
  When they attempt to proceed to quote calculation
  Then the system must block progression
  And display the message "You must be at least 17 years old to get a quote."
```

Every business rule, validation, constraint, and edge case maps directly to testable scenarios. See `docs/BDD-GENERATION.md` for the full conversion rules.
