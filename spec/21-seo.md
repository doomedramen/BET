# 21 — SEO & Discoverability [OPTIONAL]

> URL structure, metadata, structured data, and indexing rules.
> Include this for public-facing web applications.
>
> **Dependencies**: [01-capabilities.md](01-capabilities.md), [06-flow-state.md](06-flow-state.md)

---

## URL Structure

| Page / State | URL pattern | Example |
|---|---|---|
| _[e.g., Home]_ | _[/]_ | _[example.com/]_ |
| _[e.g., Start quote]_ | _[/quote/start]_ | _[example.com/quote/start]_ |
| _[e.g., Quote results]_ | _[/quote/{id}/results]_ | _[example.com/quote/abc123/results]_ |

### URL Rules

- **Trailing slashes**: _[include / exclude / redirect]_
- **Case sensitivity**: _[lowercase only / case-insensitive with redirect]_
- **Parameter format**: _[kebab-case / camelCase / snake_case]_

---

## Page Metadata

| Page / State | Title | Description | Indexable |
|---|---|---|---|
| _[page]_ | _[Page title — include brand]_ | _[Meta description, max 160 chars]_ | _[yes / no (noindex)]_ |

### Canonical URLs

_Pages with potential duplicate content._

| Page | Canonical URL | Reason |
|---|---|---|
| _[page variant]_ | _[canonical URL]_ | _[e.g., Query parameters don't change content]_ |

---

## Structured Data

_Schema.org markup or other structured data for search engines._

### _[Schema type]_

- **Type**: _[e.g., Organization, Product, FAQ, BreadcrumbList]_
- **Applied to**: _[Which page(s)]_
- **Fields**: _[Key properties populated]_

---

## Indexing Rules

- **Robots.txt rules**: _[What is blocked from crawling]_
- **Sitemap**: _[yes / no — which pages are included]_
- **Noindex pages**: _[List pages that should not be indexed — e.g., user-specific pages, flow steps]_
- **Pagination**: _[rel="next"/"prev" / canonical to first page / self-referencing canonical]_

---

## Social Sharing

| Platform | Tag type | Content |
|---|---|---|
| _[e.g., Open Graph (Facebook)]_ | _[og:title, og:description, og:image]_ | _[Values]_ |
| _[e.g., Twitter Card]_ | _[twitter:card, twitter:title]_ | _[Values]_ |
