struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>( _s: SGen<T>) {}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct Point<T>{
    x: T,
    y: T
}

struct Point2<T, U> {
    x: T,
    y: U
}

struct Val<T> {
    val: T
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    fn mixup<V, W>( self, other: Point3<V, W>) -> Point3<T, W> {
        Point3 {
            x: self.x,
            y: other.y
        }
    }
}

struct Point4<T> {
    x: T,
    y: T
}

impl Point4<f32> {
    fn distance_from_origin(&self) -> f32 {
        ( self.x.powi(2) + self.y.powi(2) ).sqrt()
    }
}

fn main() {

    // 1
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(1));

    generic::<char>( SGen('a') );
    generic(SGen(String::from("ga")));

    // 2 
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    // 3
    let integer = Point{ x: 5, y: 10};
    let float = Point{ x: 5.0, y: 10.0};

    //4
    let p = Point2{x: 5, y: "hello".to_string() };

    // 5
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());

    // 6
    let p1 = Point3 { x: 5, y: 10 };
    let p2 = Point3 { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    // 7
    let p = Point4{ x: 5.0, y: 10.0};
    println!("{}",p.distance_from_origin());
}
