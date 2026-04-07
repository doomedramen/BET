use crate::models::BetSection;

/// Scan an existing spec directory and mark which sections already exist.
pub fn detect_existing_sections(sections: &mut [BetSection], spec_path: &std::path::Path) -> bool {
    if !spec_path.exists() {
        return false;
    }

    let mut found_any = false;
    for section in sections.iter_mut() {
        if spec_path.join(&section.filename).exists() {
            section.included = true;
            found_any = true;
        }
    }

    found_any
}
