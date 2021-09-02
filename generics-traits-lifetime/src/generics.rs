pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// This function can take a list of any type

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_v2<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in 0..list.len() {
        if &list[i] > largest {
            largest = &list[i]
        }
    }
    return largest;
}

pub struct PointOld<T> {
    pub x: T,
    pub y: T,
}

impl<T> PointOld<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

// This impimentation will be available only for 
// instances of f32 type
impl PointOld<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// if x will be an integer and y will be a float
// then the program won't be compiled
// To make it work we can use not only one generic

pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

impl<T: Copy, U> Point<T, U> {
    pub fn mixup<V, W>(&self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

