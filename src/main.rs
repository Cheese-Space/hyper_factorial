use std::env;
use num_bigint::{self, ToBigUint};
fn hyper_factorial(n: u32) -> num_bigint::BigUint {
    if n == 0 || n == 1 {
        return 1.to_biguint().unwrap(); 
    };
    let mut res: num_bigint::BigUint = 1.to_biguint().unwrap(); 
    for i in 2..n+1 {
        let factor: num_bigint::BigUint = i.to_biguint().unwrap();
        res *= factor.pow(i);
    };
    return res;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[0];
    println!("{}", name);
}
