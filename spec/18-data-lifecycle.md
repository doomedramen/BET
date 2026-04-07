# 18 — Data Lifecycle & Compliance [REQUIRED]

> How data is created, retained, modified, exported, and deleted.
> Covers compliance requirements (GDPR, CCPA, etc.), consent management, and audit trails.
>
> **Dependencies**: [02-data-model.md](02-data-model.md), [03-permissions.md](03-permissions.md)

---

## Consent & Collection

_What consent is required before data is collected, and how it is obtained._

### CONSENT-001: _[Consent name]_

- **Data covered**: _[Which entities/fields require this consent]_
- **Purpose**: _[Why the data is collected — quote generation, marketing, analytics]_
- **Collection method**: _[Checkbox / implicit via form submission / separate consent step]_
- **Required**: _[yes (blocking) / optional (affects functionality)]_
- **Revocable**: _[yes / no — can the user withdraw consent later?]_
- **Text**: _[LEGAL-XXX](11-messages.md#LEGAL-XXX) — exact consent wording_
- **Stored as**: _[Where and how consent records are kept — timestamp, version, IP]_

---

_Add more consent items as needed._

---

## Retention Rules

_How long data is kept and what triggers its removal._

| Entity | Retention period | Trigger for deletion | Deletion method |
|---|---|---|---|
| _[ENT-XXX]_ | _[e.g., 30 days after last activity]_ | _[Automated / on user request / on account closure]_ | _[Soft delete → hard delete after X days / immediate hard delete / anonymise]_ |

---

## User Rights

_Capabilities the system must provide for data subject rights (GDPR Article 15-22, CCPA, etc.)._

### RIGHT-001: Right to Access

- **What**: _[User can request a copy of all their personal data]_
- **Response format**: _[JSON export / PDF / email / in-app download]_
- **Response time**: _[e.g., within 30 days / immediate download]_
- **Authentication**: _[How is identity verified before providing data]_

### RIGHT-002: Right to Deletion

- **What**: _[User can request deletion of their personal data]_
- **Scope**: _[All data / specific entities / specific fields]_
- **Exceptions**: _[Data that must be retained — legal obligations, active contracts]_
- **Cascading**: _[What happens to related data — e.g., if Driver is deleted, are their QuoteRequests deleted too?]_
- **Confirmation**: _[How is the user informed that deletion is complete]_
- **Timeline**: _[How quickly is deletion executed]_

### RIGHT-003: Right to Rectification

- **What**: _[User can correct inaccurate personal data]_
- **Mechanism**: _[Self-service edit / support request / both]_
- **Impact**: _[Does correcting data invalidate existing quotes, calculations, etc.?]_

### RIGHT-004: Right to Portability

- **What**: _[User can export their data in a machine-readable format]_
- **Format**: _[JSON / CSV / XML]_
- **Scope**: _[Which entities are included]_

---

_Add or remove rights based on applicable regulations. Not all systems require all rights._

---

## Audit Trail

_What actions are recorded for compliance, debugging, or accountability._

| Action | What is logged | Retention | Accessible to |
|---|---|---|---|
| _[e.g., Data created]_ | _[Entity, fields, timestamp, actor]_ | _[7 years]_ | _[System admin / compliance team]_ |
| _[e.g., Data modified]_ | _[Entity, field, old value, new value, timestamp, actor]_ | _[7 years]_ | _[System admin]_ |
| _[e.g., Data deleted]_ | _[Entity, reason, timestamp, actor]_ | _[7 years]_ | _[Compliance team]_ |
| _[e.g., Data exported]_ | _[Entities included, format, timestamp, actor]_ | _[2 years]_ | _[System admin]_ |
| _[e.g., Consent given/withdrawn]_ | _[Consent type, timestamp, method]_ | _[Duration of relationship + 7 years]_ | _[Compliance team]_ |

---

## Data Classification

_Sensitivity levels for data in the system._

| Classification | Description | Handling rules | Entities/fields |
|---|---|---|---|
| _[e.g., PII]_ | _[Personally identifiable information]_ | _[Encrypted at rest, masked in logs, excluded from analytics]_ | _[Driver.firstName, Driver.lastName, Driver.dateOfBirth]_ |
| _[e.g., Sensitive]_ | _[Financial or health data]_ | _[Encrypted at rest + in transit, access logged, no caching]_ | _[Quote.annualPrice, Vehicle.estimatedValue]_ |
| _[e.g., Internal]_ | _[Non-sensitive operational data]_ | _[Standard security controls]_ | _[QuoteRequest.id, QuoteRequest.createdAt]_ |

---

## Applicable Regulations

_Which regulations apply and how they map to the requirements above._

| Regulation | Jurisdiction | Key requirements | Covered by |
|---|---|---|---|
| _[e.g., GDPR]_ | _[EU/UK]_ | _[Consent, access, deletion, portability, data minimisation]_ | _[CONSENT-001, RIGHT-001 through RIGHT-004]_ |
| _[e.g., CCPA]_ | _[California]_ | _[Disclosure, deletion, opt-out of sale]_ | _[RIGHT-001, RIGHT-002]_ |
