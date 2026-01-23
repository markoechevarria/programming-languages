fn main() {
    let _x: i32 = 20;
    let mut y: i32 = 10;

    y += 1; 

    println!("{y}");
    printn_message("wazaaaaaa");

    let (x,y) = (1,2);
    let mut x = x;
    x += 2;

    let (x,y);
    (x, ..) = (3,4);
    [.., y] = [1,2];

    println!("The value of x {x} and y {y}")
}

fn printn_message(message: &str) {
    println!("Message: <{message}>");
}
