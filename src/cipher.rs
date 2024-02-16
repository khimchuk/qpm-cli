pub fn encryption(password: String) -> String {
    let mut result: String = String::new();

    for byte in password.chars() {
        let binary_repr = format!("{:08b}", byte as u8);
        result.push_str(&binary_repr);
    }

    return result;
}


pub fn decryption(to_encrypt: String) -> String {
    let mut result: String = String::new();

    for chunk in to_encrypt.chars().collect::<Vec<_>>().chunks(8) {
        let byte: String = chunk.iter().collect();
        let decrypted_byte: u8 = u8::from_str_radix(&byte, 2).unwrap();
        result.push(decrypted_byte as char);
    }

    return result;
}
