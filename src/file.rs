use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use rfd;


pub struct FileStatus {
    path: Option<PathBuf>,
    is_unsaved: bool,
    is_new: bool
}

impl Default for FileStatus {
    fn default() -> Self {
        Self { path: None, is_unsaved: true, is_new: true }
    }
}

impl FileStatus {
    pub fn reset(&mut self) {
        self.path = None;
        self.is_unsaved = true;
        self.is_new = true;
    }

    pub fn _set_is_new(&mut self, status: bool) { self.is_new = status; }
    pub fn is_new(&self) -> bool { self.is_new }

    pub fn is_unsaved(&self) -> bool { self.is_unsaved }
    pub fn set_unsaved(&mut self, status: bool) { self.is_unsaved = status; }


    pub fn get_path_string(&self) -> String {
        match self.path.clone() {
            Some(path) => String::from(path.to_str().unwrap()),
            None => String::from("-NOT SAVED-")
        }
    }

    pub fn save_file(&mut self, contents: &String) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = match &self.path {
            Some(path) => path,
            None => return Err("Path not set".into())
        };
        
        match std::fs::write(file_path, contents) {
            Ok(_) => {
                self.is_new = false;
                self.is_unsaved = false;
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
                self.is_unsaved = false;
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
        let old_path = self.path.clone();
        let old_is_unsaved = self.is_unsaved.clone();
        let old_is_new = self.is_new.clone();

        let open_path = match Self::open_file_sel_dialog() {
            Some(p) => p,
            None => return Ok(None)
        };
        self.path = Some(open_path);
        self.is_unsaved = false;
        self.is_new = false;
        match self.get_contents() {
            Ok(contents) => Ok(Some(contents)),
            Err(e) => {
                // Pass error and return to previous state
                self.path = old_path;
                self.is_unsaved = old_is_unsaved;
                self.is_new = old_is_new;
                Err(e)
            }
        }
    }

    fn get_contents(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
            Err(e) => return Err(format!("Error opening file: {}", e).into())
        };
        Ok(contents)
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
