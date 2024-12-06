use std::time::{Duration, Instant};
use otp_exchange::databook::DataBook;
use otp_exchange::traits::*;

#[test]
fn test_read_speed(){
    let pad = DataBook::from_zip("bin/temp_databook.zip", "bin/temp_databook");
    let start = Instant::now();
    let data = pad.get_bytes_fast(0, pad.get_size_bytes());
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}