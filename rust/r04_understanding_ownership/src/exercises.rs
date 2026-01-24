fn main() {

    // 1

    let x = String::from("Hello world");
    let y = x.clone();

    println!("{}, {}", x, y);

    // 2

    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);
    println!("{}", s2);

    // 3
    
    let s = give_ownership();
    println!("{}", s);

    // 4
    
    let s = String::from("Hello world");
    println!("{}", s);
    print_str(s);

    // 5

    let x = (1,2, (), "Hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y );

    // 6
    
    let s = String::from("hello");
    let mut s1 = s;
    s1.push_str("world");

    // 7

    let x = Box::new(5);
    let mut y = Box::new(3);
    *y = 4;
    assert_eq!(*x, 5);

    // 8

    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    println!("{:?}", t.1);

    // 9

    let t = (String::from("hello"), String::from("world"));
    let ( s1, s2 ) = t.clone() ;
    println!("{:?}, {:?}, {:?}", s1, s2, t);


    // 1

    let x = 5;
    let p = &x;
    println!("The memory address of x is {:p}", p);

    // 2
    
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);

    // 3

    let mut s = String::from("hello, ");
    borrow_object(&s);

    // 4

    let mut s = String::from("hello, ");
    push_str(&mut s);

    // 5

    let mut s = String::from("hello, ");
    let mut p = s;
    p.push_str("world");

    // 6 
    
    let c = 'a';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));

    // 7

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // 8

    let mut s = String::from("hello, ");
    borrow_object_new(&mut s);

    // 9 

    let mut s = String::from("hello, ");
    borrow_object_re_new(&s);
    s.push_str("world");

    // 10

    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    // println!("{}", r1);

    // 11

    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    let _s = s.clone().into_bytes();
    s
}

fn print_str(s: String) {
    println!("{}", s)
}
fn borrow_object(_s: &String) {}
fn push_str(s: &mut String) {
    s.push_str("world");
}
fn get_addr(r: &char) -> String {
    format!("{:p}", r) 
}
fn borrow_object_new(s: &mut String) {}
fn borrow_object_re_new(s: &String) {}
