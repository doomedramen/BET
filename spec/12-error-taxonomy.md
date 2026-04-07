# 12 — Error Taxonomy [OPTIONAL]

> Categories of errors and how each is handled.
> Distinguishes between user errors, system errors, and external errors.
>
> **Dependencies**: [09-external-interactions.md](09-external-interactions.md), [11-messages.md](11-messages.md)
> **Guide**: [docs/GUIDE.md — Error Taxonomy](../docs/GUIDE.md#12--error-taxonomy-optional)

---

## Error Categories

### ERR-CAT-001: User Errors

**Description**: Errors caused by invalid user input or actions.

**Handling strategy**: Show a clear message explaining what went wrong and how to fix it. Do not log as system error.

**Examples**: Invalid form data, missing required fields, unauthorised action attempts.

---

### ERR-CAT-002: System Errors

**Description**: Errors caused by bugs, unexpected state, or infrastructure failures.

**Handling strategy**: Show a generic error message to the user. Log full details for ops/engineering. Alert if frequency exceeds threshold.

**Examples**: Unhandled exceptions, database connection failures, out-of-memory.

---

### ERR-CAT-003: External Errors

**Description**: Errors caused by third-party services or dependencies.

**Handling strategy**: Apply fallback behaviour from [09-external-interactions.md](09-external-interactions.md). Retry transient failures. Alert ops if persistent.

**Examples**: API timeouts, third-party service downtime, unexpected response formats.

---

## Specific Errors

### ERR-001: _[Error name]_

- **Category**: _[ERR-CAT-XXX]_
- **Condition**: _[What triggers this error]_
- **User sees**: _[MSG-XXX](11-messages.md#MSG-XXX) or generic message_
- **Logged**: _[What is logged for ops]_
- **Retry**: _[yes (how many times, with what backoff) / no]_
- **Fallback**: _[What happens if retry fails or no retry]_

---

_Add more errors as needed._
