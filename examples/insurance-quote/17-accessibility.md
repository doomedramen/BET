# 17 — Accessibility

## Compliance Target

- **Standard**: WCAG 2.1 AA
- **Legal requirement**: yes — UK Equality Act 2010, Public Sector Bodies Accessibility Regulations 2018 (if applicable)
- **Testing approach**: Automated (axe-core in CI) + manual screen reader testing (VoiceOver on macOS, NVDA on Windows)

---

## Keyboard Navigation

### Tab Order

| State | Tab sequence |
|---|---|
| STATE-001: CollectDriverDetails | Skip link → First name → Last name → Date of birth → Occupation → Licence type → Years held → Consent checkbox → Continue button |
| STATE-002: CollectVehicleDetails | Skip link → Registration number → (auto-populated fields, read-only — skipped in tab order) → Estimated value → Usage → (Business type, if visible) → Continue button |
| STATE-003: CollectCoverDetails | Skip link → Cover type → Start date → Review summary → Terms checkbox → Get quotes button |
| STATE-005: QuoteResults | Skip link → Sort dropdown → Filter controls → First quote card → ... → Last quote card |

### Keyboard Shortcuts

| Key | Action | Context |
|---|---|---|
| Enter | Submit current step | When focus is on Continue/Submit button |
| Escape | Close error summary | When error summary is focused |
| Tab | Move to next interactive element | All states |
| Shift+Tab | Move to previous interactive element | All states |

### Skip Navigation

- **Skip link**: yes — "Skip to main content" link as the first focusable element on every page
- **Landmark regions**: `<main>` (form content), `<nav>` (step indicator), `<footer>` (legal links)

---

## Focus Management

| Event | Focus moves to |
|---|---|
| State transition (forward) | First form field in the new state |
| State transition (backward) | First form field in the previous state |
| Error on submit | Error summary region at top of form |
| Vehicle lookup completes (success) | estimatedValue field (first editable field after auto-populated ones) |
| Vehicle lookup fails | make field (first manual entry field) |
| Quote results load | Results heading ("Your quotes") |
| Error banner appears (system error) | The error banner |

### Focus Trapping

| Context | Trapped within | Exit method |
|---|---|---|
| No modals in current flow | N/A | N/A |

_Note: The current quote flow does not use modal dialogs. If modals are added in future, focus must be trapped within them._

---

## Screen Reader Announcements

### ANNOUNCE-001: Validation Errors on Submit

- **Trigger**: Form submission fails validation
- **Text**: "{count} errors found. Please review and correct the highlighted fields."
- **Priority**: assertive
- **Mechanism**: role="alert" on the error summary region

### ANNOUNCE-002: Vehicle Lookup in Progress

- **Trigger**: Vehicle registration lookup starts
- **Text**: "Looking up your vehicle..."
- **Priority**: polite
- **Mechanism**: aria-live="polite" region

### ANNOUNCE-003: Vehicle Lookup Complete

- **Trigger**: Vehicle registration lookup succeeds
- **Text**: "Vehicle details found and filled in: {make} {model}, {year}."
- **Priority**: polite
- **Mechanism**: aria-live="polite" region

### ANNOUNCE-004: Vehicle Lookup Failed

- **Trigger**: Vehicle registration lookup fails (timeout, not found, service down)
- **Text**: "Vehicle lookup unsuccessful. Please enter your vehicle details manually."
- **Priority**: assertive
- **Mechanism**: role="alert"

### ANNOUNCE-005: Quote Calculation in Progress

- **Trigger**: Quote calculation begins
- **Text**: "Getting your quotes. This may take a few moments."
- **Priority**: polite
- **Mechanism**: aria-live="polite" region

### ANNOUNCE-006: Quote Results Ready

- **Trigger**: Quote results are displayed
- **Text**: "{count} quotes found."
- **Priority**: assertive
- **Mechanism**: role="alert"

### ANNOUNCE-007: Auto-Save Confirmation

- **Trigger**: Auto-save completes successfully
- **Text**: "Progress saved."
- **Priority**: polite
- **Mechanism**: role="status"

---

## Form Accessibility

- **Labels**: Every field has a visible `<label>` programmatically associated via `for`/`id`
- **Error association**: Errors linked to fields via `aria-describedby` pointing to the inline error element
- **Required fields**: Indicated visually with "(required)" text after the label AND programmatically via `aria-required="true"`
- **Field groups**: Date of birth fields (if using day/month/year selectors) grouped with `<fieldset>` and `<legend>`
- **Help text association**: Help text linked to fields via `aria-describedby` (appended to error reference if both present)

---

## Alternative Content

| Content | Alternative | Type |
|---|---|---|
| Loading spinner (vehicle lookup) | "Looking up vehicle..." | aria-label on spinner element |
| Loading spinner (quote calculation) | "Calculating quotes..." | aria-label on spinner element |
| Step progress indicator | "Step 2 of 3: Vehicle Details" | aria-label + aria-valuenow/aria-valuemax |
| Read-only field indicator | "Auto-filled, read only" | aria-readonly="true" + visually hidden text |
| Sort order indicator (results) | "Sorted by annual price, ascending" | aria-label on sort control |
