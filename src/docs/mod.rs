use std::fs;
use std::path::Path;

const DOC_FILES: &[&str] = &[
    "FORMAT.md",
    "GUIDE.md",
    "COMPLETENESS-CHECKLIST.md",
    "BDD-GENERATION.md",
    "TESTING.md",
];

/// Default GitHub raw content base URL.
/// Can be overridden via BET_DOCS_URL env var.
fn docs_base_url() -> String {
    std::env::var("BET_DOCS_URL").unwrap_or_else(|_| {
        "https://raw.githubusercontent.com/doomedramen/BET/main/docs".to_string()
    })
}

/// Fetch a single doc from GitHub and write it to the docs directory.
fn fetch_doc(docs_path: &Path, filename: &str) -> Result<(), String> {
    let base = docs_base_url();
    let url = format!("{}/{}", base.trim_end_matches('/'), filename);

    let response = ureq::get(&url)
        .call()
        .map_err(|e| format!("Failed to fetch {}: {}", filename, e))?;

    let content = response
        .into_string()
        .map_err(|e| format!("Failed to read content of {}: {}", filename, e))?;

    fs::write(docs_path.join(filename), content)
        .map_err(|e| format!("Failed to write {}: {}", filename, e))?;

    Ok(())
}

/// Fetch all docs from GitHub and write them to spec/docs/.
pub fn generate_all_docs(docs_path: &Path) {
    fs::create_dir_all(docs_path).expect("Failed to create docs directory");

    println!("\n📥 Fetching documentation from GitHub...");

    let mut fetched = 0;
    let mut failed = Vec::new();

    for filename in DOC_FILES {
        match fetch_doc(docs_path, filename) {
            Ok(()) => {
                println!("   ✓ {}", filename);
                fetched += 1;
            }
            Err(e) => {
                println!("   ✗ {} — {}", filename, e);
                failed.push((*filename, e));
            }
        }
    }

    if !failed.is_empty() {
        println!("\n⚠️  {} doc(s) failed to fetch.", failed.len());
        println!("   Set BET_DOCS_URL to a different raw content URL if needed.");
    } else {
        println!("\n✅ All {} docs fetched.", fetched);
    }
}
