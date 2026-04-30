//! Compile-fail tests for RemoteString type-level isolation.
//!
//! Covers spec requirement:
//! - (k) trybuild RemoteString compile-fail templates
//!
//! t!() proc-macro only accepts string literals (LitStr). Any non-literal expression —
//! including `RemoteString::as_str()` — is rejected at compile time with a clear error.

#[test]
fn t_macro_rejects_non_literal_keys() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/t_macro_rejects_variable.rs");
    t.compile_fail("tests/compile_fail/t_macro_rejects_expr.rs");
}

#[test]
#[ignore = "proc-macro key index is populated from bundles/en/*.ftl at compile time; this test needs the proc-macro crate to have been built with the key set populated. Run with --include-ignored when key count > 0."]
fn t_macro_rejects_unknown_key_at_compile_time() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/t_macro_rejects_unknown_key.rs");
}
