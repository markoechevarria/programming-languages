enum Direction {
    East,
    West,
    North,
    South,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MyEnum {
    Foo,
    Bar
}

enum Foo {
    Bar(u8)
}

enum Foo2 {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {

    // 1
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        },
        _ => println!("West"),
    };

    // 2
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0
    };
    assert_eq!(binary, 1);

    // 3
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,0,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    // 4
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
    for ab in alphabets {
        assert!(matches!( ab, alphabets))
    }

    // 5
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        // if e == MyEnum::Foo {
        if matches!( e, MyEnum::Foo ) {
            count += 1;
        }
    }
    assert_eq!(count, 2);

    // 6 
    let o = Some(7);
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }

    // 7
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
    }

    // 8
    let a = Foo2::Qux(10);

    match a {
        Foo2::Bar => println!("match foo::bar"),
        Foo2::Baz => println!("match foo::baz"),
        _ => println!("match others")
    }

    // 9 
    let age = Some(30);
    if let Some(age) = age {
        let age = Some(age);
       assert_eq!(age, Some(30));
    }
    
    match age {
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Move{ x: a, y: b} => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        }
        __ => println!("no data in these variants")
    }
}
