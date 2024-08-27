use std::ops::{BitXor};
use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
pub struct Packet1KB {
    pub head: usize,
    pub tail: usize,
    pub data: [u8;1024],
}

impl Packet1KB {
    pub fn new() -> Packet1KB {
        Packet1KB{ head:0, tail:0, data:[0;1024]}
    }

    pub fn to_hex(self) -> String {
        let mut hex_string = String::new();
        let mut temp = String::new();
        for i in 0..1024 {
            temp = String::new();
            write!(temp, "{:02x}", self.data[i]);
            hex_string.push_str(&temp);
        }
        hex_string
    }
    pub fn add(mut self, i:usize, byte:u8) {
        self.data[i] = byte;
    }
}

impl BitXor for Packet1KB {
    type Output = Self;
    fn bitxor(self, other:Self) -> Self {
        let mut out_data:[u8;1024] = [0;1024];
        if self.head == other.head && self.tail == other.tail {
            for i in 0..1024 {
                out_data[i] = self.data[i]^other.data[i];
            }
            Self{head:self.head, tail:self.tail, data:out_data}

        }
        else {
            println!("WARNING, packet head and tail do not match, Undefined behavior");
            for i in 0..1024 {
                out_data[i] = self.data[i]^other.data[i];
            }
            Self{head:self.head, tail:self.tail, data:out_data}
        }
    }
}
