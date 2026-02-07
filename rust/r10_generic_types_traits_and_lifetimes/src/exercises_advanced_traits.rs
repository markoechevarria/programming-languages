/*
pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
  type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
  fn is_null(&self) -> bool;
}
*/

// 1
struct Container(i32, i32);
trait Contains {
    type A;
    type B;
    
    fn contains(&self, _: Self::A, _: Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    
    fn contains(&self, number_1: i32, number_2: i32) -> bool {
        (self.0 == number_1) && (self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

// 2
use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// 3
trait UsernameWidget { fn get(&self) -> String; }

trait AgeWidget { fn get(&self) -> u8; }

struct Form { username: String, age: u8, }

impl UsernameWidget for Form {
    fn get(&self) -> String { self.username.clone() }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 { self.age }
}

trait Pilot { 
    fn fly(&self) -> String; 
}

trait Wizard { 
    fn fly(&self) -> String; 
}
struct Human;

impl Pilot for Human {
    fn fly(&self) -> String { 
        String::from("This is your captain speaking.") 
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String { 
        String::from("*waving arms furiously*") 
    }
}

// 4
trait Person { fn name(&self) -> String; }
trait Student: Person { fn university(&self) -> String; }
trait Programmer { fn fav_language(&self) -> String; }
trait CompSciStudent: Programmer + Student { fn git_username(&self) -> String; }

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String
}

impl Person for CSStudent {
    fn name(&self) -> String { self.name.clone() }
}

impl Student for CSStudent {
    fn university(&self) -> String { self.university.clone() }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String{ self.fav_language.clone() }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String{ self.git_username.clone() }
}

// 5
use std::fmt;

struct Pretty (String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    // 1
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        number_1, number_2,
        container.contains(number_1, number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
    
    // 2
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 }, Point { x: 1, y: 3 });

    // 3
    let form = Form{ username: "rustacean".to_owned(), age: 28, };

    println!("{}", UsernameWidget::get(&form));
    
    let username = UsernameWidget::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = AgeWidget::get(&form);
    let age2 = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);

    let person = Human;

    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
    assert_eq!(Wizard::fly(&person), "Up!");
    assert_eq!(Human::fly(&person), "*waving arms furiously*");

    // 4
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string()
    };

    println!("{}", comp_sci_student_greeting(&student));

    // 5
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}

fn difference< C: Contains >(container: &C) -> i32 {
    container.last() - container.first()
}
