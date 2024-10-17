
pub trait GetData {
    fn get_size_bytes(&self) -> usize;
    fn get_max_size_bytes(&self) -> usize;
    fn get_byte(&self, index:usize) -> u8;
    fn get_bytes(&self, index:usize, n:usize) -> Vec<u8> {
        if index + n > self.get_size_bytes() {
            panic!("request for bytes goes out of bounds")
        }
        let mut out = Vec::<u8>::new();
        for i in 0..n {
            out.push(self.get_byte(index+i));
        }
        out
    }
}
