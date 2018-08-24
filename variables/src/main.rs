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
    let z = 2.0; // f64

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

    // The Boolean type
    let t = true;
    println!("The value of t is: {}", t);

    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {}", f);

    // The Character Type

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    let tup = (500, 6.4, 1);

    let (k, l, n) = tup;
    println!("The value of k is: {}", k);
    println!("The value of l is: {}", l);
    println!("The value of n is: {}", n);

    let p: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = p.0;
    let six_point_four = p.1;
    let one = p.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);
    println!("The value of a is: {:#?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of months is: {:?}", months);
    println!("The value of months is: {:#?}", months);
    println!("The value of a is: {:?}", a);
    println!("The value of a is: {:#?}", a);

    // 3.3 function
    println!("Hello, world!");

    another_function(5, 6);

    let q = 5;

    let s = {
        let q = 3;
        q + 1
    };

    println!("The value of s is: {}", s);
    println!("The value of q is: {}", q);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Control Flow
    let number = 3;

    if number != 0 {
        println!("number was something other than zero.");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one (x: i32) -> i32 {
    x + 1
}
