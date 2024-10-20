use crate::databook::DataBook;
use crate::traits::*;

use pyo3::prelude::*;

#[pyclass]
pub struct OneTimePad {
    pub pad:DataBook,
    pub consumed: usize,
}

#[pymethods]
impl OneTimePad {
    pub const MAX_SHEETS: usize = 32;
    pub const MAX_BYTES: usize = 1024*1024;

    #[new]
    pub fn load_zip(target_zip:&str, target_dir:&str) -> Self {
        Self {
            pad: DataBook::from_zip(target_zip, target_dir),
            consumed:0,
        }
    }
    pub fn encrypt(& mut self, data:Vec<u8>) -> Vec<u8> {
        let size = data.len();
        if self.consumed + size > self.pad.get_size_bytes() {
            panic!("Attempting to encrypt beyond the size of pad")
        }
        let pad_data = self.pad.get_bytes(self.consumed, size);
        self.consumed += size;
        let mut out = Vec::<u8>::new();
        for i in 0..size {
            out.push(pad_data[i]^data[i]);
        }
        out
    }

    pub fn decrypt(& mut self, data:Vec<u8>) -> Vec<u8> {
        // For 1 time pads, encrypting and decryyping is the same, neat huh!
        self.encrypt(data)
    }

}
