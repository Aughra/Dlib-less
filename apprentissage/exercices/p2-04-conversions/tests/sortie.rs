// Checks the program output, and that integers are converted with `as`.
use std::process::Command;

const SOURCE: &str = include_str!("../src/main.rs");

#[test]
fn affiche_les_conversions() {
    let sortie = Command::new(env!("CARGO_BIN_EXE_p2-04-conversions"))
        .output()
        .expect("the program should start");
    let texte = String::from_utf8(sortie.stdout).expect("output should be UTF-8");
    let attendu = "37 °C = 98.6 °F\nmoyenne = 13.5\n";
    assert_eq!(
        texte, attendu,
        "\n--- the program does not print the expected lines ---"
    );
}

#[test]
fn les_donnees_sont_des_entiers() {
    assert!(
        SOURCE.contains("let celsius: i32 = 37;"),
        "keep `let celsius: i32 = 37;` as given"
    );
    assert!(
        SOURCE.contains("let note1: i32 = 12;") && SOURCE.contains("let note2: i32 = 15;"),
        "keep `let note1: i32 = 12;` and `let note2: i32 = 15;` as given"
    );
    assert!(
        SOURCE.contains("as f64"),
        "convert the integers with `as f64`"
    );
}
