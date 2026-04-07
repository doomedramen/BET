# 13 — Edge Cases & Exceptions [REQUIRED]

> Everything that could go wrong, categorised.
> This is where most rewrites and new projects fail — the weird cases that nobody thought of.
>
> **Dependencies**: All previous sections
> **Guide**: [docs/GUIDE.md — Edge Cases](../docs/GUIDE.md#13--edge-cases-required)
> **Format**: [docs/FORMAT.md — Edge Cases Format](../docs/FORMAT.md#edge-cases-format)

---

## Edge Cases

### EDGE-001: _[Edge case name]_

- **Category**: _[data / timing / concurrency / multi-session / partial state / permissions / boundary]_
- **Scenario**: _[What happens — the weird situation]_
- **Expected behaviour**: _[How the system should handle it]_
- **References**: _[Link to relevant states, rules, or services]_

---

### EDGE-002: _[Edge case name]_

- **Category**: _[data / timing / concurrency / multi-session / partial state / permissions / boundary]_
- **Scenario**: _[What happens]_
- **Expected behaviour**: _[How to handle it]_
- **References**: _[Links]_

---

_Add more edge cases as needed. Aim for at least 5 — if you have fewer, you haven't looked hard enough._

---

## Category Prompts

_Use these questions to discover edge cases you might have missed._

**Data**: What if a field is null? Empty string? Maximum length? Contains special characters? Is zero? Is negative?

**Timing**: What if two actions happen simultaneously? What if a session expires mid-action? What if data changes between when it was loaded and when it's submitted?

**Concurrency**: What if two users edit the same record? What if the same user has two tabs open? What if a background job processes data that a user is currently editing? What if the user starts the flow on mobile and continues on desktop?

**Multi-session**: What if the same user has the flow open in two tabs? What if they submit in one tab while editing in another? What if they're logged in on two devices? What if session data conflicts?

**Partial state**: What if the user abandons the flow? What if the browser crashes? What if a multi-step process fails partway through?

**Permissions**: What if a user's role changes mid-session? What if a shared resource's permissions change while someone is using it?

**Boundary**: What if an external service returns data in an unexpected format? What if a list has zero items? One item? 10,000 items?
