use radix_fmt::radix;

fn main() {
    let hex_36bit: &str = "000Z";
    let hex_10bit: u32 = u32::from_str_radix(hex_36bit, 36).unwrap();
    println!("10bit: {}", hex_10bit);
    let return_hex_36bit = radix(hex_10bit, 36).to_string().to_uppercase();
    let zero_filling = format!("{:0>4}", return_hex_36bit);
    println!("36bit: {}", zero_filling);
}
