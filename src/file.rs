use std::fs::File;
use std::io::prelude::*;


pub struct FileStatus {
    path: Option<String>,
    is_new: bool
}

impl Default for FileStatus {
    fn default() -> Self {
        Self { path: None, is_new: true }
    }
}

impl FileStatus {
    pub fn set_path(&mut self, path: &mut String) {
        self.path = Some(path.clone());
    }

    fn _set_is_new(&mut self, status: bool) {
        self.is_new = status;
    }

    pub fn is_new(&self) -> bool {
        self.is_new
    }

    pub fn get_contents(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let file_path = match &self.path {
            Some(path) => path,
            None => return Err("Path not set".into())
        };

        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(e.into())
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(e) => panic!("file not read: {}", e)
        };
        self.is_new = false;
        Ok(contents)
    }

    pub fn save_file(&mut self, contents: &String) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = match &self.path {
            Some(path) => path,
            None => return Err("Path not set".into())
        };
        
        match std::fs::write(file_path, contents) {
            Ok(_) => {
                self.is_new = false;
                Ok(())
            },
            Err(e) => Err(e.into())
        }
    }
}
