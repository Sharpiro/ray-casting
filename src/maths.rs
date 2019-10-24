pub fn div_mod(left: usize, right: usize) -> (usize, usize) {
    let quotient = left / right;
    let modulus = left - quotient * right;
    (quotient, modulus)
}
