use std::fs::File;
use std::io::prelude::*;

pub fn encode(mut number: u32) -> Vec<u8> {
    let mut bytes : Vec<u8> = Vec::new(); 
    loop {
        prepend(&mut vec![(number % 128) as u8], &mut bytes);
        if number < 128 {
            break;
        }
        number = number/128;
    }
    *bytes.last_mut().unwrap() += 128;
    return bytes;
}

pub fn prepend(stuff_to_prepend: &mut Vec<u8>, prepend: &mut Vec<u8>) {
    stuff_to_prepend.append(prepend);
    prepend.append(stuff_to_prepend);
}

pub fn decode(mut file: &File) -> Option<(u32, u32)> {
    let mut number : u32 = 0;
    let mut file_buf = [0; 5]; // At most 5 bytes.
    file.read_exact(&mut file_buf);
    let file_buf_iter = file_buf.into_iter();
    let mut counter = 1; // We read at least one byte
    for byte in file_buf_iter {
        if *byte < 128 {
            number = 128 * number + *byte as u32;
        }
        else {
            number = 128 * number + (*byte as u32 - 128);
            return Some((number, counter));
        }
        counter += 1;
    }
    None
}

// Example Usage:
// println!("Variable Byte Encoded 24: {:?}", variable_byte::vb_encode(1337));
// let mut test_buf = &variable_byte::encode(1337)[..];
// println!("VBE Translated: {:?}", test_buf.read_uint::<BigEndian>(test_buf.len())); 
