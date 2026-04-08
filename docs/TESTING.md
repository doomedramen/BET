# BET Testing Guide

How a complete BET spec drives a full test suite — unit tests, integration tests, end-to-end tests, contract tests, and BDD scenarios.

BDD scenario generation is covered separately in `BDD-GENERATION.md`. This guide covers the full testing pyramid.

---

## The Principle

> "Every item in the spec is a test obligation. If it's specified, it must be tested."

A complete BET spec tells you *exactly* what to test and at which layer. Nothing requires guesswork.

---

## The Testing Pyramid

```
         ┌─────────────────┐
         │   E2E / BDD     │  ← Flow states, edge cases, accessibility, interaction behaviour
         ├─────────────────┤
         │  Integration    │  ← External interactions, events, data lifecycle, persistence
         ├─────────────────┤
         │  Contract       │  ← API contracts from external interactions
         ├─────────────────┤
         │     Unit        │  ← Business rules, validation rules, conditional logic, constraints
         └─────────────────┘
```

---

## Spec → Test Layer Mapping

### Unit Tests

Unit tests cover logic that can be exercised in isolation, without I/O or external dependencies.

| Spec Section | What to unit test |
|---|---|
| **04 — Validation Rules** | Each `VR-XXX` rule: valid input passes, invalid input fails, timing behaviour |
| **05 — Business Rules** | Each `BR-XXX` rule: both branches (IF true / IF false), priority ordering when rules conflict |
| **05 — Presentation Rules** | Each `PR-XXX` rule: sort order, filter logic, empty state condition |
| **07 — Conditional Logic** | Each `COND-XXX` condition: trigger produces correct outcome, cascading effects |
| **08 — Constraints** | Each `CON-XXX` constraint: blocking condition is evaluated correctly |
| **08 — Rate Limits** | Each `RATE-XXX` limit: threshold enforcement, recovery logic |
| **02 — Data Model** | Calculated/derived fields: formula correctness, edge values (null, zero, boundary) |
| **02 — Transformations** | Each `TRANS-XXX`: field mapping, formatting, omission rules |

**Yield per section:**
- Validation rules → 2 tests per rule (valid + invalid), more for boundary values
- Business rules → 2+ tests per rule (positive + negative branches)
- Conditional logic → 2+ tests per condition (trigger true + false, cascade)

---

### Integration Tests

Integration tests cover behaviour that requires real infrastructure — database, message queues, file storage, background jobs.

| Spec Section | What to integration test |
|---|---|
| **06 — Flow / State** | State transitions persist correctly; resumability (data survives process restart) |
| **09 — External Interactions** | Real calls to external services (or controlled test doubles at the network boundary) |
| **10 — Events & Side Effects** | Events fire at the correct point in the flow; audit records are written |
| **18 — Data Lifecycle** | Data is created with correct consent records; retention policy enforcement; cascading deletion |
| **15 — Non-Functional** | Persistence correctness; query performance against real data volumes |
| **14 — Dependencies** | Seed/reference data is present and correctly loaded |

**Yield per section:**
- Each `STATE-XXX` resumability rule → 1 persistence test
- Each `EVT-XXX` event → 1 test confirming the event fires and the side effect occurs
- Each `CONSENT-XXX` → 1 test confirming consent is recorded before data is written
- Each `RIGHT-XXX` (deletion) → 1 test confirming cascading deletion is complete

---

### Contract Tests

Contract tests verify that the actual shape of API calls matches what the spec documents. Run these against sandbox/mock endpoints or using consumer-driven contract tools (Pact, etc.).

| Spec Section | What to contract test |
|---|---|
| **09 — External Interactions** | Each `EXT-XXX` API contract: method, endpoint, auth, request schema, response schema, error codes |

**For each `EXT-XXX` with an API contract, produce:**
- 1 test: happy path request/response matches contract schema
- 1 test per error code: error response matches documented schema
- 1 test: authentication is sent correctly
- 1 test per documented rate limit: rate limit response is handled correctly

---

### End-to-End Tests

E2E tests cover complete user journeys through the running system. Use the flow states as your journey map.

| Spec Section | What to E2E test |
|---|---|
| **06 — Flow / State** | Complete journeys from entry state to each terminal state |
| **13 — Edge Cases** | Partial state recovery, multi-session, concurrency, timing |
| **16 — Interaction Behaviour** | Loading feedback, error display placement, double-submit prevention, back button |
| **17 — Accessibility** | Keyboard-only navigation, focus management, screen reader announcements |
| **09 — External Interactions** | Failure mode behaviour (service down, timeout) — using test doubles at the boundary |
| **08 — Constraints** | Hard stop scenarios — the user genuinely cannot proceed |
| **25 — Onboarding** | First-time experience differs correctly from returning user |
| **26 — Multi-Channel** | Cross-device continuity, channel-specific behaviour |

**Yield per section:**
- Each `STATE-XXX` → at least 1 journey test (happy path) + 1 test per failure branch
- Each `EDGE-XXX` → 1 E2E test for the specified scenario
- Each `LOADING-XXX` → 1 test confirming feedback appears and clears correctly
- Each `ANNOUNCE-XXX` → 1 accessibility test (screen reader announcement)

---

### BDD Scenarios

BDD scenarios (Given/When/Then) sit at the E2E layer but are driven directly from the spec. See `BDD-GENERATION.md` for conversion rules.

Every REQUIRED section must produce at least one BDD scenario. Use `@SPEC-ID` tags to link scenarios to spec items.

---

## Coverage Targets by Section

| Section | Test layer | Minimum coverage |
|---|---|---|
| 04 — Validation Rules | Unit | 100% of `VR-XXX` rules |
| 05 — Business Rules | Unit | 100% of `BR-XXX` rules (both branches) |
| 05 — Presentation Rules | Unit | 100% of `PR-XXX` rules |
| 06 — Flow / State | E2E | Every state reachable; every terminal state covered |
| 07 — Conditional Logic | Unit | 100% of `COND-XXX` conditions |
| 08 — Constraints | Unit + E2E | 100% of `CON-XXX` conditions; blocking verified E2E |
| 08 — Rate Limits | Unit + Integration | 100% of `RATE-XXX` limits |
| 09 — External Interactions | Contract + Integration | 100% of `EXT-XXX` failure modes |
| 10 — Events & Side Effects | Integration | 100% of `EVT-XXX` events |
| 13 — Edge Cases | E2E | 100% of `EDGE-XXX` cases |
| 16 — Interaction Behaviour | E2E | 100% of `LOADING-XXX` patterns |
| 17 — Accessibility | E2E | 100% of `ANNOUNCE-XXX` announcements; all states keyboard-navigable |
| 18 — Data Lifecycle | Integration | 100% of `CONSENT-XXX` and `RIGHT-XXX` items |

---

## Test ID Convention

Link tests back to spec IDs using tags or comments:

```
// @VR-001 @BR-001
test("underage driver is rejected", () => { ... })
```

```gherkin
@BR-001 @CON-001
Scenario: Underage driver is blocked
```

This makes it possible to find which tests cover a given spec item, and which spec items have no test coverage.

---

## Documenting Testing Decisions

Use `spec/27-testing-strategy.md` (if included) to record:
- Which test framework(s) the project uses
- Which spec sections are covered at which layer
- Known gaps and deferred coverage
- How to run the test suite locally and in CI
