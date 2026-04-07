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
- **API Contract**:
  - **Method**: _[GET / POST / PUT / DELETE / GraphQL query]_
  - **Endpoint**: _[URL pattern or service identifier]_
  - **Authentication**: _[API key / OAuth / mTLS / none]_
  - **Request schema**:
    ```
    {
      "_field_": "_type — description_",
      "_field_": "_type — description_"
    }
    ```
  - **Response schema**:
    ```
    {
      "_field_": "_type — description_",
      "_field_": "_type — description_"
    }
    ```
  - **Error codes**:
    | Code | Meaning | Maps to |
    |---|---|---|
    | _[e.g., 404]_ | _[Not found]_ | _[COND-XXX / fallback behaviour]_ |
    | _[e.g., 429]_ | _[Rate limited]_ | _[Retry with backoff]_ |
  - **Rate limits**: _[Provider-imposed limits, if any]_
  - **Data transformation**: _[TRANS-XXX](02-data-model.md#TRANS-XXX) — how request/response maps to entities_

---

### EXT-002: _[Service name]_

- **Description**: _[What this service does]_
- **Input**: _[What data is sent]_
- **Output**: _[What data is returned]_
- **Failure modes**:
  - _[Mode]_: _[What happens]_
- **Fallback**: _[Fallback behaviour]_
- **API Contract**:
  - **Method**: _[method]_
  - **Endpoint**: _[endpoint]_
  - **Authentication**: _[auth method]_
  - **Request schema**: _[schema]_
  - **Response schema**: _[schema]_
  - **Error codes**: _[codes and mappings]_

---

_Add more services as needed._
