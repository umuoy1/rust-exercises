#![allow(overflowing_literals)]
fn main() {
    let dec = 65.93456f32;

    let int: u8 = dec as u8;
    let character = int as char;

    println!("{} => {} => {}", dec, int, character);

    println!("1000 as a u16: {}", 1000 as u16);
    println!("1000 as a u8: {}", 1000 as u8); // 1000 - 256 - 256 - 256
    println!("1000 as a u16: {}", 1000 as u16);
}
