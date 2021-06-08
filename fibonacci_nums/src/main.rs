use std::io;

fn main() {
    println!("Whick n-th element of Fibonacci do you want?:");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Can't read line.");

    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("{}", fibonacci_num(n));
}

fn fibonacci_num(n: i32) -> f64 {
    let Phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let phi: f64 = (1.0 - 5.0_f64.sqrt()) / 2.0;
    (Phi.powi(n) - phi.powi(n)) / 5.0_f64.sqrt()
}
