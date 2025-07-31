use std::env;
use std::path::Path


fn main() {
    let args:Vec<String> = env.args().skip(1).collect();
    println!("Hello, world!");
}
