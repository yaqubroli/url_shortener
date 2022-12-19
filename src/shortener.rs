// Dangling library of functions to create as short of a shortened string as humanly possible.

const BASE64_CHARS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '-', '_',
];

pub fn base64(num: u64) -> String {
    let mut result = String::new();
    let mut num = num;
    while num > 0 {
        let remainder = num % 64;
        num = num / 64;
        result.push(BASE64_CHARS[remainder as usize]);
    }
    result
}

// creates a string of random base64 characters, initial length is 4, but if the total amount of strings exceeds 64^4, it will increase the length of the string by 1
/*
pub fn random_base64(num: u64) -> String {
    let mut result = String::new();
    let mut num = num;
    let mut length = 4;
    while num > 0 {
        let remainder = num % 64;
        num = num / 64;
        result.push(BASE64_CHARS[remainder as usize]);
        if num > 0 {
            length += 1;
        }
    }
    while result.len() < length {
        result.push(BASE64_CHARS[rand::random::<usize>() % 64]);
    }
    result
} */