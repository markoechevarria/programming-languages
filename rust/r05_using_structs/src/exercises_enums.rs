enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 1
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i32, Number2::One as i32);

    // 2
    let msg1 = Message::Move{x: 1, y: 2};
    let msg2 = Message::Write(String::from("hello, world!"));

    // 3
    let msg = Message::Move{ x:1, y:1};
    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a,b);
    }

    // 5
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n)  = six {
        println!("{}", n);
    }

} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
