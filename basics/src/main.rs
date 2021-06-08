fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of constant is: {}", MAX_POINTS);

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // floating point numbers
    let x = 2.0;
    let y: f32 = 3.0;

    // math operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quptient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // bool
    let t = true;
    let f: bool = false;

    // characters
    let c = 'z';
    let z = 'Z';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring a tuple
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "Ocober", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // array contains the same value for all elements
    let a = [3; 5];

    // accessing array elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // expressions
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // functions
    another_function(5, 6);

    // functions with returns
    let x = five();
    println!("The value of x is: {}", x);

    // conditions
    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 2, 3, 4");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn another_function(x: i32, y: i32) {
    println!("The sum of x and y is: {}", (x + y));
}

fn five() -> i32 {
    5
}
