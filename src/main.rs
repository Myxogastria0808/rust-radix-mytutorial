fn main() {
    let hex_36bit = "ZZZZ";
    println!("Hex 8-bit: {}", u32::from_str_radix(hex_36bit, 36).unwrap());
}
