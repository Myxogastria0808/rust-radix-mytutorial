use radix_fmt::radix;

fn main() {
    let hex_36bit: &str = "ZZZZ";
    let hex_10bit: u32 = u32::from_str_radix(hex_36bit, 36).unwrap();
    println!("10bit: {}", hex_10bit);
    let return_hex_36bit = radix(hex_10bit, 36).to_string().to_uppercase();
    println!("36bit: {}", return_hex_36bit);
}
