extern crate base64;
extern crate hex;

use std::u8;

pub fn hex_to_base64(hex: String) -> String {

    let mut bytes = Vec::new();
    for i in 0..(hex.len()/2) {
        let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    };

    base64::encode(&bytes)
}

pub fn fixed_xor(str_a: String, str_b: String) -> String {

    let buffer_a = match hex::decode(str_a) {
        Ok(bytes) => bytes,
        Err(_) => panic!("Could not decode"),
    };

    let buffer_b = match hex::decode(str_b) {
        Ok(bytes) => bytes,
        Err(_) => panic!("Could not decode"),
    };

    let key_iter = buffer_b.iter().cycle();
    let res : Vec<u8> = buffer_a.iter().zip(key_iter).map(|(&a, b)| a ^ b ).collect();
    hex::encode(res)
}
