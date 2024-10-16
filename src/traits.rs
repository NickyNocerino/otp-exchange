
pub trait GetData {
    fn get_size(&self) -> usize {
        self.size_bytes
    }

    fn get_max_size(&self) -> usize {
        self.max_size_bytes
    }

    fn get_1B(&self) -> u8 
}
