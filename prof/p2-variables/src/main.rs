// `score = score + 5` is kept on purpose: the lesson shows what clippy says about it.
#[allow(clippy::assign_op_pattern)]
fn main() {
    let tirelire = 10; // a box with a value inside, sealed
    println!("tirelire = {tirelire}");

    let mut score = 0; // `mut`: this box can be opened and refilled
    score = score + 5;
    println!("score = {score}");
}
