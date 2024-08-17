use data_packets;

use data_packets::Packet64;

pub struct OTP64 {
    data:Packet64,
}

impl OTP64 {
    pub fn new(data:u64) -> OTP64 {
        OTP64{data:Packet64::new(data)}
    }

    pub fn send(self, message:Packet64) -> Packet64 {
        Packet64::new(message.access()^self.data.access())
    }

    pub fn receive(self, message:Packet64) -> Packet64 {
        Packet64::new(message.access()^self.data.access())
    }
}
