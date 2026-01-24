use std::ops::{Range, RangeInclusive};
use std::mem::size_of_val;

fn main() {
    let _x: u8 = 32;

    let x: u32 = 5;
    let mut _y: u32 = 5;
    _y = x;

    let _z = 10;

    let _v: u16 = 32_u8 as u16;

    let _v1 = 251_u16 + 8;
    let _v2 = u8::checked_add(251, 3).unwrap();

    let _v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{}", _v);
    assert!( _v == 1597);

    let _x = 1_000.00_1;
    let _y: f32 = 0.12;
    let _z = 0.01_f64;

    assert_eq!(type_of(&_x), "f64".to_string());

//    assert!(0.1+0.2==0.3);

    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    println!("{}", sum);
    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c);
    }

    assert_eq!((1..5), Range{start: 1, end: 5} );
    assert_eq!((1..=5), RangeInclusive::new(1,5) );

    assert!(1u32 + 2 == 3 );
    assert!(1i32 - 2 == -1 );
    assert!(1i8 - 2 == -1 );
    assert!( 3 * 50 == 150 );
    assert!( 9 / 3 == 3 );
    assert!(24 % 5 == 4 );

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is {}", 0x80u32 >> 2);


    // char bool and unit
    
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);
    print_char(c1);

    let _f : bool = false;
    let t = false;
    if !t {
        println!("Success!");
    }

    let f = true;
    let t = true && false;
    assert_eq!(!t, f);

    let _v: () = ();
    let v = (2,3);
    assert_eq!(v, implicitly_ret_unit());

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    // statements and expressions
    
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };
    let z = {
        2 * x
    };

    let v1 = {
        let mut x = 1;
        x += 2;
        x
    };
    let v2 = { 
        let x = 3;
        x
    };
    let v3 = sum2(1,2);

    assert_eq!(v1, 3);
    assert!(v2 == 3);
    assert_eq!(v3, 3);

    // functions
    
    let (x,y) = (1,2);
    let s = sum3(x,y);
    assert_eq!(s, 3);

    print();

    never_return();

    let b = true;
    let _v = match b {
        true => 1,
        false => {
            println!("Success");
            0
        }
    };
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn print_char(c: char) {
    println!("{}", c);
}

fn implicitly_ret_unit() -> (u32, u32) {
    (2, 3)
}
fn sum2(x: i32, y: i32) -> i32 {
    x + y
}

fn sum3(x:i32, y:i32) -> i32 {
    x + y
}
fn print() -> () {
    println!("succes");
}
fn never_return() -> () {
    println!("do nothing");
}
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(1),
        _ => {never_return_fn(); return None}
    }
}
fn never_return_fn() -> () {}
