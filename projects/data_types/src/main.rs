// #![allow(unused)]
// fn main() {

//     let guess: u32 = "42".parse().expect("Not a number!");
//     println!("guess is: {guess}");

//     let x = 2.0; // f64
//     let y: f32 = 3.0;

//     println!("x and y are: {x} and {y}")
// }

fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat} {z} {c}");

    let tup: (i32, f64, i8) = (500, 6.4, 1);
}