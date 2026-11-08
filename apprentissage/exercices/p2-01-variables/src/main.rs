fn main() {
    let mut tirelire = 10; // a box with a value inside, sealed

    println!("tirelire = {tirelire}");

    tirelire = 20;

    println!("tirelire = {tirelire}");

    let mut score = 0; // `mut`: this box can be opened and refilled
    score += 5;
    println!("score = {score}");
}
