# 01 — Capabilities

## System Overview

**System name**: Insurance Quote Engine

**Purpose**: Allow users to receive insurance price quotes based on their personal and vehicle data.

---

## Actors

| Actor | Type | Description |
|---|---|---|
| User | human | Person seeking an insurance quote |
| Quote Engine | internal system | Calculates and aggregates quotes from providers |
| Vehicle Lookup Service | external system | Returns vehicle details from registration number |
| Insurance Providers | external system | Third-party insurers that return quote prices |

---

## Capabilities

### CAP-001: Get Insurance Quote

**Goal**: Allow a user to receive insurance price quotes based on their driver and vehicle information.

**Actors**: User, Quote Engine, Insurance Providers

**Description**: The user provides their personal details and vehicle information. The system validates eligibility, gathers all required data, and submits a quote request to multiple insurance providers. The user receives a list of available quotes to compare.

---

### CAP-002: Vehicle Lookup

**Goal**: Auto-populate vehicle details from a registration number to reduce user effort and improve data accuracy.

**Actors**: User, Vehicle Lookup Service

**Description**: When a user provides a vehicle registration number, the system looks up the vehicle details from an external service and populates the relevant fields. If lookup fails, the user enters details manually.

---

### CAP-003: Resume Partial Quote

**Goal**: Allow a user to return to an in-progress quote without re-entering data.

**Actors**: User

**Description**: If a user leaves the quote flow before completion, their data persists. When they return, they resume from their last valid state.
