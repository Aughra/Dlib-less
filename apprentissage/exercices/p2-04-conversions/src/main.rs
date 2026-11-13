fn main() {
    let celsius: i32 = 37;
    let note1: i32 = 12;
    let note2: i32 = 15;


    let far = celsius as f64 * 9.0 / 5.0 + 32.0;
    let moyenne = (note1 as f64 + note2 as f64) / 2.0;

    println!("{} °C = {} °F", celsius , far    );
    println!("moyenne = {}", moyenne); 

}
