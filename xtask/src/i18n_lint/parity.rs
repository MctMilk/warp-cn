use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Mismatch {
    pub bundle: String,
    pub kind: MismatchKind,
    pub key: String,
}

#[derive(Debug)]
pub enum MismatchKind {
    MissingInZh,
    MissingInEn,
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = match self.kind {
            MismatchKind::MissingInZh => "missing in zh-CN",
            MismatchKind::MissingInEn => "missing in en",
        };
        write!(f, "parity: {} key {:?} {side}", self.bundle, self.key)
    }
}

/// Compare en/zh-CN bundles file-by-file. Returns the union of missing keys.
pub fn check(bundles_root: &Path) -> Result<Vec<Mismatch>> {
    let en = collect(&bundles_root.join("en"))?;
    let zh = collect(&bundles_root.join("zh-CN"))?;
    let mut mismatches = Vec::new();

    let names: BTreeSet<&String> = en.keys().chain(zh.keys()).collect();
    for name in names {
        let empty = BTreeSet::new();
        let en_keys = en.get(name).unwrap_or(&empty);
        let zh_keys = zh.get(name).unwrap_or(&empty);
        for k in en_keys.difference(zh_keys) {
            mismatches.push(Mismatch {
                bundle: name.clone(),
                kind: MismatchKind::MissingInZh,
                key: k.clone(),
            });
        }
        for k in zh_keys.difference(en_keys) {
            mismatches.push(Mismatch {
                bundle: name.clone(),
                kind: MismatchKind::MissingInEn,
                key: k.clone(),
            });
        }
    }
    Ok(mismatches)
}

fn collect(dir: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.into_path();
        if path.extension().and_then(|s| s.to_str()) != Some("ftl") {
            continue;
        }
        let name = path
            .strip_prefix(dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        out.entry(name).or_default().extend(parse_keys(&text));
    }
    Ok(out)
}

/// Minimal Fluent key extractor: a key starts at the beginning of a line and
/// matches `[A-Za-z][A-Za-z0-9_-]*` followed by ` =`. Comments (`#`) and
/// continuation lines are ignored.
fn parse_keys(text: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for line in text.lines() {
        if line.starts_with('#') || line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        let trimmed = line.trim_end();
        let Some(eq) = trimmed.find('=') else {
            continue;
        };
        let head = trimmed[..eq].trim();
        if head.is_empty() {
            continue;
        }
        let mut chars = head.chars();
        let Some(first) = chars.next() else {
            continue;
        };
        if !first.is_ascii_alphabetic() {
            continue;
        }
        if !chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            continue;
        }
        keys.insert(head.to_string());
    }
    keys
}
