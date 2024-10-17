use otp_exchange::data::DataSheet;

#[test]
fn test_initialize(){
    let data = DataSheet::new();
    let rand_data = DataSheet::random();
    println!("data:  {}", data.to_string());
    println!("rand_data: {}", rand_data.to_string());
}
