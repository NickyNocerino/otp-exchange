use otp_exchange::data::DataSheet;

pub fn main(){
    let data = DataSheet::random();
    data.to_file("bin/tempfile.binary");

    let data_2 = DataSheet::from_file("bin/tempfile.binary");

    println!("{}", data_2.to_string())

}
