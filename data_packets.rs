

pub struct Packet64 {
    data: u64,
}

pub struct Packet256 {
    data0: u64,
    data1: u64,
    data2: u64,
    data3: u64,
    occupancy: i32
}

impl Packet64 {
    pub fn new(data: u64) -> Packet64 {
        Packet64{data:data}
    }

    pub fn access(self) -> u64 {
        self.data
    }
}

impl Packet256 {
    pub fn new() -> Packet256 {
        Packet256{0,0,0,0,0}
    }
}
