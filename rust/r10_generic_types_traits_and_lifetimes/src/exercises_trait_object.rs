trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

// 3
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 4
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

fn static_dispatch<T: Foo>(x: T) {}

fn dynamic_dispatch(x: &dyn Foo) {}

// 5
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() }
}

fn my_function<T: MyTrait>(x: Box<T>) -> T {
    x.f()
}

fn main() {
    // 1
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");

    // 2
    let birds: [&dyn Bird; 2] = [ &Duck, &Swan ];
    let birds2: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

    for bird in birds {
        bird.quack();
    }

    for bird in birds2 {
        bird.quack();
    }

    // 3
    let x = 1.1f64;
    let y = 8u8;

    draw_with_box(Box::new(x));
    draw_with_ref(&y);

    // 4
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    // 5
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

}   

fn hatch_a_bird( option: u32 ) -> Box< dyn Bird>  {
    if option == 1 {
        Box::new(Duck)
    } else {
        Box::new(Swan)
    }
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}
