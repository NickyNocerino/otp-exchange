use std::fmt::Write;
use std::fs;
use std::cmp;
use std::path::Path;
use std::path::PathBuf;


use rand::prelude::*;
use zip_extensions::*;

use crate::traits::GetData;
use crate::datasheet::DataSheet;

const MAX_BYTES:usize = 1024;
const MAX_SHEETS:usize = 16;

pub struct DataBook{
    pub size: usize,
    pub max_size:usize,
    pub location: Box<String>,
    pub lens:[usize;MAX_SHEETS],
}

impl DataBook {
    pub fn new() -> Self{
        Self {
            size: 0,
            max_size: MAX_SHEETS,
            location: Box::new("".to_owned()),
            lens: [0usize;MAX_SHEETS]
        }
    }

    pub fn new_zeros(target_dir:&str) -> Self {
        fs::create_dir_all(target_dir).expect("failed to create directory");
        for i in 0..MAX_SHEETS {
            let file_name = format!("DATASHEET{:#09}.bin", i);
            let full_path = format!("{}/{}", target_dir,file_name);
            DataSheet::new_zeros().to_file(&full_path);

        }
        Self {
            size:MAX_SHEETS,
            max_size: MAX_SHEETS,
            location: Box::new(target_dir.clone().to_owned()),
            lens: [MAX_BYTES;MAX_SHEETS]
        }
    }

    pub fn new_random(target_dir:&str) -> Self {
        fs::create_dir_all(target_dir).expect("failed to create directory");
        for i in 0..MAX_SHEETS {
            let file_name = format!("DATASHEET{:#09}.bin", i);
            let full_path = format!("{}/{}", target_dir,file_name);
            DataSheet::new_random().to_file(&full_path);

        }
        Self {
            size:MAX_SHEETS,
            max_size: MAX_SHEETS,
            location: Box::new(target_dir.clone().to_owned()),
            lens: [MAX_BYTES;MAX_SHEETS]
        }
    }

    pub fn from_dir(target_dir:&str) -> Self {
        if !Path::new(target_dir).exists() {
            panic!("Trying to create databook from directory that does not exist")
        }
        let mut count: usize = 0;
        let mut lens = [0usize;MAX_SHEETS];
        for i in 0..MAX_SHEETS {
            let file_name = format!("DATASHEET{:#09}.bin", i);
            let full_path = format!("{}/{}", target_dir,file_name);
            if !Path::new(&full_path).exists() {
                return Self {
                    size:i,
                    max_size: MAX_SHEETS,
                    location: Box::new(target_dir.clone().to_owned()),
                    lens: lens
                };
            }
            let tempsheet = DataSheet::from_file(&full_path);
            lens[i] = tempsheet.size_bytes;
        }
        Self {
            size:MAX_SHEETS,
            max_size: MAX_SHEETS,
            location: Box::new(target_dir.clone().to_owned()),
            lens: lens
        }
    }

    pub fn from_vec(target_dir:&str, data:&Vec<u8>) -> Self {
        let size = cmp::min(data.len(), MAX_BYTES*MAX_SHEETS);
        let mut count: usize = 0;
        let mut lens = [0usize;MAX_SHEETS];
        fs::create_dir_all(target_dir).expect("failed to create directory");
        for i in 0..MAX_SHEETS {
            let file_name = format!("DATASHEET{:#09}.bin", i);
            let full_path = format!("{}/{}", target_dir,file_name);
            let mut temp = Vec::<u8>::new();
            for j in 0..MAX_BYTES {
                if count < size {
                    temp.push(data[count]);
                }
                else {
                    lens[i] = temp.len();
                    DataSheet::from_vec(&temp).to_file(&full_path);
                    return Self {
                        size:i+1,
                        max_size: MAX_SHEETS,
                        location: Box::new(target_dir.to_owned()),
                        lens: lens
                    };
                }
                count+=1;
            }
            lens[i] = MAX_BYTES;
            DataSheet::from_vec(&temp).to_file(&full_path);
        }

        Self {
            size:MAX_SHEETS,
            max_size: MAX_SHEETS,
            location: Box::new(target_dir.to_owned()),
            lens: lens
        }
    }

    pub fn from_zip(target_file:&str, target_dir:&str) -> Self {
        if !Path::new(target_file).exists() {
            panic!("Trying to create databook from zip file that does not exist")
        }
        let _ = zip_extract(&PathBuf::from(target_file), &PathBuf::from(target_dir))
        .expect("failed to extract from zip file");
        Self::from_dir(target_dir)
    }

    pub fn to_zip(&self, target_file:&str ) {
        zip_create_from_directory( &PathBuf::from(target_file), &PathBuf::from(*self.location.clone()))
        .expect("failed to make zip");
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let out = Vec::<u8>::new();
        out
    }
}
