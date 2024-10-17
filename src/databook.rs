use std::fmt::Write;
use std::fs;

use rand::prelude::*;

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
}
