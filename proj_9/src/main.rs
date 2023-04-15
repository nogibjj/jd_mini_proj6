// generate random password with input length and flags for numbers and special characters

use rand::Rng;
use std::io;

fn create_password(length:i32, number:Bool, special:Bool) -> String {
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    let mut i = 0;
    while i < length {
        let mut random_char = rng.gen_range(33, 126);
        if number == true {
            random_char = rng.gen_range(33, 126);
        }
        if special == true {
            random_char = rng.gen_range(33, 126);
        }
        password.push(random_char as char);
        i += 1;
    }
    return password;
}


fn main() {
    password = create_password(10, true, true);
    println!("Your new password is: {}", password)
}
