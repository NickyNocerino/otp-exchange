use otp_exchange::datasheet::DataSheet;
use otp_exchange::databook::DataBook;

#[test]
fn test_initialize(){
    let data = DataSheet::new();
    let rand_data = DataSheet::new_random();
    println!("data:  {}", data.to_string());
    println!("rand_data: {}", rand_data.to_string());
}
