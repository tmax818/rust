// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // This will print `hello, world!`
// }

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deeply copy the heap data of the string

    println!("{}, world!", s1);
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // this works because integers are stored on the stack
}