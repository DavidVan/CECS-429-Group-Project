pub fn vb_encode(number:u32) -> Vec<u8> {
    let mut bytes: vec<u8> = Vec::new(); 
    if number == 0 {
        return Vec![0b0];
    }
    
    loop {

    }
    return bytes;
}

pub fn prepend(current: vec<u8>,to_prepend :vec<u8>) -> vec<u8> {
    return to_prepend.append(current);
}