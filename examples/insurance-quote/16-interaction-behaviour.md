# 16 — Interaction Behaviour

## Error Display

### Error Placement

| Context | Placement | Behaviour |
|---|---|---|
| Field validation (on input) | Inline below the field | Appears when field loses focus, clears when field value becomes valid |
| Field validation (on submit) | Inline below field + summary at top of form | Appears on submit, summary links to each errored field, clears per-field when resolved |
| Cross-field validation | Summary at top of form | Appears on submit, explains the relationship between fields |
| Constraint block (e.g., underage) | Inline below the triggering field | Appears immediately, cannot be dismissed, prevents progression |
| System error | Full-width banner at top of page | Persists until dismissed or user retries |

### Error Behaviour

- **Multiple errors**: Show all at once — both inline per-field and in the summary
- **Scroll behaviour**: Scroll to the error summary at the top of the form
- **Clear timing**: Inline errors clear when the individual field value changes and passes validation. Summary clears on next submit attempt.

---

## Loading & Processing Feedback

### LOADING-001: Vehicle Registration Lookup

- **Trigger**: User enters a valid registration number and field loses focus (after debounce)
- **Immediate feedback**: Spinner appears inside the registration field
- **Threshold**: No enhanced feedback (lookup is expected to be fast)
- **Timeout**: 5s — spinner removed, manual entry fields shown, warning displayed ([MSG-011](11-messages.md#MSG-011))
- **Cancellable**: yes — user can edit the registration number to cancel the pending lookup
- **Success feedback**: Vehicle fields auto-populated, briefly highlighted, then marked read-only
- **References**: [EXT-001](09-external-interactions.md#EXT-001), [COND-002](07-conditional-logic.md#COND-002), [COND-003](07-conditional-logic.md#COND-003)

### LOADING-002: Quote Calculation

- **Trigger**: User submits complete quote request in [STATE-003](06-flow-state.md#STATE-003)
- **Immediate feedback**: Submit button disabled, text replaced with spinner
- **Threshold**: After 3s, transition to a dedicated loading screen showing "Getting your quotes..." ([MSG state message](11-messages.md))
- **Timeout**: 15s — show error message ([MSG-009](11-messages.md#MSG-009))
- **Cancellable**: no
- **Success feedback**: Redirect to [STATE-005](06-flow-state.md#STATE-005) (QuoteResults)
- **References**: [STATE-004](06-flow-state.md#STATE-004), [EXT-002](09-external-interactions.md#EXT-002)

---

## Auto-Save Behaviour

- **Enabled**: yes
- **Trigger**: on field blur (when user leaves a field)
- **Debounce**: none (saves on blur, not on keystroke)
- **Feedback**: Subtle "Saved" text appears near the form footer, fades after 2s
- **Failure**: Retry silently once. If second attempt fails, show warning: "Your progress may not be saved. Please don't close this page."
- **Scope**: per state — all fields in the current state are saved together

---

## Input Behaviour

### Debounce & Throttle

| Action | Type | Duration | Reason |
|---|---|---|---|
| Vehicle registration lookup | debounce | 500ms | Wait for user to finish typing registration |
| Quote request submission | throttle | 5s | Prevent double-submission |

### Confirmation Patterns

| Action | Confirmation type | Message |
|---|---|---|
| Navigate away with unsaved changes | Browser beforeunload prompt | "You have unsaved changes that may be lost." |
| Submit quote request | Review step in STATE-003 | Summary of all entered data displayed before submission |

---

## Navigation Behaviour

- **Back navigation**: Allowed freely between STATE-001, STATE-002, STATE-003
- **Data on back**: Preserved — all entered data retained when moving backward
- **Browser back button**: Navigates within the flow (each state is a history entry)
- **Deep linking**: Not supported — all users start at STATE-001 (or resume from saved state). Direct links to STATE-002+ redirect to the latest valid state.
- **Invalidation on change**: If Driver details change after vehicle/cover details are entered, vehicle and cover details are preserved but cached quotes are invalidated. Recalculation required.
