use std::fmt::Write;

use rand::prelude::*;

use crate::traits::GetData;

#[derive(Debug, Clone, Copy)]
pub struct DataSheet {
    pub size_bytes:usize,
    pub max_size_bytes:usize,
    pub data: [u8;1024],
}

impl DataSheet {
    pub fn new() -> Self {
        Self{
            size_bytes: 0,
            max_size_bytes:1024,
            data: [0;1024]
        }
    }

    pub fn random() -> Self {
        let mut data = [0u8;1024];
        rand::thread_rng().fill(&mut data[..]);
        Self{
            size_bytes: 0,
            max_size_bytes:1024,
            data: data,
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for byte in self.data {
            write!(&mut s, "{:#04X} ", byte).expect("Unable to write");
        }
        s
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
