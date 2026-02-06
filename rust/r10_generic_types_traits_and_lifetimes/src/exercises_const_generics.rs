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

fn main() {
    let slice: &[u8] = b"Hello, world";
    let reference: Option<&u8> = slice.get(6);
    assert!(reference.is_some());

    let slice: &[u8] = b"Hello, world";
    let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
    let value: u8 = minslice.head[6];
    assert_eq!(value, b' ')     
}
