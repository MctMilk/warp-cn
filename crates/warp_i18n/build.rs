//! Compile-time validation for the embedded Fluent bundles.
//!
//! Responsibilities:
//! - Parse every `bundles/<locale>/*.ftl` and abort the build on syntax errors.
//! - Emit `OUT_DIR/key_index.rs` containing a `phf::Set<&'static str>` of keys defined
//!   in `bundles/en/`. Downstream code may `include!` it for runtime sanity checks.
//! - Re-run when bundle contents change.

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let bundles_dir = manifest_dir.join("bundles");
    println!("cargo:rerun-if-changed={}", bundles_dir.display());

    let mut en_keys: BTreeSet<String> = BTreeSet::new();
    let mut errors: Vec<String> = Vec::new();

    for entry in walkdir::WalkDir::new(&bundles_dir).into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|s| s.to_str()) != Some("ftl") {
            continue;
        }
        println!("cargo:rerun-if-changed={}", entry.path().display());
        let source = match fs::read_to_string(entry.path()) {
            Ok(s) => s,
            Err(e) => {
                errors.push(format!("read {}: {e}", entry.path().display()));
                continue;
            }
        };
        match fluent_syntax::parser::parse(source.as_str()) {
            Ok(resource) => {
                if is_en(entry.path(), &bundles_dir) {
                    for entry in resource.body {
                        if let fluent_syntax::ast::Entry::Message(msg) = entry {
                            en_keys.insert(msg.id.name.to_string());
                        }
                    }
                }
            }
            Err((_, parse_errors)) => {
                for err in parse_errors {
                    errors.push(format!("{}: {err:?}", entry.path().display()));
                }
            }
        }
    }

    if !errors.is_empty() {
        for e in &errors {
            eprintln!("warp_i18n: ftl error: {e}");
        }
        panic!("warp_i18n: {} ftl parse error(s); aborting build", errors.len());
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let dest = out_dir.join("key_index.rs");
    let mut file = fs::File::create(&dest).expect("create key_index.rs");
    let mut builder = phf_codegen::Set::new();
    for k in &en_keys {
        builder.entry(k.as_str());
    }
    writeln!(
        file,
        "pub static EN_KEY_INDEX: phf::Set<&'static str> = {};",
        builder.build()
    )
    .expect("write key_index.rs");
    writeln!(file, "pub const EN_KEY_COUNT: usize = {};", en_keys.len()).unwrap();
}

fn is_en(path: &Path, bundles_dir: &Path) -> bool {
    path.strip_prefix(bundles_dir)
        .ok()
        .and_then(|p| p.components().next())
        .map(|c| c.as_os_str() == "en")
        .unwrap_or(false)
}
