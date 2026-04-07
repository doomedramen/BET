# 09 — External Interactions [OPTIONAL]

> Every system boundary — APIs, services, third-party integrations.
> Include both external services and your own internal services that are called across boundaries.
>
> **Dependencies**: [02-data-model.md](02-data-model.md)
> **Guide**: [docs/GUIDE.md — External Interactions](../docs/GUIDE.md#09--external-interactions-optional)
> **Format**: [docs/FORMAT.md — External Interactions Format](../docs/FORMAT.md#external-interactions-format)

---

## Services

### EXT-001: _[Service name]_

- **Description**: _[What this service does]_
- **Input**: _[What data is sent]_
- **Output**: _[What data is returned]_
- **Failure modes**:
  - Timeout (_[duration]_): _[What happens]_
  - Not found: _[What happens]_
  - Service down: _[What happens]_
  - Bad data: _[What happens]_
- **Fallback**: _[What the system does when this service is unavailable]_

---

### EXT-002: _[Service name]_

- **Description**: _[What this service does]_
- **Input**: _[What data is sent]_
- **Output**: _[What data is returned]_
- **Failure modes**:
  - _[Mode]_: _[What happens]_
- **Fallback**: _[Fallback behaviour]_

---

_Add more services as needed._
