pub struct Packet64 {
    data: u64
}

impl Packet64 {
    pub fn new(self, data: u64) -> Packet64 {
        Packet64{data:data}
    }

    pub fn access(self) -> u64 {
        self.data
    }
}
