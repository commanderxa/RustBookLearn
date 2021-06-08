fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {

        }
    }

    let m = Message::Write(String::from("Hello, World!"));
    m.call();

    struct Ipv4 {

    }

    struct Ipv6 {

    }

    enum IpAddresses {
        V4(Ipv4),
        V6(Ipv6),
    }

    enum States {
        Alaska,
        California,
        NewYork,
        Washington,
        Texas,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(States),
    }

    fn value_in_cents(coin: Coin) -> u8{
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => 25,

        }
    }

    println!("{}", value_in_cents(Coin::Quarter(States::Alaska)));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("Quarter");
    } else {
        count++;
    }
}
