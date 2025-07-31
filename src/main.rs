use std::env;
use std::path::{Path, PathBuf};
use std::cell::RefCell;

thread_local! {
    static PREV_DIR: RefCell<Option<PathBuf>> = RefCell::new(None);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let current_dir = env::current_dir().expect("Failed to get current dir");

    let target = if args.is_empty() {
        // Default to HOME if no args
        env::var("HOME").expect("HOME not set")
    } else if args[0] == "-" {
        // Use previous directory if available
        let prev = PREV_DIR.with(|p| p.borrow().clone());
        match prev {
            Some(p) => p.to_string_lossy().to_string(),
            None => {
                eprintln!("No previous directory");
                return;
            }
        }
    } else {
        args[0].clone()
    };

    let path = Path::new(&target)
        .canonicalize()
        .unwrap_or_else(|_| {
            eprintln!("No such file or directory: {}", target);
            std::process::exit(1);
        });

    // Save current directory for next run
    PREV_DIR.with(|p| *p.borrow_mut() = Some(current_dir));

    println!("{}", path.display());
}
