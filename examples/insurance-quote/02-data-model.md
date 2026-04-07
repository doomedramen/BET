# 02 — Data Model

## Entities

### ENT-001: Driver

The person requesting the insurance quote.

| Field | Type | Required | Source | Description |
|---|---|---|---|---|
| firstName | string | yes | user input | Driver's first name |
| lastName | string | yes | user input | Driver's last name |
| dateOfBirth | date | yes | user input | Used for age calculation |
| age | number | yes | calculated | Derived from dateOfBirth. Formula: current year - birth year, adjusted for month/day |
| occupation | string | yes | user input | Driver's current occupation |
| licenceType | string | yes | user input | Full, provisional, international |
| yearsHeld | number | yes | user input | Years driving licence has been held |

---

### ENT-002: Vehicle

The vehicle to be insured.

| Field | Type | Required | Source | Description |
|---|---|---|---|---|
| registrationNumber | string | yes | user input | UK vehicle registration number |
| make | string | yes | api / user input | Vehicle manufacturer (from lookup or manual) |
| model | string | yes | api / user input | Vehicle model (from lookup or manual) |
| year | number | yes | api / user input | Year of manufacture |
| engineSize | number | no | api / user input | Engine capacity in cc |
| fuelType | string | no | api / user input | Petrol, diesel, electric, hybrid |
| estimatedValue | number | yes | user input | User's estimate of current vehicle value |
| usage | string | yes | user input | Personal, commute, business |

---

### ENT-003: Quote Request

The compiled request sent to insurance providers.

| Field | Type | Required | Source | Description |
|---|---|---|---|---|
| id | string | yes | system-generated | Unique quote request identifier |
| driver | Driver | yes | calculated | Reference to completed Driver entity |
| vehicle | Vehicle | yes | calculated | Reference to completed Vehicle entity |
| coverType | string | yes | user input | Comprehensive, third-party, third-party fire & theft |
| startDate | date | yes | user input | When coverage should begin |
| createdAt | datetime | yes | system-generated | When the quote request was created |

---

### ENT-004: Quote

An individual quote returned from a provider.

| Field | Type | Required | Source | Description |
|---|---|---|---|---|
| id | string | yes | system-generated | Unique quote identifier |
| quoteRequestId | string | yes | system-generated | Links to the originating quote request |
| provider | string | yes | api | Name of the insurance provider |
| annualPrice | number | yes | api | Annual premium in GBP |
| monthlyPrice | number | no | api | Monthly premium if available |
| coverLevel | string | yes | api | What's covered |
| excess | number | yes | api | Voluntary excess amount |

---

## Relationships

| Entity A | Relationship | Entity B | Description |
|---|---|---|---|
| Driver | HAS MANY | Quote Request | A driver can request multiple quotes over time |
| Vehicle | HAS MANY | Quote Request | A vehicle can be in multiple quote requests |
| Quote Request | HAS ONE | Driver | Each request is for one driver |
| Quote Request | HAS ONE | Vehicle | Each request is for one vehicle |
| Quote Request | HAS MANY | Quote | Each request returns multiple quotes from providers |
