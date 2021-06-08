fn main() {
    println!("{}", c_to_f(0.0));
    println!("{}", f_to_c(30.0));
}

fn f_to_c(value: f64) -> f64 {
    (value - 32.0) * (5.0/9.0)
}

fn c_to_f(value: f64) -> f64 {
    (value * (9.0/5.0)) + 32.0
}
