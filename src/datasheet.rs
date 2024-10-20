use std::fmt::Write;
use std::fs;
use std::cmp;
use std::ops::BitXor;

use rand::prelude::*;

use crate::otp::OneTimePad;
use crate::traits::GetData;

const MAX_BYTES:usize = OneTimePad::MAX_BYTES;
//const MAX_SHEETS:usize = OneTimePad::MAX_SHEETS;

#[derive(Debug, Clone, Copy)]
pub struct DataSheet {
    pub size_bytes:usize,
    pub max_size_bytes:usize,
    pub data: [u8;MAX_BYTES],
}

impl BitXor for DataSheet {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        let size = cmp::min(self.size_bytes, other.size_bytes);
        let mut data = [0u8;MAX_BYTES];
        for i in 0..MAX_BYTES {
            if i < size {
                data[i] = self.data[i]^other.data[i];
            }
        }
        Self{
            size_bytes: size,
            max_size_bytes:MAX_BYTES,
            data:data,
        }
    }
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
        let data = [0u8;MAX_BYTES];
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
        let mut size = 0;
        for i in 0..MAX_BYTES {
            if i < file_data.len(){
                data[i] = file_data[i];
                size += 1;
            }
        }

        Self{
            size_bytes: size,
            max_size_bytes:MAX_BYTES,
            data: data,
        }
    }

    pub fn from_vec(in_data:&Vec<u8>) -> Self{
        let mut data = [0u8;MAX_BYTES];
        for i in 0..MAX_BYTES {
            if i < in_data.len(){
                data[i] = in_data[i];
            }
            else {
                data[i] = 0;
            }
        }

        Self{
            size_bytes: cmp::min(MAX_BYTES, in_data.len()),
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
    fn get_size_bytes(&self) -> usize {
        self.size_bytes
    }
    fn get_max_size_bytes(&self) -> usize {
        self.max_size_bytes
    }
    fn get_byte(&self, index:usize) -> u8 {
        if index > self.get_size_bytes() {
            panic!("index access out of bounds")
        }
        self.data[index]
    }
}
