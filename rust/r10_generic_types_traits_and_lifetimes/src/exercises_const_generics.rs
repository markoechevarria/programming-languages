#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N]
}

// impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {}

fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>();
    foo::<2021>();
    foo::<{20 * 100 + 20 * 10 + 1}>();
    // foo::<{M+1}>();
    // foo::<{ std::mem::size_of::<T>() }>();
    // let _: [u8; M];
    // let _: [u8; std::mem::size_of::<T>() ];
}

pub struct MinSlice<T, const N: usize> {
    pub head: [T; N],
    pub tail: [T],
}

struct Array<T, const N: usize> {
    data : [T; N]
}

fn check_size<T>(_val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{}

fn main() {
    let slice: &[u8] = b"Hello, world";
    let reference: Option<&u8> = slice.get(6);
    assert!(reference.is_some());

    // 1
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 22, 33],
        },
        Array {
            data: [1, 2, 4]
        }
    ];

    // 2
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);

    // 3
    check_size([0u8; 767]); 
    check_size([0i32; 191]);
    /*
    check_size(["hello你好"; 47]);
    check_size([(); 31].map(|_| "hello你好".to_string()));
    check_size(['中'; 191 ]);
*/

}

fn print_array<T: std::fmt::Debug, const N: usize>( arr: [T; N]) {
    println!("{:?}", arr);
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
