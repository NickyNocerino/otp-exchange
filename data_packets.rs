use std::ops::{BitXor};
use std::fmt::Write;

pub struct Packet64 {
    data: u64,
}

pub struct Packet256 {
    d0: Packet64,
    d1: Packet64,
    d2: Packet64,
    d3: Packet64,
    occupancy: i32,
}

pub struct Packet1B {
    data:u8
}

pub struct Packet1KB {
    data:[Packet1B;1024],
    pub  occupancy: usize,

}

pub struct Packet1MB {
    data:[Packet1KB;1024],
    occupancy: i32,

}


impl BitXor for Packet64 {
    type Output = Self;
    fn bitxor(self, other:Self) -> Self {
        Self{data:self.access()^other.access()}
    }
}

impl Packet64 {
    pub fn new(data: u64) -> Packet64 {
        Packet64{data:data}
    }

    pub fn access(self) -> u64 {
        self.data
    }
}

impl BitXor for Packet1B {
    type Output = Self;
    fn bitxor(self, other:Self) -> Self {
        Self{data:self.access()^other.access()}
    }
}

impl Packet1B {
    pub fn new(data: u8) -> Packet1B {
        Packet1B{data:data}
    }

    pub fn access(self) -> u8 {
        self.data
    }
}

impl BitXor for Packet256 {
    type Output = Self;
    fn bitxor(self, other:Self) -> Self {
        Self{d0:self.d0^other.d0, d1:self.d1^other.d1, d2:self.d2^other.d2, d3:self.d3^other.d3, occupancy:4}
    }
}

impl Packet256 {
    pub fn new() -> Packet256 {
        Packet256{d0:Packet64::new(0),d1:Packet64::new(0),d2:Packet64::new(0),d3:Packet64::new(0),occupancy:0}
    }
    pub fn add(mut self, new:Packet64) {
        match self.occupancy {
            0 => {
                self.d0 = new;
                self.occupancy = 1;
            }
            1 => {
                self.d1 = new;
                self.occupancy = 2;
            }
            2 => {
                self.d2 = new;
                self.occupancy = 3;
            }
            3 =>{
                self.d3 = new;
                self.occupancy = 4;
            }
            4 => {
                println!("Danger, attempting to add to a full Packet256");
            }
            y => {
                println!("UHOH, illegal occupancy value {}", y);
            }
        }
    }
    pub fn pop(mut self) -> Packet64 {
        match self.occupancy {
            0 => {
                println!("Danger, attempting to pop an empty Packet256");
                Packet64::new(0)
            }
            1 => {
                self.occupancy = 0;
                self.d0
            }
            2 => {
                self.occupancy = 1;
                self.d1
            }
            3 =>{
                self.occupancy = 2;
                self.d2
            }
            4 => {
                self.occupancy = 3;
                self.d3
            }
            y => {
                println!("UHOH, illegal occupancy value {}", y);
                Packet64::new(0)
            }
        }
    }
}

impl BitXor for Packet1KB {
    type Output = Self;
    fn bitxor(self, other:Self) -> Self {
        if self.occupancy != other.occupancy {
            println!("WARNING, trying to xor packets with different fill levels");
        }
        let out:[Packet1B;1024];
        for i in 0..1024 {
            out[i] = self.data[i]^other.data[i];
        }
        Self{data:out, occupancy:self.occupancy}
    }
}

impl Packet1KB {
    pub fn new() -> Packet1KB {
        let data: [Packet1B;1024];
        for i in 0..1024 {
            data [i] = Packet1B::new(0);
        }
        Packet1KB{data:data, occupancy:0}
    }
    // 
    // pub fn add(mut self, data:Packet1B) {
    //     match self.occupancy {
    //         0..=1023 => {
    //             self.data[self.occupancy] = data;
    //             self.occupancy += 1;
    //         }
    //         1024.. => {
    //             println!("WARNING, adding Packet1B to full Packet1KB");
    //         }
    //     }
    // }
    // pub fn pop(mut self) -> Packet1B {
    //     match self.occupancy {
    //         0 => {
    //             println!("WARNING, popping from Packet1KB with occupancy 0");
    //             self.data[0]
    //         }
    //         1..=1023 => {
    //             self.occupancy -= 1;
    //             self.data[self.occupancy]
    //         }
    //         1024.. => {
    //             println!("WARNING, Packet1KB has occupancy over 1024");
    //             self.data[0]
    //         }
    //     }
    // }
    pub fn to_hex(self) -> String {
        let mut hex_string = String::new();
        let mut temp = String::new();
        for i in 0..self.occupancy {
            write!(temp, "{:02x}", self.data[i].access());
            hex_string.push_str(&temp);
        }
        hex_string

    }
}
