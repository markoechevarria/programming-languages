fn main() {
    // 1
    let arr: [u32; 5] = [1,2,3,4,5];
    assert!(arr.len() == 5);

    // 2
    let _arr0 = [1,2,3];
    let arr: [char; 3] = ['a', 'b', 'c'];
    assert!(std::mem::size_of_val(&arr) == 12);

    // 3
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);

    // 4
    let _arr: [u8; 3] = [1,2,3];

    // 5
    let arr = ['a', 'b', 'c'];
    let ele = arr[0];
    assert!( ele == 'a' );

    // 6
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    let _name0 = names.get(0).unwrap();
    let _name1 = &names[1];
}
