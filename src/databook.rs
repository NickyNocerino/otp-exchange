use std::fs;
use std::cmp;
use std::path::Path;
use std::path::PathBuf;



use zip_extensions::*;

use crate::traits::GetData;
use crate::datasheet::DataSheet;
use crate::otp::OneTimePad;

const MAX_BYTES:usize = OneTimePad::MAX_BYTES;
const MAX_SHEETS:usize = OneTimePad::MAX_SHEETS;

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
            location: Box::new(target_dir.to_owned()),
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
            location: Box::new(target_dir.to_owned()),
            lens: [MAX_BYTES;MAX_SHEETS]
        }
    }

    pub fn from_dir(target_dir:&str) -> Self {
        if !Path::new(target_dir).exists() {
            panic!("Trying to create databook from directory that does not exist")
        }
        let mut lens = [0usize;MAX_SHEETS];
        for i in 0..MAX_SHEETS {
            let file_name = format!("DATASHEET{:#09}.bin", i);
            let full_path = format!("{}/{}", target_dir,file_name);
            if !Path::new(&full_path).exists() {
                return Self {
                    size:i,
                    max_size: MAX_SHEETS,
                    location: Box::new(target_dir.to_owned()),
                    lens: lens
                };
            }
            let tempsheet = DataSheet::from_file(&full_path);
            lens[i] = tempsheet.size_bytes;
        }
        Self {
            size:MAX_SHEETS,
            max_size: MAX_SHEETS,
            location: Box::new(target_dir.to_owned()),
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
            for _j in 0..MAX_BYTES {
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

    pub fn get_bytes_fast(&self, index:usize, n:usize) -> Vec<u8> {
        if index  + n > self.get_size_bytes() {
            panic!("index access out of bounds")
        }

        let mut out = Vec::<u8>::new();
        let mut count:usize = 0;

        for i in 0..self.size {
            if self.lens[i] + count <= index {
                count  += self.lens[i];
            }
            else if count + self.lens[i] >= index && count < (index + n){
                let file_name = format!("DATASHEET{:#09}.bin", i);
                let full_path = format!("{}/{}", *self.location.clone() ,file_name);
                let sheet = DataSheet::from_file(&full_path);
                out.append(&mut sheet.get_bytes(
                    cmp::max(index as i64 - count as i64, 0) as usize,
                    cmp::min((index +n) - count, self.lens[i]) as usize
                ));
                count += self.lens[i];
            }
            else {
                break;
            }
        }
        out
    }
}

impl GetData for DataBook {
    fn get_size_bytes(&self) -> usize {
        self.lens.iter().sum()
    }
    fn get_max_size_bytes(&self) -> usize {
        self.max_size * MAX_BYTES
    }
    fn get_byte(&self, index:usize) -> u8 {
        if index > self.get_size_bytes() {
            panic!("index access out of bounds")
        }
        let mut remainder:usize = index;
        for i in 0..self.size {
            if self.lens[i] <= remainder {
                remainder -= self.lens[i];
            }
            else {
                let file_name = format!("DATASHEET{:#09}.bin", i);
                let full_path = format!("{}/{}", *self.location.clone() ,file_name);
                let sheet = DataSheet::from_file(&full_path);
                return sheet.get_byte(remainder)
            }
        }
        panic!("should never reach here")
    }
}
