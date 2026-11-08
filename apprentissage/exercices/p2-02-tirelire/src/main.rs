fn main() {
    let objectif = 20;
    let mut tirelire = 0;

    tirelire += 2;
    println!("lundi : {}", tirelire);

    tirelire += 5;
    println!("mardi : {}", tirelire);

    tirelire += 3;
    println!("mercredi : {}", tirelire);

    println!("reste à économiser : {}", objectif - tirelire);
}
