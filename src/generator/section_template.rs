use crate::models::BetSection;
use crate::sections::id_prefix_for_section;

/// Generate the markdown content for a single section file.
pub fn generate_section(section: &BetSection, project_name: &str) -> String {
    let tag = id_prefix_for_section(section.number);

    format!(
        r#"# {section_name}

> Project: {project_name}
> Status: [ ] Not Started  [ ] In Progress  [ ] Complete

---

## Overview

{description}

---

## Entries

<!--
  Add entries using the format below.
  Each entry must have a unique ID ({tag}-XXX).
  See docs/FORMAT.md for syntax conventions.
-->

### {tag}-001: [Title]

- **Description**: [What this entry captures]
- **Details**: [Specifics, rules, conditions]

---

## Notes

<!-- Any additional context, decisions, or open questions -->

"#,
        section_name = section.name,
        project_name = project_name,
        description = section.description,
        tag = tag,
    )
}
