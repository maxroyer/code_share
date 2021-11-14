use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::str::FromStr;
use rfd;


pub struct FileStatus {
    path: Option<PathBuf>,
    is_new: bool
}

impl Default for FileStatus {
    fn default() -> Self {
        Self { path: None, is_new: true }
    }
}

impl FileStatus {
    pub fn _set_path(&mut self, path: &mut String) {
        let path_buf = match PathBuf::from_str(path) {
            Ok(buf) => buf,
            Err(e) => panic!("Error: {}" ,e)
        };
        self.path = Some(path_buf);
    }

    fn _set_is_new(&mut self, status: bool) {
        self.is_new = status;
    }

    pub fn is_new(&self) -> bool {
        self.is_new
    }

    pub fn get_path_string(&self) -> String {
        match self.path.clone() {
            Some(path) => String::from(path.to_str().unwrap()),
            None => String::from("-NOT SAVED-")
        }
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

    pub fn save_file_as(&mut self, contents: &mut String) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let saved_path = match Self::open_file_save_dialog() {
            Some(path) => path,
            None => return Ok(None)
        };
        match std::fs::write(&saved_path, contents) {
            Ok(_) => {
                self.is_new = false;
                self.path = Some(saved_path);
                return Ok(Some(()))
            },
            Err(e) => {
                let e_msg = format!("File not saved: {}", e);
                return Err(e_msg.into())
            }
        }
    }

    pub fn open_file(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let open_path = match Self::open_file_sel_dialog() {
            Some(p) => p,
            None => return Ok(None)
        };
        self.path = Some(open_path);
        match self.get_contents() {
            Ok(contents) => Ok(Some(contents)),
            Err(e) => Err(e)
        }
    }

    fn open_file_sel_dialog() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .set_directory(std::env::var("HOME").unwrap())
            .pick_file()        
    }
    
    fn open_file_save_dialog() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .set_directory(std::env::var("HOME").unwrap())
            .save_file()
    }
}
