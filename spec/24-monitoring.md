# 24 — Monitoring & Observability [OPTIONAL]

> Dashboards, alerts, logging, and operational visibility.
> Include this for systems that need production monitoring beyond basic analytics.
>
> **Dependencies**: [09-external-interactions.md](09-external-interactions.md), [10-events-side-effects.md](10-events-side-effects.md), [12-error-taxonomy.md](12-error-taxonomy.md)

---

## Key Metrics

_What the system should measure and expose._

| Metric | Description | Source | Alert threshold |
|---|---|---|---|
| _[e.g., Quote completion rate]_ | _[% of started quotes that reach results]_ | _[Application events]_ | _[Below 60% over 1 hour]_ |
| _[e.g., API response time (p95)]_ | _[95th percentile response time]_ | _[Application metrics]_ | _[Above 2s over 5 minutes]_ |
| _[e.g., External service error rate]_ | _[% of calls that fail]_ | _[HTTP client]_ | _[Above 5% over 10 minutes]_ |
| _[e.g., Active sessions]_ | _[Current concurrent users]_ | _[Session store]_ | _[Above 90% of capacity]_ |

---

## Dashboards

_What operational dashboards should exist._

### DASH-001: _[Dashboard name]_

- **Audience**: _[ops team / engineering / product / exec]_
- **Refresh rate**: _[real-time / 1 minute / 5 minutes]_
- **Panels**:
  | Panel | Metric | Visualisation |
  |---|---|---|
  | _[panel name]_ | _[metric]_ | _[line chart / counter / gauge / heatmap]_ |

---

## Alerts

_What triggers alerts and who receives them._

### ALERT-001: _[Alert name]_

- **Condition**: _[What triggers the alert — metric threshold, error pattern, health check failure]_
- **Severity**: _[critical / warning / info]_
- **Notify**: _[Who receives it — ops team, on-call, engineering, Slack channel, PagerDuty]_
- **Runbook**: _[Link to investigation/resolution steps, if applicable]_

---

_Add more alerts as needed._

---

## Logging

### Log Levels

| Level | Usage | Example |
|---|---|---|
| _[ERROR]_ | _[Unexpected failures requiring attention]_ | _[Database connection failed, unhandled exception]_ |
| _[WARN]_ | _[Degraded behaviour, recovered failures]_ | _[External service timeout, cache miss, fallback used]_ |
| _[INFO]_ | _[Significant business events]_ | _[Quote requested, user created, state transition]_ |
| _[DEBUG]_ | _[Diagnostic detail for development]_ | _[Request payload, calculation steps, SQL queries]_ |

### Sensitive Data in Logs

- **Never log**: _[Passwords, tokens, full credit card numbers, session secrets]_
- **Mask**: _[PII fields — show first/last character only, or hash]_
- **Redact in production**: _[Request bodies containing personal data]_

---

## Health Checks

| Check | Endpoint | Interval | Timeout | Failure action |
|---|---|---|---|---|
| _[e.g., Application]_ | _[/health]_ | _[30s]_ | _[5s]_ | _[Restart container / alert ops]_ |
| _[e.g., Database]_ | _[/health/db]_ | _[30s]_ | _[5s]_ | _[Alert ops, degrade gracefully]_ |
| _[e.g., External service]_ | _[/health/ext]_ | _[60s]_ | _[10s]_ | _[Switch to fallback mode]_ |
