# 10 — Events & Side Effects [OPTIONAL]

> What the system emits — analytics, audit logs, webhooks, notifications, emails.
> Things that happen as a consequence of actions.
>
> **Dependencies**: [06-flow-state.md](06-flow-state.md), [05-business-rules.md](05-business-rules.md)
> **Guide**: [docs/GUIDE.md — Events & Side Effects](../docs/GUIDE.md#10--events--side-effects-optional)

---

## Events

### EVT-001: _[Event name]_

- **Trigger**: _[What causes this event — state transition, business rule, user action]_
- **Timing**: _[before / after the triggering action]_
- **Type**: _[analytics / audit / webhook / notification / email]_
- **Payload**: _[What data is included]_
- **Audience**: _[Who/what consumes this — analytics system, user, admin, ops]_

---

### EVT-002: _[Event name]_

- **Trigger**: _[What causes this event]_
- **Timing**: _[before / after]_
- **Type**: _[analytics / audit / webhook / notification / email]_
- **Payload**: _[What data]_
- **Audience**: _[Who consumes this]_

---

_Add more events as needed._
