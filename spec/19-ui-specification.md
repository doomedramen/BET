# 19 — UI Specification [OPTIONAL]

> Visual design and layout — how the system looks.
> Include this section when the spec needs to capture the visual design, not just the behaviour.
>
> **Dependencies**: [06-flow-state.md](06-flow-state.md), [07-conditional-logic.md](07-conditional-logic.md)

---

## Design Tokens

_Core visual values that define the design language._

### Colours

| Token | Value | Usage |
|---|---|---|
| _[e.g., primary]_ | _[#1A73E8]_ | _[Primary actions, links]_ |
| _[e.g., error]_ | _[#D32F2F]_ | _[Error states, destructive actions]_ |
| _[e.g., success]_ | _[#2E7D32]_ | _[Success states, confirmations]_ |
| _[e.g., background]_ | _[#FFFFFF]_ | _[Page background]_ |
| _[e.g., text-primary]_ | _[#1A1A1A]_ | _[Body text]_ |

### Typography

| Token | Value | Usage |
|---|---|---|
| _[e.g., heading-1]_ | _[32px / 700 / 1.2 line-height]_ | _[Page titles]_ |
| _[e.g., body]_ | _[16px / 400 / 1.5 line-height]_ | _[Body text]_ |
| _[e.g., caption]_ | _[12px / 400 / 1.4 line-height]_ | _[Help text, labels]_ |
| _[e.g., font-family]_ | _[Inter, system-ui, sans-serif]_ | _[All text]_ |

### Spacing

| Token | Value | Usage |
|---|---|---|
| _[e.g., space-xs]_ | _[4px]_ | _[Tight spacing]_ |
| _[e.g., space-sm]_ | _[8px]_ | _[Related elements]_ |
| _[e.g., space-md]_ | _[16px]_ | _[Between sections]_ |
| _[e.g., space-lg]_ | _[32px]_ | _[Between major sections]_ |

---

## Page Layouts

_High-level structure for each state or screen._

### _[STATE-XXX]_: _[State name]_

- **Layout type**: _[single column / two column / sidebar + main / wizard step]_
- **Max content width**: _[e.g., 640px / 960px / full width]_
- **Sections** (top to bottom):
  1. _[Section name — e.g., "Progress indicator"]_
  2. _[Section name — e.g., "Page heading + intro text"]_
  3. _[Section name — e.g., "Form fields"]_
  4. _[Section name — e.g., "Navigation buttons"]_

---

_Repeat for each state that has a distinct layout._

---

## Component Inventory

_Reusable components and their specifications._

### COMP-001: _[Component name]_

- **Description**: _[What this component does]_
- **States**: _[default, hover, focus, disabled, error, loading]_
- **Variants**: _[primary, secondary, destructive, etc.]_
- **Used in**: _[Which states/pages use this component]_

---

_Add more components as needed._

---

## Responsive Behaviour

| Breakpoint | Width | Layout changes |
|---|---|---|
| _[e.g., Mobile]_ | _[< 768px]_ | _[Single column, stacked navigation, full-width fields]_ |
| _[e.g., Tablet]_ | _[768px — 1024px]_ | _[Two column where applicable]_ |
| _[e.g., Desktop]_ | _[> 1024px]_ | _[Max-width container, sidebar visible]_ |

---

## Brand Guidelines

_References to external brand or design system documentation._

| Resource | Location | Notes |
|---|---|---|
| _[e.g., Brand guidelines]_ | _[URL or file path]_ | _[Logo usage, brand colours, tone of voice]_ |
| _[e.g., Design system]_ | _[URL or file path]_ | _[Component library, Figma, Storybook]_ |
