use std::io::Read;

fn byte_to_hex(byte: u8) -> String {
    let left_bits = (byte & 0b11110000) >> 4;
    let right_bits = byte & 0b00001111;

    let left_bits_char = {
        if left_bits == 0 {
            '0'
        } else if left_bits == 1 {
            '1'
        } else if left_bits == 2 {
            '2'
        } else if left_bits == 3 {
            '3'
        } else if left_bits == 4 {
            '4'
        } else if left_bits == 5 {
            '5'
        } else if left_bits == 6 {
            '6'
        } else if left_bits == 7 {
            '7'
        } else if left_bits == 8 {
            '8'
        } else if left_bits == 9 {
            '9'
        } else if left_bits == 10 {
            'A'
        } else if left_bits == 11 {
            'B'
        } else if left_bits == 12 {
            'C'
        } else if left_bits == 13 {
            'D'
        } else if left_bits == 14 {
            'E'
        } else if left_bits == 15 {
            'F'
        } else {
            ' '
        }
    };

    let right_bits_char = {
        if right_bits == 0 {
            '0'
        } else if right_bits == 1 {
            '1'
        } else if right_bits == 2 {
            '2'
        } else if right_bits == 3 {
            '3'
        } else if right_bits == 4 {
            '4'
        } else if right_bits == 5 {
            '5'
        } else if right_bits == 6 {
            '6'
        } else if right_bits == 7 {
            '7'
        } else if right_bits == 8 {
            '8'
        } else if right_bits == 9 {
            '9'
        } else if right_bits == 10 {
            'A'
        } else if right_bits == 11 {
            'B'
        } else if right_bits == 12 {
            'C'
        } else if right_bits == 13 {
            'D'
        } else if right_bits == 14 {
            'E'
        } else if right_bits == 15 {
            'F'
        } else {
            ' '
        }
    };

    let mut as_text = String::new();

    as_text.push(left_bits_char);
    as_text.push(right_bits_char);

    return as_text;
}

fn main() {
    let input = "ckczppom";
    let mut answer = String::new();

    for i in 0..100000000 {
        let as_text = &mut String::from(i.to_string());
        let compound_text = input.to_owned() + as_text;
        let computed_hash = md5::compute(compound_text);
        let computed_hash_bytes = computed_hash.bytes();
        let mut hash_as_string = String::new();
        for byte in computed_hash_bytes {
            match byte {
                Ok(byte) => {
                    hash_as_string.push_str(&byte_to_hex(byte))
                },
                Err(err) => println!("{}", err)
            }
        }
        let first_six_chars = &hash_as_string[0..6];
        println!("{hash_as_string} - {}", input.to_owned() + as_text);
        if first_six_chars.matches("000000").count() > 0 {
            answer = as_text.to_string();
            break
        }
    }

    println!("{}", answer);
}