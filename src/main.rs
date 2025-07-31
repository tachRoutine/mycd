use std::env;        // For args & current dir
use std::path::Path; // For path handling

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let target = if args.is_empty() {
        env::var("HOME").expect("HOME not set")
    } else {
        args[0].clone()
    };

    let path = Path::new(&target)
        .canonicalize()
        .expect("Invalid path");

    println!("{}", path.display());
}
