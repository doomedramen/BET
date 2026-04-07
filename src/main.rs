mod models;
mod sections;
mod generator;
mod ui;
mod docs;

use clap::Parser;
use inquire::Confirm;
use std::fs;
use std::path::PathBuf;

use models::{all_sections, detect_existing_sections};
use generator::{generate_section, generate_readme};
use docs::generate_all_docs;
use ui::{
    print_banner, print_generation_complete, print_add_complete,
    gather_project_info, gather_section_selection, gather_add_sections,
};

#[derive(Parser, Debug)]
#[command(name = "bet")]
#[command(about = "Generate a BET (Behaviour Extraction Template) spec structure")]
struct Args {
    /// Path where the spec/ directory will be created
    #[arg(short, long, default_value = ".")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let output_path = PathBuf::from(&args.output);
    let spec_path = output_path.join("spec");

    print_banner();

    let mut sections = all_sections();
    let existing = detect_existing_sections(&mut sections, &spec_path);

    if existing {
        // Existing spec — offer to add missing sections
        handle_existing_spec(&spec_path, &mut sections);
    } else {
        // New spec — full questionnaire
        handle_new_spec(&spec_path, &mut sections);
    }
}

fn handle_new_spec(spec_path: &PathBuf, sections: &mut Vec<models::BetSection>) {
    let info = gather_project_info();
    *sections = gather_section_selection(std::mem::take(sections));

    println!("\n📁 Output directory: {}", spec_path.display());

    let confirm = Confirm::new("Generate spec structure here?")
        .with_default(true)
        .prompt()
        .unwrap_or(true);

    if !confirm {
        println!("Aborted.");
        return;
    }

    fs::create_dir_all(spec_path).expect("Failed to create spec directory");

    let mut generated = 0;
    for section in sections.iter().filter(|s| s.included) {
        let content = generate_section(section, &info.name);
        let file_path = spec_path.join(&section.filename);
        fs::write(&file_path, content).expect("Failed to write section file");
        generated += 1;
    }

    generate_readme(spec_path, &info.name, sections);
    generate_all_docs(&spec_path.join("docs"));

    print_generation_complete(generated, spec_path);
}

fn handle_existing_spec(spec_path: &PathBuf, sections: &mut Vec<models::BetSection>) {
    println!("📂 Existing spec found at: {}\n", spec_path.display());

    // Show what's already there
    println!("📋 Existing sections:");
    for s in sections.iter().filter(|s| s.included) {
        let tag = if s.required { "REQUIRED" } else { "OPTIONAL" };
        println!("   ✓ {} ({})", s.filename, tag);
    }

    // Find what's missing
    let missing: Vec<&models::BetSection> = sections.iter().filter(|s| !s.included).collect();

    if missing.is_empty() {
        println!("\n✅ All 26 sections are present. Nothing to add.");
        return;
    }

    println!("\n📦 {} section(s) not yet added:", missing.len());
    for s in &missing {
        let tag = if s.required { "REQUIRED" } else { "OPTIONAL" };
        println!("   ○ {} ({})", s.filename, tag);
    }

    let confirm = Confirm::new("\nAdd sections to your spec?")
        .with_default(true)
        .prompt()
        .unwrap_or(true);

    if !confirm {
        println!("Aborted.");
        return;
    }

    let selected_names = gather_add_sections(missing);

    if selected_names.is_empty() {
        println!("\nNo sections selected. Aborted.");
        return;
    }

    let mut added = 0;
    for section in sections.iter_mut() {
        if !section.included {
            let display_name = format!("{} — {}", section.filename, section.description);
            if selected_names.contains(&display_name) {
                section.included = true;
                added += 1;
            }
        }
    }

    // Only ask for project name for the README — existing files won't be overwritten
    let project_name = inquire::Text::new("Project name (for README):")
        .with_default("My Project")
        .prompt()
        .unwrap_or_else(|_| "My Project".to_string());

    // Generate only the missing section files
    for section in sections.iter().filter(|s| s.included) {
        let file_path = spec_path.join(&section.filename);
        if !file_path.exists() {
            let content = generate_section(section, &project_name);
            fs::write(&file_path, content).expect("Failed to write section file");
        }
    }

    // Regenerate README to reflect current state
    generate_readme(spec_path, &project_name, sections);

    print_add_complete(added, spec_path);
}
