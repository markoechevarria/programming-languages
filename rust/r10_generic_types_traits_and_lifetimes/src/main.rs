struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Points<T, U> {
    x: T,
    y: U
}

impl<X1, Y1> Points<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Points<X2, Y2>) -> Points<X1,Y2> {
        Points {
            x: self.x,
            y: other.y
        }
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}


fn main() {
    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    let largest_char = largest_char(&char_list);
    println!("The largest char is {largest_char} ");

    let largest_char = largest(&char_list);
    println!("The largest char is {largest_char} ");

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Points { x: 1.0, y: 3 };
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        println!("{item}");
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}
