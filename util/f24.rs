
fn f32_to_f24(x: f32) -> f32 {
    let mut bytes = x.to_be_bytes();
    bytes[3] = 0;
    f32::from_be_bytes(bytes)
}

fn main() {
    let original = std::f32::consts::PI;
    let truncated = f32_to_f24(original);

    println!("f32: {:#034b} | {:08X} => {}", original.to_bits(), original.to_bits(), original);
    println!("f24: {:#034b} | {:08X} => {}", truncated.to_bits(), truncated.to_bits(), truncated);
}
