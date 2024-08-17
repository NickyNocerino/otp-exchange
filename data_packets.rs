

pub struct Packet64 {
    data: u64,
}

pub struct Packet256 {
    data0: Packet64,
    data1: Packet64,
    data2: Packet64,
    data3: Packet64,
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
    pub fn add(self, new:Packet64) -> Nil {
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
            4 => Nil
        }
    }
    pub fn pop(self) -> Nil {
        match self.occupancy {
            0 => {
                Nil
            }
            1 => {
                self.occupancy = 0;
                self.d0
            }
            2 => {
                self.occupancy = 1;
                self.d1;
            }
            3 =>{
                self.occupancy = 2;
                self.d2
            }
            4 => {
                self.occupancy = 3
                self.d3
            }
        }
    }
}
