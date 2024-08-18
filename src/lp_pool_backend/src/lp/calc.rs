// 1_000_000 standard for most common tokens on Solana
// even Solana have 1_000_000_000
const PRECISION: u64 = 1_000_000;

pub fn multiply(a: u64, b: u64) -> u64 {
    let result = a * b;
    result / PRECISION
}

pub fn divide(a: u64, b: u64) -> u64 {
    if b == 0 {
        panic!("Division by zero");
    }
    
    let a_scale = a * PRECISION;
    let result = a_scale / b;
    result
}
