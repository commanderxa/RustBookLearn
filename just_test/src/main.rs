use rand::Rng;

fn main() {

    // let w = rand::thread_rng().gen_range(1..20);
    // let b = rand::thread_rng().gen_range(1..20);

    // let x_arr = [-40, 0, 20, 40, 70, 100];
    // let y_arr = [-40, 32, 68, 104, 158, 212];

    // let epochs = 10;

    // for epoch in 0..=epochs {
    //     for idx in x.iter() {
    //         let x = x_arr[idx];
    //         let y = y_arr[idx];

    //         let y_hat = nn(x, w, b);
    //         let loss = loss(y, y_hat);
            
    //         let w = w - 1 * w_grad(x, y, y_hat);
    //         let b = b - 1 * b_grad(y, y_hat);
    //     }
    // }

    let string = "";
    let mut count = 0;
    for (i, &item) in string.as_bytes().iter().enumerate() {
        if (item == b' ') {
            count += 1;
        }
    }
    count += 1;
    println!("{}", count);
}

fn nn(x: i32, w: i32, b: i32) -> i32 {
    w * x + b
}

fn loss(y: i32, y_hat: i32) -> i32 {
    (y - y_hat).pow(2)
}

fn w_grad(x: i32, y: i32, y_hat: i32) -> i32 {
    2 * (y - y_hat) * x
}

fn b_grad(y: i32, y_hat: i32) -> i32 {
    2 * (y - y_hat)
}
