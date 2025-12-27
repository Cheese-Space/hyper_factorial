use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[0];
    println!("{}", name);
}
