# 02 — Data Model [REQUIRED]

> Every piece of data the system touches — entities, fields, types, ownership.
>
> **Dependencies**: [01-capabilities.md](01-capabilities.md)
> **Guide**: [docs/GUIDE.md — Data Model](../docs/GUIDE.md#02--data-model-required)
> **Format**: [docs/FORMAT.md — Data Model Format](../docs/FORMAT.md#data-model-format)

---

## Entities

### ENT-001: _[Entity name]_

_[Brief description of what this entity represents]_

| Field | Type | Required | Source | Description |
|---|---|---|---|---|
| _fieldName_ | _string_ | _yes_ | _user input_ | _[Description]_ |
| _fieldName_ | _number_ | _no_ | _calculated_ | _[Description + formula]_ |
| _fieldName_ | _date_ | _yes_ | _api_ | _[Description]_ |
| _fieldName_ | _string_ | _yes_ | _system-generated_ | _[Description]_ |

**Source values**: `user input`, `api`, `calculated`, `system-generated`

---

### ENT-002: _[Entity name]_

_[Brief description]_

| Field | Type | Required | Source | Description |
|---|---|---|---|---|
| | | | | |

---

_Add more entities as needed._

---

## Relationships

_How entities relate to each other._

| Entity A | Relationship | Entity B | Description |
|---|---|---|---|
| _[Entity]_ | HAS MANY | _[Entity]_ | _[Description]_ |
| _[Entity]_ | BELONGS TO | _[Entity]_ | _[Description]_ |
| _[Entity]_ | REFERENCES | _[Entity]_ | _[Description]_ |
