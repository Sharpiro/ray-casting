pub fn div_mod(left: u32, right: u32) -> (u32, u32) {
    let quotient = left / right;
    let modulus = left - quotient * right;
    (quotient, modulus)
}
