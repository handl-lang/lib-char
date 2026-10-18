//! Locks the `char` contract surface: the axiom set every backend must
//! satisfy.

use handl_lib_char::CHAR_HNL;

fn axioms() -> Vec<&'static str> {
    CHAR_HNL
        .lines()
        .filter_map(|line| line.trim().strip_prefix("axiom "))
        .filter_map(|rest| rest.split_whitespace().next())
        .collect()
}

#[test]
fn contract_declares_eighteen_axioms() {
    assert_eq!(axioms().len(), 18, "char contract axiom count");
}

#[test]
fn contract_covers_classification_case_and_access() {
    let names = axioms();
    for expected in [
        // classification
        "char.is_whitespace",
        "char.is_alpha",
        "char.is_numeric",
        "char.is_alnum",
        "char.is_uppercase",
        "char.is_lowercase",
        // case mapping
        "char.to_upper",
        "char.to_lower",
        "char.to_title",
        // scalar conversion
        "char.code_point",
        "char.from_code_point",
        // sequence access
        "char.at",
        "char.slice",
        "char.length",
        "char.concat",
        "char.code_point_at",
        // bulk conversion
        "char.chars",
        "char.from_chars",
    ] {
        assert!(names.contains(&expected), "missing axiom {expected}");
    }
}
