use std::fs;
use std::path::{Path, PathBuf};

use comment_remover::core::language::TreeSitterLanguage;
use comment_remover::core::parser::parse;
use comment_remover::core::remover::{CommentRemover, compile_keep_patterns};
use tree_sitter::Node;

fn fixtures() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ts");
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.file_name().unwrap().to_string_lossy().contains(".in."))
        .collect();
    v.sort();
    v
}

fn expected_path(input: &Path) -> PathBuf {
    let name = input.file_name().unwrap().to_string_lossy().replace(".in.", ".out.");
    input.with_file_name(name)
}

fn strip(lang: TreeSitterLanguage, src: &str) -> String {
    let keep = compile_keep_patterns(&[]).unwrap();
    CommentRemover::with_keep_patterns(lang, Some(1), keep).process_str(src).unwrap()
}

fn tokens(lang: TreeSitterLanguage, src: &str) -> Vec<(String, String)> {
    fn only_comments(n: Node) -> bool {
        let mut c = n.walk();
        let mut any = false;
        for ch in n.named_children(&mut c) {
            any = true;
            if ch.kind() != "comment" {
                return false;
            }
        }
        any
    }
    fn walk(n: Node, src: &str, out: &mut Vec<(String, String)>) {
        if n.kind() == "comment" || (n.kind() == "jsx_expression" && only_comments(n)) {
            return;
        }
        if n.child_count() == 0 {
            if n.byte_range().is_empty() {
                return;
            }
            let text = src[n.byte_range()].to_string();
            if n.kind() == "jsx_text" {
                if let Some(last) = out.last_mut() {
                    if last.0 == "jsx_text" {
                        last.1.push_str(&text);
                        return;
                    }
                }
            }
            out.push((n.kind().to_string(), text));
            return;
        }
        let mut c = n.walk();
        for ch in n.children(&mut c) {
            walk(ch, src, out);
        }
    }
    let tree = parse(src, lang).unwrap();
    let mut out = Vec::new();
    walk(tree.root_node(), src, &mut out);
    out
}

fn lang_for(p: &Path) -> TreeSitterLanguage {
    TreeSitterLanguage::detect_from_path(p).expect("language compiled in")
}

#[test]
fn every_fixture_matches_its_expected_output() {
    let all = fixtures();
    assert!(all.len() >= 20, "fixtures missing: {}", all.len());
    for input in all {
        let src = fs::read_to_string(&input).unwrap();
        let want = fs::read_to_string(expected_path(&input)).unwrap();
        assert_eq!(strip(lang_for(&input), &src), want, "{}", input.display());
    }
}

#[test]
fn output_is_a_fixed_point() {
    for input in fixtures() {
        let want = fs::read_to_string(expected_path(&input)).unwrap();
        assert_eq!(strip(lang_for(&input), &want), want, "not idempotent: {}", input.display());
    }
}

#[test]
fn only_comments_change_never_code_tokens() {
    for input in fixtures() {
        let lang = lang_for(&input);
        let src = fs::read_to_string(&input).unwrap();
        let out = strip(lang, &src);
        assert_eq!(tokens(lang, &src), tokens(lang, &out), "code tokens changed: {}", input.display());
    }
}

#[test]
fn crlf_input_gives_crlf_equivalent_output() {
    for input in fixtures() {
        let src = fs::read_to_string(&input).unwrap();
        let want = fs::read_to_string(expected_path(&input)).unwrap();
        let got = strip(lang_for(&input), &src.replace('\n', "\r\n"));
        assert_eq!(got.replace("\r\n", "\n"), want, "crlf: {}", input.display());
        assert!(!got.contains("\n\n\n"), "crlf collapse: {}", input.display());
    }
}

#[test]
fn ts_uses_non_jsx_grammar_and_tsx_uses_jsx_grammar() {
    let ts = "const a = <number>value; // c\n";
    assert_eq!(strip(TreeSitterLanguage::TypeScript, ts), "const a = <number>value;\n");
    let tsx = "const a = <div>{/* c */}</div>;\n";
    assert_eq!(strip(TreeSitterLanguage::Tsx, tsx), "const a = <div></div>;\n");
}

#[test]
fn preserve_layout_mode_keeps_line_count() {
    let keep = compile_keep_patterns(&[]).unwrap();
    let r = CommentRemover::with_keep_patterns(TreeSitterLanguage::TypeScript, Some(usize::MAX), keep);
    let src = "// a\nconst x = 1; // b\n/* c\nd */\nconst y = 2;\n";
    let out = r.process_str(src).unwrap();
    assert_eq!(out.matches('\n').count(), src.matches('\n').count());
    assert!(!out.contains("//") && !out.contains("/*"));
}

fn walk_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            if p.file_name().unwrap() != ".git" {
                walk_files(&p, out);
            }
        } else if matches!(
            p.extension().and_then(|x| x.to_str()),
            Some("ts" | "tsx" | "mts" | "cts" | "js" | "jsx" | "mjs" | "cjs")
        ) {
            out.push(p);
        }
    }
}

fn has_error(lang: TreeSitterLanguage, src: &str) -> bool {
    parse(src, lang).unwrap().root_node().has_error()
}

#[test]
fn corpus_token_equivalence() {
    let Ok(root) = std::env::var("RMCM_CORPUS") else { return };
    let mut files = Vec::new();
    walk_files(Path::new(&root), &mut files);
    let (mut checked, mut skipped, mut bad) = (0, 0, Vec::new());
    for f in files {
        let Ok(src) = fs::read_to_string(&f) else { continue };
        let Some(lang) = TreeSitterLanguage::detect_from_path(&f) else { continue };
        if has_error(lang, &src) {
            skipped += 1;
            continue;
        }
        let out = strip(lang, &src);
        checked += 1;
        let reason = if tokens(lang, &src) != tokens(lang, &out) {
            "tokens"
        } else if strip(lang, &out) != out {
            "not idempotent"
        } else if has_error(lang, &out) {
            "parse error after strip"
        } else {
            continue;
        };
        bad.push(format!("{reason}: {}", f.display()));
    }
    eprintln!("corpus: checked {checked}, skipped (parse errors) {skipped}, bad {}", bad.len());
    assert!(bad.is_empty(), "token/idempotency/parse mismatches: {bad:#?}");
}

