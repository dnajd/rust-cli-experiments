mod cli;
use cli::paths::Paths;

fn main() {
    println!("Hello, world!");
    let paths = Paths::new();
    paths.print_paths(); 
}
