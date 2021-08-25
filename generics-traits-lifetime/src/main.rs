mod generics;

fn main() {
    println!("Hello, world!");

    let num_list = [10, 400, 20, 32, -10, 77];
    let result = generics::largest(&num_list);
    println!("{}", result);

    let integer = generics::PointOld { x: 5, y: 10, };
    let float = generics::PointOld { x: 1.0, y: 4.0, };
    let mixed = generics::Point { x: 5, y: 4.0, };

    let p = generics::PointOld { x: 5, y: 10, };
    println!("p.x = {}", p.x());

    let p1 = generics::Point { x: 5, y: 10.4, };
    let p2 = generics::Point { x: "Hello", y: 'c', };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
