# 18 — Data Lifecycle & Compliance

## Consent & Collection

### CONSENT-001: Data Processing for Quote Generation

- **Data covered**: All Driver fields (ENT-001), all Vehicle fields (ENT-002)
- **Purpose**: Generating insurance quotes by sharing data with insurance provider partners
- **Collection method**: Checkbox in [STATE-001](06-flow-state.md#STATE-001) before first progression
- **Required**: yes (blocking) — cannot proceed to STATE-002 without consent
- **Revocable**: yes — user can withdraw consent, which triggers deletion of all their data
- **Text**: [LEGAL-001](11-messages.md#LEGAL-001)
- **Stored as**: timestamp (UTC), consent version identifier, IP address, user agent

### CONSENT-002: Analytics Cookies

- **Data covered**: Session analytics, page views, funnel progression
- **Purpose**: Improving the quote flow and understanding user behaviour
- **Collection method**: Cookie banner on first visit ([LEGAL-003](11-messages.md#LEGAL-003))
- **Required**: no — analytics still works without this, but data is anonymised
- **Revocable**: yes — via cookie preferences link in footer
- **Text**: [LEGAL-003](11-messages.md#LEGAL-003)
- **Stored as**: cookie preference record (timestamp, choices)

---

## Retention Rules

| Entity | Retention period | Trigger for deletion | Deletion method |
|---|---|---|---|
| Driver (ENT-001) | 30 days after last quote activity | Automated nightly job | Soft delete → hard delete after 7 days |
| Vehicle (ENT-002) | 30 days after last quote activity | Automated nightly job | Soft delete → hard delete after 7 days |
| QuoteRequest (ENT-003) | 30 days after creation | Automated nightly job | Soft delete → hard delete after 7 days |
| Quote (ENT-004) | 24 hours after creation | Automated hourly job | Hard delete (API-sourced, not our data to retain) |
| Consent records | Duration of data retention + 7 years | Manual only — compliance team | Archive to cold storage |
| Audit trail | 7 years | Automated annual archival | Archive to cold storage, then delete |

---

## User Rights

### RIGHT-001: Right to Access

- **What**: User can request a copy of all personal data held about them
- **Response format**: JSON download via self-service (in-app), or PDF via support request
- **Response time**: Immediate for self-service download; within 30 days for support requests (GDPR requirement)
- **Authentication**: Logged-in session for self-service. Identity verification via email + date of birth for support requests.
- **Scope**: All Driver data, Vehicle data, QuoteRequest data. Quote data only if still retained. Consent and audit records included.

### RIGHT-002: Right to Deletion

- **What**: User can request deletion of all their personal data
- **Scope**: Driver (ENT-001), Vehicle (ENT-002), QuoteRequest (ENT-003), Quote (ENT-004)
- **Exceptions**: Consent records and audit trail entries are retained for legal compliance (7 years), but anonymised (personal identifiers removed)
- **Cascading**: Delete Driver → delete all associated Vehicles, QuoteRequests, and Quotes
- **Confirmation**: Email confirmation sent when deletion is complete
- **Timeline**: Soft delete within 24 hours, hard delete within 7 days

### RIGHT-003: Right to Rectification

- **What**: User can correct inaccurate personal data
- **Mechanism**: Self-service — user can edit Driver and Vehicle fields at any point before quote submission. After submission, user must start a new quote with corrected data.
- **Impact**: Editing data after a quote has been calculated invalidates existing quotes. User must recalculate.

### RIGHT-004: Right to Portability

- **What**: User can export their data in a machine-readable format
- **Format**: JSON
- **Scope**: Driver, Vehicle, QuoteRequest entities. Quotes excluded (third-party data).

---

## Audit Trail

| Action | What is logged | Retention | Accessible to |
|---|---|---|---|
| Quote flow started | Session ID, timestamp, entry point | 7 years | Engineering, compliance |
| Driver data entered | Fields provided (not values), timestamp | 7 years | Compliance |
| Vehicle lookup performed | Registration number (masked: XX12 XXX), result status, timestamp | 7 years | Engineering, compliance |
| Quote request submitted | QuoteRequest ID, provider count, timestamp | 7 years | Engineering, compliance |
| Quotes received | QuoteRequest ID, provider names, quote count, timestamp | 7 years | Engineering, compliance |
| Consent given | Consent type, version, timestamp, IP, user agent | Duration + 7 years | Compliance |
| Consent withdrawn | Consent type, timestamp, method | Duration + 7 years | Compliance |
| Data deletion requested | Entities deleted, timestamp, method (self-service/support) | 7 years | Compliance |
| Data export requested | Entities included, format, timestamp | 7 years | Compliance |

---

## Data Classification

| Classification | Description | Handling rules | Entities/fields |
|---|---|---|---|
| PII | Personally identifiable information | Encrypted at rest (AES-256), masked in logs, excluded from analytics, access logged | Driver.firstName, Driver.lastName, Driver.dateOfBirth, Driver.occupation |
| Sensitive | Data that could be used for discrimination or profiling | Encrypted at rest, not cached, access restricted | Driver.age (derived), Driver.licenceType, Vehicle.registrationNumber |
| Financial | Monetary values and pricing | Encrypted at rest, not logged in plaintext | Quote.annualPrice, Quote.monthlyPrice, Vehicle.estimatedValue |
| Internal | Non-sensitive operational data | Standard security controls | QuoteRequest.id, QuoteRequest.createdAt, Quote.id |

---

## Applicable Regulations

| Regulation | Jurisdiction | Key requirements | Covered by |
|---|---|---|---|
| UK GDPR | United Kingdom | Lawful basis, consent, access, deletion, portability, data minimisation | CONSENT-001, RIGHT-001 through RIGHT-004, retention rules |
| UK PECR | United Kingdom | Cookie consent, electronic marketing | CONSENT-002, LEGAL-003 |
| FCA regulations | United Kingdom | Fair treatment, clear communication, record keeping | LEGAL-002, audit trail, CONTENT-001 |
