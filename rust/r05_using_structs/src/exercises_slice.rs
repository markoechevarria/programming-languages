fn main() {

    // 1
    let arr = [1,2,3];
    let _s1: &[i32] = &arr[0..2];
    let _s2: &str = "hello world" as &str;

    // 2
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    assert!(std::mem::size_of_val(&slice) == 16);

    // 3
    let arr: [i32;5] = [1,2,3,4,5];
    let slice =  &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);    

    // 4
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..=1];
    assert_eq!( slice1, slice2);

    // 5
    let s = "你好，世界";
    let slice = &s[..3];
    assert!(slice == "你");

    // 6
    let mut s = String::from("hello world");
    let letter = first_letter(&s);
    s.clear();
    println!("the first letter is: {}", letter);
}

fn first_letter(s: &str) -> String {
    (&s[..1]).to_string()
}
