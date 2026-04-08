# Testing Strategy

> Project: [Project Name]
> Status: [ ] Not Started  [ ] In Progress  [ ] Complete

---

## Overview

Documents the testing approach for this project — which frameworks are used, which spec sections are covered at which test layer, and any known gaps.

See `docs/TESTING.md` for how BET spec items map to unit, integration, contract, and E2E tests.

---

## Test Frameworks & Tooling

<!--
  Document the test tools in use for each layer.
  Example:
  - Unit: Jest / Vitest / pytest / RSpec
  - Integration: Supertest / TestContainers / pytest-django
  - Contract: Pact / Schemathesis
  - E2E: Playwright / Cypress / Selenium
  - BDD runner: Cucumber / Behave / SpecFlow
-->

| Layer | Framework | Notes |
|---|---|---|
| Unit | [framework] | |
| Integration | [framework] | |
| Contract | [framework] | |
| E2E / BDD | [framework] | |

---

## Coverage Plan

Map spec sections to the test layers that cover them. Mark gaps explicitly.

| Spec section | Unit | Integration | Contract | E2E/BDD | Gap / Notes |
|---|---|---|---|---|---|
| 04 — Validation Rules | [ ] | — | — | [ ] | |
| 05 — Business Rules | [ ] | — | — | [ ] | |
| 05 — Presentation Rules | [ ] | — | — | [ ] | |
| 06 — Flow / State | — | [ ] | — | [ ] | |
| 07 — Conditional Logic | [ ] | — | — | [ ] | |
| 08 — Constraints | [ ] | — | — | [ ] | |
| 08 — Rate Limits | [ ] | [ ] | — | — | |
| 09 — External Interactions | — | [ ] | [ ] | [ ] | |
| 10 — Events & Side Effects | — | [ ] | — | — | |
| 13 — Edge Cases | — | — | — | [ ] | |
| 16 — Interaction Behaviour | — | — | — | [ ] | |
| 17 — Accessibility | — | — | — | [ ] | |
| 18 — Data Lifecycle | — | [ ] | — | — | |

---

## Entries

<!--
  Use TEST-XXX IDs for decisions, constraints, or deferred coverage.
  Each entry should document a specific testing decision or known gap.
-->

### TEST-001: [Title]

- **Layer**: [unit | integration | contract | e2e | bdd]
- **Covers**: [spec IDs this test or decision covers, e.g. BR-001, VR-003]
- **Decision / Gap**: [What was decided, or why coverage is deferred]
- **Status**: [ ] Implemented  [ ] Deferred  [ ] Out of scope

---

## How to Run

<!--
  Document how to run each layer locally and in CI.
  Example:
  - Unit: `npm test`
  - Integration: `docker compose up -d && npm run test:integration`
  - E2E: `npx playwright test`
  - CI: runs on pull request via GitHub Actions
-->

| Layer | Local command | CI trigger |
|---|---|---|
| Unit | `[command]` | |
| Integration | `[command]` | |
| Contract | `[command]` | |
| E2E | `[command]` | |

---

## Notes

<!-- Open questions, deferred decisions, or known limitations in test coverage -->
