# 16 — Interaction Behaviour [REQUIRED]

> How the system behaves in response to user actions — not how it looks, but how it responds.
> Covers error display patterns, loading behaviour, auto-save, feedback, and timing.
>
> **Dependencies**: [06-flow-state.md](06-flow-state.md), [04-validation-rules.md](04-validation-rules.md), [11-messages.md](11-messages.md)

---

## Error Display

_How and where error messages are presented to the user._

### Error Placement

| Context | Placement | Behaviour |
|---|---|---|
| _[e.g., Field validation]_ | _[inline below field / summary at top / toast / modal]_ | _[Appears on trigger, clears when resolved]_ |
| _[e.g., Cross-field validation]_ | _[summary at top]_ | _[Appears on submit, links to relevant fields]_ |
| _[e.g., System error]_ | _[full-page / modal / toast]_ | _[Persists until dismissed or resolved]_ |
| _[e.g., Constraint block]_ | _[inline / modal]_ | _[Cannot be dismissed without resolving]_ |

### Error Behaviour

- **Multiple errors**: _[Show all at once / show first only / show per-field + summary]_
- **Scroll behaviour**: _[Scroll to first error / no scroll / scroll to summary]_
- **Clear timing**: _[Clear on input change / clear on re-submit / clear manually]_

---

## Loading & Processing Feedback

_What the user sees during asynchronous operations._

### LOADING-001: _[Operation name]_

- **Trigger**: _[What action starts the loading state]_
- **Immediate feedback**: _[What happens instantly — e.g., button disabled, spinner appears]_
- **Threshold**: _[How long before enhanced feedback — e.g., after 2s show progress message]_
- **Timeout**: _[Maximum wait before showing error — e.g., 15s]_
- **Cancellable**: _[yes / no — can the user abort?]_
- **Success feedback**: _[What happens on completion — e.g., redirect, toast, inline update]_
- **References**: _[Link to relevant state or external interaction]_

---

_Add more loading patterns as needed._

---

## Auto-Save Behaviour

_Whether and how user input is automatically preserved._

- **Enabled**: _[yes / no]_
- **Trigger**: _[on field change / on blur / after idle period / on interval]_
- **Debounce**: _[Duration — e.g., 500ms after last keystroke]_
- **Feedback**: _[How the user knows data is saved — e.g., "Saved" indicator, no feedback, status icon]_
- **Failure**: _[What happens if save fails — retry silently, show warning, queue for retry]_
- **Scope**: _[per field / per section / per state]_

---

## Input Behaviour

_Behavioural patterns for user input that affect how the system responds._

### Debounce & Throttle

| Action | Type | Duration | Reason |
|---|---|---|---|
| _[e.g., Registration lookup]_ | _debounce_ | _[500ms]_ | _[Avoid excessive API calls]_ |
| _[e.g., Search input]_ | _debounce_ | _[300ms]_ | _[Wait for user to finish typing]_ |
| _[e.g., Form submit]_ | _throttle_ | _[2s]_ | _[Prevent double submission]_ |

### Confirmation Patterns

_Actions that require explicit confirmation before executing._

| Action | Confirmation type | Message |
|---|---|---|
| _[e.g., Delete data]_ | _modal with confirm/cancel_ | _"Are you sure? This cannot be undone."_ |
| _[e.g., Navigate away with unsaved changes]_ | _browser prompt_ | _"You have unsaved changes."_ |
| _[e.g., Submit quote request]_ | _review step_ | _Summary of entered data before submission_ |

---

## Navigation Behaviour

_How the system handles forward/backward movement through the flow._

- **Back navigation**: _[Allowed freely / allowed with warning / not allowed]_
- **Data on back**: _[Preserved / cleared / preserved but revalidated]_
- **Browser back button**: _[Navigates within flow / exits flow / blocked with prompt]_
- **Deep linking**: _[Can users link directly to a specific state? If so, what are the entry conditions?]_
- **Invalidation on change**: _[If earlier data changes, what later data is invalidated?]_
