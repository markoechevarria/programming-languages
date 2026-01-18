fn main() {
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;

    println!("The value of x is: {x}");

    x = 10;
    println!("The value of const is: {THREE_HOURS_IN_SECONDS}");

    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Shadowing
    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is {spaces}");

    //  Data types
    
    let var_integer: u8 = 20;
    let var_floating: f32 = 3.0;
    
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    let var_boolean: bool = false;
    let var_char: char = 'a';

    let var_tup: (i32, f64, u8) = (500, 6.4, 1);
    let (var_x, var_y , var_z) = var_tup;
    println!("The value of var_y is {var_y}.");

    let first_value_tup = var_tup.0;
    println!("The value of var_x is {first_value_tup}.");
    
    let var_array = [1,2,3,4,5];
    let var_array: [i32; 5] = [1,2,3,4,5];
    let var_array_repeted = [3; 5];
    let var_array_0 = var_array[0];

    println!("The value of var_array_0 is {var_array_0}.");


    // functions
    
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    fn expression_statements() {
        let y = {
            let x = 3;
            x + 1
        };
        println!("The value of y is: {y}");
    };

    fn five() -> i32 {
        5
    }


    let value_five = five();
    println!("The value of five is: {value_five}");


    // Control flow
    
    let var_control_flow = 3;

    if var_control_flow < 5 {
        println!("condition was true");
    } else if var_control_flow < 10 {
        println!("condition was false");
    } else {
        println!("condition was ..."); 
    } 

    let condition: bool = true;
    let var_if = if condition { 5 } else {6};
    println!("The value of var_if is: {var_if}");


    // loops
    
    loop {
        println!("stop with ctrl + c");
        break;
    };

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            };
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    };
    println!("LIFTOFF!!!");



    let a = [10,20,30,40,50];
    let mut index = 10;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }


    for element in a {
        println!("The value is: {element}");
    }


    for number in (1..4).rev() {
        println!("{number}");
    }
}
