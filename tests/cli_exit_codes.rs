use assert_cmd::Command;
use predicates::str::contains;
use std::fs;

fn rmcm() -> Command {
    Command::cargo_bin("rmcm").unwrap()
}

#[test]
fn default_build_lists_typescript_and_tsx() {
    rmcm().arg("--list-languages").assert().success().stdout(contains("typescript")).stdout(contains("tsx"));
}

#[test]
fn only_unsupported_files_is_an_error_even_with_force() {
    let d = tempfile::tempdir().unwrap();
    fs::write(d.path().join("a.svg"), "<svg/>").unwrap();
    fs::write(d.path().join("b.tsv"), "a\tb").unwrap();
    rmcm().args(["-i", "-r", "-f"]).arg(d.path()).assert().failure().stdout(contains("Skipped 2"));
}

#[test]
fn unsupported_files_next_to_supported_ones_are_skipped_not_failed() {
    let d = tempfile::tempdir().unwrap();
    fs::write(d.path().join("a.svg"), "<svg/>").unwrap();
    fs::write(d.path().join("c.ts"), "// c\nlet a = 1;\n").unwrap();
    rmcm().args(["-i", "-r"]).arg(d.path()).assert().success().stdout(contains("Skipped 1"));
    assert_eq!(fs::read_to_string(d.path().join("c.ts")).unwrap(), "let a = 1;\n");
    assert_eq!(fs::read_to_string(d.path().join("a.svg")).unwrap(), "<svg/>");
}

#[test]
fn tsx_files_are_processed_in_place_with_the_jsx_grammar() {
    let d = tempfile::tempdir().unwrap();
    let f = d.path().join("x.tsx");
    fs::write(&f, "const a = <div>{/* c */}</div>; // t\n").unwrap();
    rmcm().args(["-i"]).arg(&f).assert().success();
    assert_eq!(fs::read_to_string(&f).unwrap(), "const a = <div></div>;\n");
}

#[test]
fn explicit_unknown_extension_fails() {
    let d = tempfile::tempdir().unwrap();
    let f = d.path().join("x.zig");
    fs::write(&f, "// c\n").unwrap();
    rmcm().args(["-i"]).arg(&f).assert().failure();
}

#[cfg(not(feature = "typescript"))]
#[test]
fn missing_feature_is_named_in_the_error() {
    let d = tempfile::tempdir().unwrap();
    let f = d.path().join("x.ts");
    fs::write(&f, "// c\n").unwrap();
    rmcm().args(["-i"]).arg(&f).assert().failure().stdout(contains("--features typescript"));
}
