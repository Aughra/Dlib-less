// Checks the program output, and how the boxes (variables) are declared.
use std::process::Command;

const SOURCE: &str = include_str!("../src/main.rs");

#[test]
fn affiche_la_semaine() {
    let sortie = Command::new(env!("CARGO_BIN_EXE_p2-02-tirelire"))
        .output()
        .expect("the program should start");
    let texte = String::from_utf8(sortie.stdout).expect("output should be UTF-8");
    let attendu = "lundi : 2\nmardi : 7\nmercredi : 10\nreste à économiser : 10\n";
    assert_eq!(
        texte, attendu,
        "\n--- the program does not print the expected week ---"
    );
}

#[test]
fn objectif_est_scelle() {
    assert!(
        SOURCE.contains("let objectif"),
        "no `let objectif` box found"
    );
    assert!(
        !SOURCE.contains("let mut objectif"),
        "`objectif` never changes: its box should stay sealed (no `mut`)"
    );
}

#[test]
fn tirelire_peut_changer() {
    assert!(
        SOURCE.contains("let mut tirelire"),
        "no `let mut tirelire` box found"
    );
    assert!(SOURCE.contains("+="), "add each day's coins with `+=`");
}
