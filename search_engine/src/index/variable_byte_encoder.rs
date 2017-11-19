pub fn vb_encode(number:u32) -> Vec<u8> {
    let mut bytes: vec<u8> = Vec::new(); 
    if number == 0 {
        return Vec![0b0];
    }
    
    loop {
        bytes = prepend(bytes,vec![(n % 128) as u8]);
        if n < 128 {
            break;
        }
        n = n/128;
    }
    bytes.last() = bytes.last() + 128;
    return bytes;
}

pub fn prepend(current: vec<u8>,to_prepend :vec<u8>) -> vec<u8> {
    return to_prepend.append(current);
}