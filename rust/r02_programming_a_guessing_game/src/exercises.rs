fn main() {
    let _x: u8 = 32;

    let x: u32 = 5;
    let mut y: u32 = 5;
    y = x;

    let z = 10;

    let v: u16 = 32_u8 as u16;

    let v1 = 251_u8 + 8;
    let v2 = u8::checked_add(251, 3).unwrap();
}
