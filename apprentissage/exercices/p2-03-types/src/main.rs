fn main() {
    let pommes: i32 = 7; // whole number (integer)
    let prix: f64 = 0.35; // number with a decimal point (float)
    let en_stock: bool = true; // yes or no
    let initiale: char = 'D'; // one single character

    let petit: u8 = 200;
    println!("{}", petit);

    println!("{}", initiale);
    let initiale: char = 'L';
    println!("{}", initiale);

    println!("{pommes} pommes, {prix} € l'une, en stock : {en_stock}, initiale : {initiale}");

    let total = pommes as f64 * prix;
    println!("total = {total}");

    println!("7 / 2 = {}", 7 / 2); // integers: the decimal part is dropped
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);
    println!("{}", -7 / 2);
}
