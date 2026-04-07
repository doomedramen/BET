# 09 — External Interactions

## Services

### EXT-001: Vehicle Lookup Service

- **Description**: Returns vehicle details from a UK registration number
- **Input**: Vehicle.registrationNumber
- **Output**: make, model, year, engineSize, fuelType
- **Failure modes**:
  - Timeout (>5s): Show manual entry form, log warning
  - Not found: Trigger [COND-002](07-conditional-logic.md#COND-002) — require manual entry
  - Service down: Show manual entry form, alert ops team
  - Invalid format: Handled before call by [VR-003](04-validation-rules.md#VR-003)
- **Fallback**: Manual vehicle entry (user provides all fields)
- **API Contract**:
  - **Method**: GET
  - **Endpoint**: `https://api.vehiclelookup.example.com/v1/vehicle/{registrationNumber}`
  - **Authentication**: API key in `X-API-Key` header
  - **Request schema**:
    ```
    Path parameter: registrationNumber (string, UK format, spaces removed)
    ```
  - **Response schema**:
    ```
    {
      "make": "string",
      "model": "string",
      "yearOfManufacture": "number",
      "engineCapacity": "number (cc)",
      "fuelType": "string (petrol | diesel | electric | hybrid)"
    }
    ```
  - **Error codes**:
    | Code | Meaning | Maps to |
    |---|---|---|
    | 404 | Vehicle not found | [COND-002](07-conditional-logic.md#COND-002) |
    | 400 | Invalid registration format | Should not occur (validated by [VR-003](04-validation-rules.md#VR-003)) |
    | 429 | Rate limited by provider | Treat as timeout — show manual entry |
    | 503 | Service unavailable | Show manual entry, alert ops |
  - **Rate limits**: Provider allows 1000 requests per hour per API key
  - **Data transformation**: [TRANS-002](02-data-model.md#TRANS-002)

### EXT-002: Insurance Quote Providers

- **Description**: Third-party insurers that return price quotes based on a complete quote request
- **Input**: Complete QuoteRequest entity (driver + vehicle + cover details)
- **Output**: List of Quote entities (provider, price, cover level, excess)
- **Failure modes**:
  - Timeout (>15s per provider): Exclude that provider from results, continue with others
  - Provider returns error: Exclude that provider, continue with others
  - All providers fail: Trigger [CON-003](08-constraints.md#CON-003)
  - Unexpected response format: Log error, exclude provider, continue with others
- **Fallback**: If all providers fail, show error message and suggest retry later
- **API Contract**:
  - **Method**: POST
  - **Endpoint**: `https://api.quoteprovider.example.com/v2/quotes`
  - **Authentication**: OAuth 2.0 client credentials (per-provider credentials)
  - **Request schema**:
    ```
    {
      "applicant": {
        "first_name": "string",
        "last_name": "string",
        "dob": "string (YYYY-MM-DD)",
        "occupation": "string",
        "licence_type": "string",
        "years_held": "number"
      },
      "vehicle": {
        "reg": "string",
        "make": "string",
        "model": "string",
        "year": "number",
        "engine_cc": "number (optional)",
        "fuel": "string (optional)",
        "value_pence": "number",
        "usage": "string"
      },
      "cover": {
        "type": "string (comprehensive | third-party | third-party-fire-theft)",
        "start_date": "string (YYYY-MM-DD)"
      }
    }
    ```
  - **Response schema**:
    ```
    {
      "quotes": [
        {
          "provider": "string",
          "annual_price_pence": "number",
          "monthly_price_pence": "number (nullable)",
          "cover_level": "string",
          "excess_pence": "number"
        }
      ]
    }
    ```
  - **Error codes**:
    | Code | Meaning | Maps to |
    |---|---|---|
    | 400 | Invalid request data | Log error, exclude provider |
    | 422 | Uninsurable risk | Return empty quotes for this provider |
    | 429 | Rate limited | Retry once after 2s, then exclude |
    | 503 | Provider unavailable | Exclude provider, continue with others |
  - **Rate limits**: Varies per provider (typically 500 requests/hour)
  - **Data transformation**: [TRANS-001](02-data-model.md#TRANS-001)
