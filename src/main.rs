use otp_exchange::datasheet::DataSheet;
use otp_exchange::databook::DataBook;

pub fn main(){
    let data = DataSheet::new_zeros();
    data.to_file("bin/tempfile.binary");

    let data_2 = DataSheet::from_file("bin/tempfile.binary");

    println!("{}", data_2.to_string());

    let dbook = DataBook::new_random("bin/temp_databook");

    dbook.to_zip("bin/temp_databook.zip");

    let dbook2 = DataBook::from_zip("bin/temp_databook.zip", "bin/temp_databook_2");
    println!("{}", dbook.size);
    println!("{}", dbook.location);
    println!("{}", dbook2.size);
    println!("{}", dbook2.location);

}
