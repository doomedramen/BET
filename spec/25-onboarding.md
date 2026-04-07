# 25 — Onboarding & Empty States [OPTIONAL]

> First-time user experience, empty states before data exists, and feature discovery.
> Include this when the system has distinct first-run behaviour or progressive disclosure.
>
> **Dependencies**: [06-flow-state.md](06-flow-state.md), [11-messages.md](11-messages.md)

---

## First-Time Experience

_What happens when a user encounters the system for the first time._

### First Visit Behaviour

- **Detection**: _[How does the system know this is a first visit — no session, no account, feature flag, cookie]_
- **Differs from returning user**: _[yes / no — describe what's different]_
- **Onboarding flow**: _[Guided tour / tooltip sequence / introductory modal / inline hints / none]_
- **Skippable**: _[yes / no]_
- **Dismissal**: _[Once dismissed, does it come back? After logout? After 30 days? Never?]_

---

## Onboarding Steps

_If there is a guided onboarding, document each step._

### ONBOARD-001: _[Step name]_

- **Trigger**: _[When this step appears]_
- **Content**: _[What is shown — text, pointer to UI element, action prompt]_
- **Completion**: _[What marks this step as done — user clicks, user performs action, automatic]_
- **Skip**: _[Can this step be skipped individually?]_

---

_Add more steps as needed._

---

## Empty States

_What the user sees when a section or page has no data yet._

| Context | Empty state content | Action available |
|---|---|---|
| _[e.g., No quotes yet]_ | _[Message: "You haven't requested any quotes yet."]_ | _[Link/button: "Get a quote"]_ |
| _[e.g., No saved items]_ | _[Message: "Nothing saved yet."]_ | _[Link/button: "Browse items"]_ |
| _[e.g., No search results]_ | _[Message: "No results for '{query}'."]_ | _[Suggestions: "Try different keywords"]_ |
| _[e.g., No notifications]_ | _[Message: "You're all caught up."]_ | _[None]_ |

---

## Progressive Disclosure

_Features or content that are revealed over time or based on user behaviour._

| Feature | Revealed when | Hidden until then |
|---|---|---|
| _[e.g., Advanced filters]_ | _[User has completed 3+ quotes]_ | _[Filter panel not shown]_ |
| _[e.g., Keyboard shortcuts]_ | _[User presses '?' anywhere]_ | _[No indication they exist]_ |

---

## Tooltips & Hints

_Contextual hints that help users understand features._

| Element | Hint text | Display | Shown |
|---|---|---|---|
| _[e.g., Cover type selector]_ | _"Choose the level of protection for your vehicle."_ | _[tooltip on hover / inline below]_ | _[always / first time only]_ |
