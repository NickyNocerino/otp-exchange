use std::time::{Duration, Instant};


#[test]
fn test_read_speed(){
    let start = Instant::now();
    
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}