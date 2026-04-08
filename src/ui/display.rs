use crate::models::BetSection;

pub fn print_banner() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║           BET Spec Generator (bet v0.1.0)               ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
}

#[allow(dead_code)]
pub fn print_summary(sections: &[BetSection]) {
    let included_count = sections.iter().filter(|s| s.included).count();
    println!("\n✅ Selected {} sections:\n", included_count);

    for s in sections.iter().filter(|s| s.included) {
        let tag = if s.required { "REQUIRED" } else { "OPTIONAL" };
        println!("   {} {}", s.filename, tag);
    }
}

pub fn print_generation_complete(generated: usize, spec_path: &std::path::Path) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                    Generation Complete                   ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("📁 Created {} section files in: {}\n", generated, spec_path.display());
    println!("Next steps:");
    println!("  1. Start filling in sections (see docs/GUIDE.md)");
    println!("  2. Follow the recommended fill order in spec/README.md");
    println!("  3. Validate with docs/COMPLETENESS-CHECKLIST.md");
    println!("  4. Generate BDD scenarios with docs/BDD-GENERATION.md");
    println!("  5. Map spec sections to test layers with docs/TESTING.md");
    println!("  6. Commit your spec/ directory to your project repository\n");
}

pub fn print_add_complete(added: usize, spec_path: &std::path::Path) {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                    Update Complete                       ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("📁 Added {} section(s) to: {}\n", added, spec_path.display());
    println!("Run `bet -o {}` again to add more sections.\n", spec_path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default());
}
