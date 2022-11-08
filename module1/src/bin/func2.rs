// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
let num1 = 5;
let num2 = 7;
println!("{:?}",sum(num1,num2));
}
fn sum(val1:i32, val2:i32) -> i32 {
    val1 + val2
}
