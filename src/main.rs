use otp_exchange::datasheet::DataSheet;
use otp_exchange::databook::DataBook;

pub fn main(){
    let data = DataSheet::new_zeros();
    data.to_file("bin/tempfile.binary");

    let data_2 = DataSheet::from_file("bin/tempfile.binary");

    println!("{}", data_2.to_string());

    let dbook = DataBook::new_random("bin/temp_databook");

}
