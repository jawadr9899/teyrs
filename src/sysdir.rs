use crate::{explorer::show_menu, utils::to_readable_time};
use chrono::{DateTime, Utc};
use std::{
    ffi::OsString,
    fs::{self, Metadata},
    io::{self, Error},
};

pub struct SysDir {
    pub path: String,
}
#[derive(Debug)]
pub struct DirType {
    pub name: OsString,
    pub metadata: Metadata,
}

impl SysDir {
    pub fn new(path: String) -> Self {
        SysDir { path }
    }
    pub fn from(path: String) -> Self {
        SysDir { path }
    }
    pub fn get_dirs(&self) -> Result<Vec<DirType>, std::io::Error> {
        let mut dirs: Vec<DirType> = vec![];
        match fs::read_dir(&self.path) {
            Ok(entries) => {
                for e in entries {
                    match &e {
                        Ok(k) => {
                            let s = DirType {
                                name: k.file_name().to_os_string(),
                                metadata: k.metadata()?,
                            };
                            dirs.push(s);
                        }
                        Err(_) => (),
                    }
                }
                Ok(dirs)
            }
            Err(e) => Err(e),
        }
    }
    pub fn get_as_vecstr(&self) -> Result<Vec<String>, std::io::Error> {
        let dirs = self.get_dirs();
        let mut names: Vec<String> = vec![];
        match dirs {
            Ok(drs) => {
                for i in drs {
                    if let Ok(ref k) = i.name.into_string() {
                        names.push(k.to_string());
                    }
                }
                Ok(names)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_metadata(&self, name: &str) -> Result<Option<Metadata>, std::io::Error> {
        let dirs = self.get_dirs()?;
        let metadata = dirs
            .iter()
            .find(|x| x.name.to_str() == Some(name))
            .map(|x| x.metadata.clone());
        Ok(metadata)
    }
    pub fn refresh(&mut self, dir: SysDir) -> Result<(), std::io::Error> {
        *self = dir;
        self.get_dirs()?;
        show_menu(self)?;
        Ok(())
    }
}
pub struct FileInfo {
    metadata: Metadata,
}
impl FileInfo {
    pub fn new(metadata: Metadata) -> Self {
        FileInfo { metadata }
    }

    pub fn last_accessed(&self) -> Result<DateTime<Utc>, Error> {
        match self.metadata.accessed() {
            Ok(t) => to_readable_time(t),
            Err(e) => {
                println!("Failed to convert into datetime!");
                Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
            }
        }
    }
    pub fn creation_time(&self) -> Result<DateTime<Utc>, Error> {
        match self.metadata.created() {
            Ok(t) => to_readable_time(t),
            Err(e) => {
                println!("Failed to convert into datetime!");
                Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
            }
        }
    }
    pub fn last_modified(&self) -> Result<DateTime<Utc>, Error> {
        match self.metadata.modified() {
            Ok(t) => to_readable_time(t),
            Err(e) => {
                println!("Failed to convert into datetime!");
                Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
            }
        }
    }

    pub fn is_dir(&self) -> bool {
        self.metadata.file_type().is_dir()
    }
    pub fn is_file(&self) -> bool {
        self.metadata.file_type().is_file()
    }
    pub fn is_symlink(&self) -> bool {
        self.metadata.file_type().is_symlink()
    }
    pub fn size_in_bytes(&self) -> u64 {
        self.metadata.len()
    }

    pub fn is_readonly(&self) -> bool {
        self.metadata.permissions().readonly()
    }
}
