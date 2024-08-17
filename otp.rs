use data_packets;

use data_packets::Packet64;
use data_packets::Packet256;

pub struct OTP64 {
    data:Packet64,
}

pub struct OTP256 {
    data: Packet256,
    occupancy: i32,
}

impl OTP64 {
    pub fn new(data:u64) -> OTP64 {
        OTP64{data:Packet64::new(data)}
    }

    pub fn send(self, message:Packet64) -> Packet64 {
        self.data^message
    }

    pub fn receive(self, message:Packet64) -> Packet64 {
        self.data^message
    }
}
