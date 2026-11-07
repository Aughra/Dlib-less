// Runs the compiled program and compares what it prints with the expected text.
use std::process::Command;

#[test]
fn affiche_la_carte_de_visite() {
    let sortie = Command::new(env!("CARGO_BIN_EXE_p1-02-carte"))
        .output()
        .expect("the program should start");
    let texte = String::from_utf8(sortie.stdout).expect("output should be UTF-8");
    let attendu = "Projet : dlib-less\nLangage : Rust\nÉtape : P1\n";
    assert_eq!(
        texte, attendu,
        "\n--- the program does not print the expected card ---"
    );
}
