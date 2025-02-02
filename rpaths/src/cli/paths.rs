use std::env;

pub struct Paths {
    exec_dir: String,
    current_dir: String,
}

impl Paths {
    pub fn new() -> Self {
        let current_exe = env::current_exe().unwrap();
        let exec_dir = current_exe.parent().unwrap();
        let current_dir = env::current_dir().unwrap();

        Paths {
            exec_dir: exec_dir.display().to_string(),
            current_dir: current_dir.display().to_string(),
        }
    }

    pub fn print_paths(&self) {
        println!(
            "{:<20}: {}",
            "Exec dir",
            self.exec_dir
        );
        println!(
            "{:<20}: {}",
            "Current dir",
            self.current_dir
        );
    }
}