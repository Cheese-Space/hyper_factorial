use std::{env, process::exit};
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
    let name: &String = &args[0];
    if args.len() < 2 {
        println!("usage: {} <number>\nerror: no number provided!", name);
        exit(1);
    }
    let n: u32 = args[1].parse::<u32>().unwrap_or_else(|_err| {
        println!("usage: {} <number>\nerror: invalid number!", name);
        exit(1);
});
    println!("{}", hyper_factorial(n));
}
