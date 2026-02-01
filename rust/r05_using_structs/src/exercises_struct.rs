struct Person {
    name: String,
    age: u8,
    hobby: String
}

struct Unit;
trait SomeTrait {}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Person2 {
    name: String,
    age: u8,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    
    // 1
    let age = 30;
    let _p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("walk")
    };

    // 2
    let u = Unit;
    do_something_with_unit(u);

    // 3
    let v = Color(0, 127, 255);
    check_color(v);

    // 4
    let age = 18;
    let mut p = Person2 {
        name: String::from("sunface"),
        age,
    };
    p.age = 30;
    p.name = String::from("sunfei");

    // 5
    let new_person = build_person(String::from("waza"), 18);

    // 6
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1
    };

}

fn check_color(p: Color) {
    let Color(x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
 }

fn do_something_with_unit(u: Unit) {   }

fn build_person(name: String, age: u8) -> Person2 {
    Person2 { name, age }
}
