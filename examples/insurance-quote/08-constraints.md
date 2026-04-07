# 08 — Constraints & Blocking Conditions

## Constraints

### CON-001: Ineligible Driver (Underage)

- **Condition**: Driver.age < 17
- **Effect**: Prevent progression past [STATE-001](06-flow-state.md#STATE-001). Cannot generate a quote.
- **Recovery**: None — user cannot proceed. Must be at least 17.
- **Message**: [MSG-008](11-messages.md#MSG-008)

### CON-002: Start Date Too Far Ahead

- **Condition**: QuoteRequest.startDate > today + 30 days
- **Effect**: Prevent progression past [STATE-003](06-flow-state.md#STATE-003)
- **Recovery**: Adjust start date to be within 30 days of today
- **Message**: [MSG-006](11-messages.md#MSG-006)

### CON-003: All Providers Failed

- **Condition**: Every insurance provider returns an error during [STATE-004](06-flow-state.md#STATE-004)
- **Effect**: Cannot display results. Transition to error state.
- **Recovery**: Retry later, or contact support
- **Message**: [MSG-009](11-messages.md#MSG-009)
