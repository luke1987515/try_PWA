fn main() {
    // 3.1. Variables and Mutablility
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINT: u32 = 100_000;
    println!("The value of x is: {}", MAX_POINT);

    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    let a: f32 = 3.0; // f32

    println!("The value of z is: {}", z);

    println!("The value of a is: {}", a);

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

}
