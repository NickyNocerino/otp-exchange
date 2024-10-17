
pub trait GetData {
    fn get_size(&self) -> usize;
    fn get_max_size(&self) -> usize;
    fn get_1b(&self, index:usize) -> u8;
}
