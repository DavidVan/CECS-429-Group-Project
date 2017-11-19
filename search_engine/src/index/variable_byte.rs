pub fn vb_encode(mut number: u32) -> Vec<u8> {
    let mut bytes : Vec<u8> = Vec::new(); 
    if number == 0 {
        return vec![0b0];
    }
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

// Example Usage:
// println!("Variable Byte Encoded 24: {:?}", variable_byte::vb_encode(1337));
// let mut test_buf = &variable_byte::vb_encode(1337)[..];
// println!("VBE Translated: {:?}", test_buf.read_uint::<BigEndian>(test_buf.len())); 
