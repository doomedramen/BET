use inquire::{Confirm, MultiSelect, Text};
use crate::models::BetSection;

pub struct ProjectInfo {
    pub name: String,
    #[allow(dead_code)]
    pub project_type: String,
    #[allow(dead_code)]
    pub include_core: bool,
}

/// Prompt for project name and type (new spec only).
pub fn gather_project_info() -> ProjectInfo {
    let project_name = Text::new("What is your project name?")
        .with_help_message("e.g., Insurance Quote System, E-commerce Checkout")
        .prompt()
        .unwrap_or_else(|_| "My Project".to_string());

    let project_type = inquire::Select::new(
        "What type of project is this?",
        vec!["New project (greenfield)", "Rewrite of existing system"],
    )
    .prompt()
    .unwrap_or("New project (greenfield)");

    println!("\n📋 Project type: {}\n", project_type);

    let include_core = Confirm::new("Include all core behaviour sections? (1-8, 11, 13, 16-18)")
        .with_default(true)
        .with_help_message("These are REQUIRED for a complete spec")
        .prompt()
        .unwrap_or(true);

    ProjectInfo {
        name: project_name,
        project_type: project_type.to_string(),
        include_core,
    }
}

/// Prompt for optional section selection on a new spec.
pub fn gather_section_selection(sections: Vec<BetSection>) -> Vec<BetSection> {
    let optional_sections: Vec<&BetSection> = sections
        .iter()
        .filter(|s| !s.required)
        .collect();

    let optional_names: Vec<String> = optional_sections
        .iter()
        .map(|s| format!("{} — {}", s.filename, s.description))
        .collect();

    println!("\n📦 Select optional sections to include:\n");

    let selected_optionals = MultiSelect::new(
        "Choose optional sections (Space to select, Enter to confirm):",
        optional_names.clone(),
    )
    .prompt()
    .unwrap_or_default();

    let mut updated = sections;
    for section in updated.iter_mut() {
        if !section.required {
            let display_name = format!("{} — {}", section.filename, section.description);
            section.included = selected_optionals.contains(&display_name);
        }
    }

    updated
}

/// Prompt to add missing sections to an existing spec.
pub fn gather_add_sections(missing: Vec<&BetSection>) -> Vec<String> {
    let names: Vec<String> = missing
        .iter()
        .map(|s| format!("{} — {}", s.filename, s.description))
        .collect();

    if names.is_empty() {
        return Vec::new();
    }

    println!("\n📦 Sections not yet in your spec:\n");

    MultiSelect::new(
        "Choose sections to add (Space to select, Enter to confirm):",
        names.clone(),
    )
    .prompt()
    .unwrap_or_default()
}
