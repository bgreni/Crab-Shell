use crate::Shell;
use nix::unistd::{chdir, getcwd};
use std::{fs};
use std::error::Error;

pub trait Basics {
    fn cd(&mut self, new_dir: &str);
    fn pwd_command(&self) -> String;
    // fn ls(&self) -> Result<(), Box<dyn Error>>;
}


impl Basics for Shell {
    fn cd(&mut self, new_dir: &str) {
        match chdir(new_dir) {
            Ok(_) => self.pwd = self.pwd_command(),
            Err(_) => eprintln!("No directory name: {}", new_dir)
        }
    }

    fn pwd_command(&self) -> String {
        return getcwd().unwrap().to_str().unwrap().to_string();
    }

    // fn ls(&self) -> Result<(), Box<dyn Error>> {
    //     for entry in fs::read_dir(self.pwd.clone())? {
    //         let entry = entry?;
    //         let path = entry.path();
    //         let metadata = fs::metadata(&path)?;
    //
    //
    //
    //     }
    //     return Ok(());
    // }
}