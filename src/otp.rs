use std::fs;

use crate::databook::DataBook;
use crate::traits::*;


pub struct OneTimePad {
    pub pad:DataBook,
    pub consumed: usize,
}

impl OneTimePad {
    pub const MAX_SHEETS: usize = 10 * 1024;
    pub const MAX_BYTES: usize = 1024*1024;

    pub fn load_zip(target_zip:&str, target_dir:&str) -> Self {
        Self {
            pad: DataBook::from_zip(target_zip, target_dir),
            consumed:0,
        }
    }

    pub fn remaining(&self) -> usize {
        self.pad.get_size_bytes() - self.consumed
    }

    pub fn encrypt(& mut self, data:Vec<u8>) -> Vec<u8> {
        let size = data.len();
        if self.consumed + size > self.pad.get_size_bytes() {
            panic!("Attempting to encrypt beyond the size of pad")
        }
        let pad_data = self.pad.get_bytes_fast(self.consumed, size);
        self.consumed += size;
        let mut out = Vec::<u8>::new();
        for i in 0..size {
            out.push(pad_data[i]^data[i]);
        }
        out
    }

    pub fn encrypt_file(&mut self, target_source_file:&str, target_dest_file:&str) {
        let file_data:Vec<u8> = fs::read(target_source_file).expect("unable to read file");
        let size = file_data.len();
        if self.consumed + size > self.pad.get_size_bytes() {
            panic!("Attempting to encrypt beyond the size of pad")
        }
        let pad_data = self.pad.get_bytes_fast(self.consumed, size);
        self.consumed += size;
        let mut out = Vec::<u8>::new();
        for i in 0..size {
            out.push(pad_data[i]^file_data[i]);
        }
        fs::write(target_dest_file, out).expect("cannot write to file");

    }

    pub fn decrypt(& mut self, data:Vec<u8>) -> Vec<u8> {
        // For 1 time pads, encrypting and decrypting is the same, neat huh!
        self.encrypt(data)
    }

    pub fn decrypt_file(&mut self, target_source_file:&str, target_dest_file:&str) {
        // For 1 time pads, encrypting and decrypting is the same, neat huh!
        self.encrypt_file(target_source_file, target_dest_file);
    }

}
