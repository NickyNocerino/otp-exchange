use std::fmt::Write;
use std::fs;

use rand::prelude::*;

use crate::traits::GetData;

const MAX_BYTES:usize = 1024;
const MAX_SHEETS:usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct DataSheet {
    pub size_bytes:usize,
    pub max_size_bytes:usize,
    pub data: [u8;MAX_BYTES],
}

impl DataSheet {
    pub fn new() -> Self {
        Self{
            size_bytes: 0,
            max_size_bytes:MAX_BYTES,
            data: [0;MAX_BYTES]
        }
    }

    pub fn new_zeros() -> Self {
        let mut data = [0u8;MAX_BYTES];
        Self{
            size_bytes: MAX_BYTES,
            max_size_bytes:MAX_BYTES,
            data: data,
        }
    }

    pub fn new_random() -> Self {
        let mut data = [0u8;MAX_BYTES];
        rand::thread_rng().fill(&mut data[..]);
        Self{
            size_bytes: MAX_BYTES,
            max_size_bytes:MAX_BYTES,
            data: data,
        }
    }

    pub fn from_file(filepath:&str) -> Self {
        let file_data = fs::read(filepath).expect("unable to read file");
        let mut data = [0u8;MAX_BYTES];
        for i in 0..MAX_BYTES {
            if i <= file_data.len(){
                data[i] = file_data[i];
            }
            else {
                data[i] = 0;
            }
        }

        Self{
            size_bytes: 0,
            max_size_bytes:MAX_BYTES,
            data: data,
        }
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for byte in self.data {
            write!(&mut out, "{:#04X} ", byte).expect("Unable to write");
        }
        out
    }

    pub fn to_file(&self, filepath:&str) {
        fs::write(filepath, self.data).expect("cannot write to file")

    }
}

impl GetData for DataSheet {
    fn get_size(&self) -> usize {
        self.size_bytes
    }
    fn get_max_size(&self) -> usize {
        self.max_size_bytes
    }
    fn get_1b(&self, index:usize) -> u8 {
        if index > self.size_bytes {
            panic!("index access out of bounds")
        }
        self.data[index]
    }
}
