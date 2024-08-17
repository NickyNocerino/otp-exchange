use std::ops::{BitXor};

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
