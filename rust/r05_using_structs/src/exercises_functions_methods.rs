struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

struct Rectangle2 {
    width: u32,
    height: u32
}

impl Rectangle2 {
    fn area(self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }
    
    // 3

    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }

    // 4
    pub fn new() -> Self {
        Self {
            color: String::from("red")
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

#[derive(Debug)]
enum TrafficLightColor2 {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor2 {
    fn color(&self) -> &str {
        match self {
            TrafficLightColor2::Red => "red",
            TrafficLightColor2::Yellow => "yellow",
            TrafficLightColor2::Green => "green",
        }
    }
}
 

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 1
    let rect1 = Rectangle2 { width: 30, height: 50 };
    assert_eq!(rect1.area(), 1500);

    // 2
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    light.show_state();
    println!("{:?}", light);

    // 4
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    // 6
    let c = TrafficLightColor2::Yellow;
    assert_eq!(c.color(), "yellow");

}
