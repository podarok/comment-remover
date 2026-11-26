use comment_remover::core::{CommentRemover, TreeSitterLanguage};

#[test]
fn python_shebang_survives_and_other_comments_go() {
    let py = CommentRemover::new(TreeSitterLanguage::Python, None);
    let out = py.process_str("#!/usr/bin/env python3\n# note\nx = 1  # trailing\n").unwrap();
    assert!(out.starts_with("#!/usr/bin/env python3\n"), "{out:?}");
    assert!(!out.contains("note") && !out.contains("trailing"), "{out:?}");
}

#[test]
fn hash_bang_later_in_file_is_an_ordinary_comment() {
    let py = CommentRemover::new(TreeSitterLanguage::Python, None);
    let out = py.process_str("x = 1\n#!/bin/sh not a shebang here\n").unwrap();
    assert!(!out.contains("#!"), "{out:?}");
}

#[test]
fn rust_inner_attribute_is_not_a_shebang() {
    let rs = CommentRemover::new(TreeSitterLanguage::Rust, None);
    let out = rs.process_str("#![allow(dead_code)]\n// gone\nfn main() {}\n").unwrap();
    assert!(out.starts_with("#![allow(dead_code)]") && !out.contains("gone"), "{out:?}");
}

#[cfg(feature = "bash")]
#[test]
fn bash_shebang_survives() {
    let sh = CommentRemover::new(TreeSitterLanguage::Bash, None);
    let out = sh.process_str("#!/bin/bash\n# usage\necho hi # trailing\n").unwrap();
    assert!(out.starts_with("#!/bin/bash\n"), "{out:?}");
    assert!(!out.contains("usage") && !out.contains("trailing"), "{out:?}");
}
