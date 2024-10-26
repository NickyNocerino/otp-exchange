use otp_exchange::databook::DataBook;
use otp_exchange::otp::OneTimePad;
use otp_exchange::traits::GetData;

pub fn main(){

    DataBook::new_random("bin/temp_databook").to_zip("bin/temp_databook.zip");

    let mut one_time_pad_alice = OneTimePad::load_zip("bin/temp_databook.zip", "bin/temp_otp_alice");
    let mut one_time_pad_bob = OneTimePad::load_zip("bin/temp_databook.zip", "bin/temp_otp_bob");

    // let message ="HELLO, perfect encryption";
    //
    // println!("message: {}", message);
    //
    // let mut encrypted = one_time_pad_alice.encrypt(message.to_owned().into_bytes());
    //
    // println!("encrypted message: {}", String::from_utf8_lossy(&encrypted));
    //
    // //decrypt and encrypt do the same thing, its just to help show off whats happening
    // let mut decrypted = one_time_pad_bob.decrypt(encrypted);
    //
    // println!("decrypted message: {}", String::from_utf8_lossy(&decrypted));
    //
    // let message_2 = "we can do this in both directions so long as we stay in phase";
    //
    // println!("message 2: {}", message_2);
    //
    // encrypted = one_time_pad_bob.encrypt(message_2.to_owned().into_bytes());
    //
    // println!("encrypted message: {}", String::from_utf8_lossy(&encrypted));
    //
    // decrypted = one_time_pad_alice.decrypt(encrypted);
    //
    // println!("decrypted message: {}", String::from_utf8_lossy(&decrypted));

    one_time_pad_bob.encrypt_file("bin/sample.txt", "bin/encrypted_sample.bin");
    one_time_pad_alice.decrypt_file("bin/encrypted_sample.bin", "bin/decrypted_sample.txt");

    println!("used {} bytes of {} total", one_time_pad_bob.consumed, one_time_pad_bob.pad.get_size_bytes())
}
