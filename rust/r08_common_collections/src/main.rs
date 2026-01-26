use std::{collections::HashMap, num};

fn main() {

    let _v: Vec<i32> = Vec::new();
    let _v = vec![1,2,3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let mut v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("Theres no thid element.")
    } 

    let first = &v[0];
    // v.push(10);
    println!("The first element is {first}");

    let mut v = vec![100, 200, 300];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        println!("{}", *i);
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    let mut v = vec![1,2,3];
    v.pop();


    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

//     let s = s1 + "-" + &s2 + "-" &s3 ;
    let s = format!("{s1}-{s2}-{s3}");
    
    let hello = "abcdefgh";
    let s = &hello[0..4];

    for c in "abcdefgh".chars() {
        println!("{c}");
    }

    for c in "abcdefgh".bytes() {
        println!("{c}");
    }

    // hashmap
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name) { 
        Some(something) => something,
        None => &0
    };
    let score2 = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score of team {team_name} is {score} / {score2}");

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Green")).or_insert(100);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut wordsMap = HashMap::new();
    let text = "Hello world wonderful world";
    for word in text.split_whitespace() {
        wordsMap.entry(word).or_insert(0);
        wordsMap.insert(word, wordsMap.get(&word).copied().unwrap_or(0) + 1);
        // let count = map.entry(word).or_insert(0);
        // *count += 1;
    }
    for (key, value) in &wordsMap {
        println!("{key}: {value}");
    }

    // Median and mode
    
    let array_numbers = [2,3,4,5,6,21,12,2,2,2,8,10];
    let numbers: Vec<usize> = array_numbers.to_vec();
 
    println!("Median of {:?} is {}", numbers, median(&numbers));
    println!("Mode of {:?} is {}", numbers, mode(&numbers));
    median(&numbers);
    mode(&numbers);

    let sorted_array = counting_sort(&numbers);
    println!("No sorted array {:?}", numbers);
    println!("Sorted array {:?}", sorted_array);
}

pub fn median(numbers: &Vec<usize>) -> usize {
    let sorted_array = counting_sort(&numbers);
    let len_vector = len_vector(&numbers);
    let index_median = len_vector / 2 ;

    println!("The median of vector is {}", sorted_array[index_median-1]);

    sorted_array[index_median-1]
}

pub fn mode(numbers: &Vec<usize>) -> usize {
    let mut numbers_map = HashMap::new();
    for i in numbers {
        numbers_map.entry(i).or_insert(0);
        numbers_map.insert(i, numbers_map.get(&i).copied().unwrap_or(0) + 1);
    }

    let max_value = numbers_map.values().max().unwrap_or(&0);

    for (key, value) in &numbers_map {
        if value == max_value {
            return **key;
        } 
    }
    
    0

}

pub fn counting_sort(numbers: &Vec<usize> ) -> Vec<usize> {
    let mut numbers: Vec<usize> = numbers.clone();
    let len_vector = len_vector(&numbers);
    let max_vector = max_vector(&numbers);
   
    let mut cnt_arr: Vec<usize> = vec![0; max_vector + 1];
    let mut ans: Vec<usize> = vec![0; len_vector];

    for val in &mut numbers {
        if *val > 0 {
            cnt_arr[*val] += 1;    
        }
    }
    
    for (i, &_) in cnt_arr.clone().iter().enumerate() {
        if i >= 1 {
            cnt_arr[i] = cnt_arr[i] + cnt_arr[i-1];
        }
    }

    for i in (0..len_vector).rev() {
        ans[ cnt_arr[ numbers[i] ] - 1 ] = numbers[i];
        cnt_arr[ numbers[i] ] -= 1;
    }

    ans
}

pub fn len_vector(numbers: &Vec<usize>) -> usize {
    let mut counter = 0;
    for _ in numbers {
        counter += 1;
    }
    counter
}

pub fn max_vector(numbers: &Vec<usize>) -> usize {
    let mut max = match numbers.get(0) {
        Some(something) => something,
        None => return 0
    };

    for i in numbers {
        if i > max {
            max = i
        } 
    }

    *max
}
