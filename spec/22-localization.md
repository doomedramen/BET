# 22 — Localization & Formatting [OPTIONAL]

> Date formats, currency, number formatting, timezone handling, and multi-language support.
> Include this for any system where formatting matters or multiple locales are supported.
>
> **Dependencies**: [02-data-model.md](02-data-model.md), [11-messages.md](11-messages.md)

---

## Default Locale

- **Language**: _[e.g., en-GB]_
- **Date format**: _[e.g., DD/MM/YYYY]_
- **Time format**: _[e.g., HH:mm (24h) / h:mm a (12h)]_
- **Currency**: _[e.g., GBP, displayed as £1,234.56]_
- **Number format**: _[e.g., 1,234.56 — comma for thousands, dot for decimal]_
- **Timezone**: _[e.g., Europe/London / UTC / user's local timezone]_

---

## Supported Locales

_Skip this section if only one locale is supported._

| Locale | Language | Date format | Currency | Number format |
|---|---|---|---|---|
| _[e.g., en-GB]_ | _[English (UK)]_ | _[DD/MM/YYYY]_ | _[£]_ | _[1,234.56]_ |
| _[e.g., en-US]_ | _[English (US)]_ | _[MM/DD/YYYY]_ | _[$]_ | _[1,234.56]_ |
| _[e.g., de-DE]_ | _[German]_ | _[DD.MM.YYYY]_ | _[EUR]_ | _[1.234,56]_ |

### Locale Detection

- **Method**: _[Browser setting / user preference / URL path / domain / manual selection]_
- **Fallback**: _[Default locale if detection fails]_
- **Switchable**: _[Can the user change locale mid-session? If so, what resets?]_

---

## Field-Specific Formatting

_How specific data fields are formatted for display._

| Field | Input format | Display format | Storage format | Notes |
|---|---|---|---|---|
| _[e.g., dateOfBirth]_ | _[DD/MM/YYYY]_ | _[1 January 1990]_ | _[ISO 8601: YYYY-MM-DD]_ | _[Show date picker with locale format]_ |
| _[e.g., annualPrice]_ | _[numeric input]_ | _[£1,234.56]_ | _[integer in pence: 123456]_ | _[Always show 2 decimal places]_ |
| _[e.g., phoneNumber]_ | _[free text]_ | _[+44 7700 900000]_ | _[E.164: +447700900000]_ | _[Validate as UK number]_ |

---

## Pluralisation & Grammar Rules

_Rules for dynamic text that changes based on quantity or context._

| Message context | Singular | Plural | Zero |
|---|---|---|---|
| _[e.g., Quote count]_ | _"1 quote found"_ | _"{count} quotes found"_ | _"No quotes found"_ |
| _[e.g., Days remaining]_ | _"1 day remaining"_ | _"{count} days remaining"_ | _"Expires today"_ |

---

## Translation Management

_Skip if single-language._

- **Translation process**: _[Manual / external service / i18n file-based]_
- **String storage**: _[JSON files / database / i18n platform]_
- **Fallback behaviour**: _[Show default locale text / show key / show blank]_
- **Dynamic content**: _[How are messages with parameters translated?]_
