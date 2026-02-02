use std::fmt::{Display, Debug};

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

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_default(&self) -> String{
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;

    fn summarize_more(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location) 
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content) 
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// impl<T: Display> ToString for T {}

struct ImportantExcerpt<'a> {
    part: &'a str
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

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {result}!");
    
    let novel = String::from("call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    }
    
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

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify4<T: Summary>(item1: &T, item2: &T) {}

pub fn notify5(item: &(impl Summary + Display)) {}

pub fn notify6<T: Summary + Display>(item: &T) {}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

pub fn some_function2<T, U>( t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
{ 0 }

pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false
    }
}

/*
fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        }
    }
}
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/*
fn longest3<'a>( x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

fn first_word( s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

