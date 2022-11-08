// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
   first_name();
   second_name();
}
fn first_name()  {
    let s = String::from("Anurag");
    println!("{}", s);
}
fn second_name() {
    let s = String::from("Pandey");
    println!("{}", s);
}

