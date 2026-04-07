# 23 — Animation & Transitions [OPTIONAL]

> Motion design — page transitions, micro-interactions, loading animations.
> Include this when motion is a meaningful part of the user experience.
>
> **Dependencies**: [06-flow-state.md](06-flow-state.md), [16-interaction-behaviour.md](16-interaction-behaviour.md)

---

## Motion Principles

- **Purpose**: _[What role does animation play — guide attention, provide feedback, create continuity, convey personality?]_
- **Reduced motion**: _[How does the system behave when prefers-reduced-motion is set? Disable all animations / reduce to opacity fades / keep essential feedback only]_
- **Duration range**: _[e.g., 100ms-500ms — nothing should feel sluggish]_

---

## Page / State Transitions

| From | To | Animation | Duration | Easing |
|---|---|---|---|---|
| _[STATE-XXX]_ | _[STATE-XXX]_ | _[slide left / fade / none]_ | _[300ms]_ | _[ease-in-out]_ |

---

## Micro-Interactions

_Small animations that provide feedback for specific actions._

### ANIM-001: _[Interaction name]_

- **Trigger**: _[What causes this animation — e.g., button click, field validation, state change]_
- **Animation**: _[Description — e.g., button depresses 2px, ripple effect from click point]_
- **Duration**: _[e.g., 200ms]_
- **Purpose**: _[feedback / attention / delight]_

---

_Add more as needed._

---

## Loading Animations

| Context | Animation type | Appears after | Description |
|---|---|---|---|
| _[e.g., Page load]_ | _[skeleton screen]_ | _[0ms]_ | _[Grey placeholder shapes matching content layout]_ |
| _[e.g., API call]_ | _[spinner]_ | _[500ms]_ | _[Appears if response takes longer than threshold]_ |
| _[e.g., Button submit]_ | _[inline spinner]_ | _[0ms]_ | _[Replaces button text with spinner]_ |

---

## Scroll Behaviour

| Context | Behaviour | Details |
|---|---|---|
| _[e.g., Navigate to error]_ | _[smooth scroll]_ | _[Scroll to first error, offset by header height]_ |
| _[e.g., New content loaded]_ | _[no scroll]_ | _[Maintain current position]_ |
| _[e.g., Back to top]_ | _[smooth scroll]_ | _[On explicit user action only]_ |
