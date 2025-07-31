use std::env;
use std::path::Path;


fn main() {
    //Collect command line arguments, skipping the first one (the program name)
    let args: Vec<String> = env::args().skip(1).collect();

    let target = if args.is_empty() {
        env::var("HOME").expect("HOME environment variable not set");
    }else{
        args[0].clone();
    };
}
