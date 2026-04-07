# 11 — Content & Messages [REQUIRED]

> All user-facing text — error messages, help text, legal copy, contextual content, and state messages.
> This section covers ALL text a user encounters, not just error messages.
>
> **Dependencies**: [04-validation-rules.md](04-validation-rules.md), [05-business-rules.md](05-business-rules.md), [08-constraints.md](08-constraints.md)
> **Guide**: [docs/GUIDE.md — Content & Messages](../docs/GUIDE.md#11--content--messages-required)
> **Format**: [docs/FORMAT.md — Messages Format](../docs/FORMAT.md#messages-format)

---

## Messages

_Reactive text triggered by system events — errors, warnings, confirmations._

### MSG-001: _[Message name]_

- **Severity**: _[error / warning / info / success]_
- **Trigger**: _[What causes this message — link to rule, constraint, or state]_
- **Text**: "_[The exact message text. Use {paramName} for dynamic content.]_"
- **Parameters**: _[paramName = description of value]_

---

### MSG-002: _[Message name]_

- **Severity**: _[error / warning / info / success]_
- **Trigger**: _[What causes this message]_
- **Text**: "_[Message text]_"
- **Parameters**: _[if any]_

---

_Add more messages as needed._

---

## State Messages

_Messages shown during specific system states (empty, loading, processing, etc.)._

| State | Message | Severity |
|---|---|---|
| _[e.g., Empty results]_ | _"No results found. Try adjusting your search."_ | info |
| _[e.g., Loading]_ | _"Loading your results..."_ | info |
| _[e.g., Processing]_ | _"We're working on this..."_ | info |
| _[e.g., Resume]_ | _"Welcome back. We saved your progress."_ | info |

---

## Help Text & Field Descriptions

_Contextual help shown alongside fields or sections. This is not error text — it's guidance that helps users understand what to provide and why._

### HELP-001: _[Field or section name]_

- **Context**: _[Which field or section this appears alongside]_
- **Text**: "_[The help text]_"
- **Display**: _[always visible / on focus / on hover / expandable]_

---

_Add help text for any field where the label alone is insufficient. Prioritise fields where users commonly make mistakes or have questions._

---

## Legal & Compliance Text

_Disclaimers, consent text, terms references, and regulatory copy that must appear in the flow._

### LEGAL-001: _[Legal text name]_

- **Location**: _[Where in the flow this appears — reference state]_
- **Text**: "_[Exact text, or reference to external document]_"
- **Interaction**: _[read-only / must acknowledge / must check box / must scroll to bottom]_
- **Required by**: _[regulation, legal team, business policy]_

---

_Add more legal text as needed. Consider: cookie consent, data processing consent, terms of service, privacy notices, regulatory disclaimers._

---

## Contextual Content

_Explanatory content that helps users understand concepts, options, or implications. This is educational/informational, not reactive._

### CONTENT-001: _[Content name]_

- **Context**: _[Where this appears and when — reference state or field]_
- **Text**: "_[The content]_"
- **Purpose**: _[Why is this here — what does it help the user understand?]_

---

_Add more contextual content as needed. Consider: option explainers ("What does comprehensive cover mean?"), section introductions, confirmation summaries._
