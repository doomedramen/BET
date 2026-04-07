# 17 — Accessibility [REQUIRED]

> Behavioural requirements for accessibility — focus management, keyboard navigation, screen reader behaviour.
> This is not about visual design (colour contrast, font sizes) — it's about how the system behaves for users of assistive technology.
>
> **Dependencies**: [06-flow-state.md](06-flow-state.md), [11-messages.md](11-messages.md), [16-interaction-behaviour.md](16-interaction-behaviour.md)

---

## Compliance Target

- **Standard**: _[WCAG 2.1 AA / WCAG 2.2 AA / Section 508 / other]_
- **Legal requirement**: _[yes (cite regulation) / best practice]_
- **Testing approach**: _[automated + manual / automated only / third-party audit]_

---

## Keyboard Navigation

_Every interactive element must be reachable and operable via keyboard._

### Tab Order

| State | Tab sequence |
|---|---|
| _[STATE-XXX]_ | _[First field → Second field → ... → Submit button]_ |

### Keyboard Shortcuts

| Key | Action | Context |
|---|---|---|
| _[e.g., Enter]_ | _[Submit current step]_ | _[When focus is on submit button]_ |
| _[e.g., Escape]_ | _[Close modal / cancel action]_ | _[When modal is open]_ |
| _[e.g., Tab]_ | _[Move to next field]_ | _[Within form]_ |

### Skip Navigation

- **Skip link**: _[yes / no — "Skip to main content" link at page top]_
- **Landmark regions**: _[List ARIA landmarks — main, nav, banner, complementary, etc.]_

---

## Focus Management

_Where keyboard focus moves during state changes and dynamic content updates._

| Event | Focus moves to |
|---|---|
| _[e.g., State transition]_ | _[First field of new state / heading of new state]_ |
| _[e.g., Error on submit]_ | _[Error summary / first errored field]_ |
| _[e.g., Modal opens]_ | _[First focusable element in modal]_ |
| _[e.g., Modal closes]_ | _[Element that triggered the modal]_ |
| _[e.g., Dynamic content loads]_ | _[Beginning of new content / no focus change]_ |
| _[e.g., Item deleted from list]_ | _[Next item / previous item / list heading]_ |

### Focus Trapping

_Contexts where focus must be constrained (user cannot Tab outside a region)._

| Context | Trapped within | Exit method |
|---|---|---|
| _[e.g., Modal dialog]_ | _[Modal boundary]_ | _[Escape key / close button]_ |

---

## Screen Reader Announcements

_Dynamic content changes that must be announced to screen reader users._

### ANNOUNCE-001: _[Announcement name]_

- **Trigger**: _[What causes the announcement]_
- **Text**: "_[What is announced]_"
- **Priority**: _[polite (announced when idle) / assertive (announced immediately)]_
- **Mechanism**: _[aria-live region / role="alert" / role="status"]_

---

_Add more announcements as needed. Consider: error messages, loading states, success confirmations, dynamic content updates, timer warnings._

---

## Form Accessibility

_Specific accessibility behaviours for form interactions._

- **Labels**: _[Every field has a visible label / programmatically associated via `for`/`id`]_
- **Error association**: _[Errors are linked to fields via aria-describedby / aria-errormessage]_
- **Required fields**: _[Indicated visually AND programmatically via aria-required]_
- **Field groups**: _[Related fields grouped with fieldset/legend or role="group"]_
- **Help text association**: _[Help text linked to fields via aria-describedby]_

---

## Alternative Content

_Non-text content that needs accessible alternatives._

| Content | Alternative | Type |
|---|---|---|
| _[e.g., Loading spinner]_ | _"Loading, please wait"_ | _aria-label_ |
| _[e.g., Status icon]_ | _"Complete" / "Error"_ | _aria-label or visually hidden text_ |
| _[e.g., Progress indicator]_ | _"Step 2 of 4"_ | _aria-label + aria-valuenow_ |
