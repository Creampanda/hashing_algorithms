use std::io::stdin;

mod modulus_hash;
mod md5_hash;

fn main() {
    let mut user_input = String::new();
    println!("Введите ваш текст:");
    stdin()
        .read_line(&mut user_input)
        .expect("Вы ввели некорректный текст");
    let user_input = user_input.trim();
    modulus_hash::module_hash(user_input);
    md5_hash::md5_hash(user_input);
}
