pub mod binary {

    extern crate rand;
    use self::rand::Rng;

    /// Sets to 0 the least significant bit of an byte (input parameter)
    pub fn empty_least_significant_bit_for_byte(byte: &mut u8) {
        match *byte % 2 {
            1       => *byte -= 1,    // Minus one to set it empty
            0 | _   => {}               // Already empty ;-)
        }
    }

    /// Store bit in byte (Input parameters)
    /// empties least significant one and sets 0 or 1 into it
    pub fn store_bit_in_u8(byte: &mut u8, bit: bool) {
        empty_least_significant_bit_for_byte(byte);

        match bit {
            true    => *byte += 1,
            false   => {}
        }
    }

    /// Store random bit in byte (Input parameter)
    pub fn store_random_bit_in_u8(number: &mut u8) {
        empty_least_significant_bit_for_byte(number);

        let random_bit_value = rand::thread_rng().gen_range(0, 2);

        if random_bit_value > 0 {
            store_bit_in_u8(number, true);
        }
    }

    /// Converts byte (input) into a boolean (~= binary) vector
    pub fn convert_u8_to_bit_vec(byte: u8) -> Vec<bool> {
        // Convert input byte to binary string
        let byte_in_binary: String = format!("{:08b}", byte);

        // Initializing output vector
        let mut result_binary_array = Vec::<bool>::new();

        // Convert zeroes and ones to booleans and push them into the output vector
        for (_, c) in byte_in_binary.chars().enumerate() {
            match c {
                '1' => result_binary_array.push(true),
                _   => result_binary_array.push(false)
            }
        }

        // Return output vector
        result_binary_array
    }

    /// Convert binary byte looking input string to byte
    pub fn convert_bit_string_to_u8_char(bit_string: String) -> u8 {
        // Slice the string into chars
        // Map '0' and '1' chars to 0s and 1s
        // Fold the whole "byte-string" into a number
        // And return it 
        bit_string.chars().map(|x| match x { '1' => 1, _ => 0 }).fold(0, |acc, b| acc * 2 + b as u8)
    }

    /// Convert u8 vector into boolean (~= binary) vector
    pub fn convert_u8_vec_to_bit_vec(vector: &Vec<u8>) -> Vec<bool> {
        // Initialize output boolean vector
        let mut message_as_binary_vector = Vec::<bool>::new();

        // Parse message byte by byte until end
        for i in 0..vector.len() {
            // Convert byte into byte bit vector
            let mut bitvec = convert_u8_to_bit_vec(vector[i]);

            // Append byte bit vector to main output vit vector
            message_as_binary_vector.append(&mut bitvec);
        }

        // And return it
        message_as_binary_vector
    }

    /// Convert bit vector into u8 vector
    pub fn convert_bit_vec_to_u8(bit_vec: &Vec<bool>) -> u8 {
        let mut byte_as_string; // Byte to string conversion variable

        // Init or reinit read byte
        byte_as_string = "".to_string();

        let read_byte_slice = &bit_vec[0..8];

        // convert boolean values to '0' and '1' chars
        for i in 0..8 {
            match read_byte_slice[i] {
                true    => byte_as_string.push('1'),
                _       => byte_as_string.push('0')
            }
        }

        // Convert "byte-string" to a ASCII char
        convert_bit_string_to_u8_char(byte_as_string)
    }
}

// Tests
#[cfg(test)]
pub mod tests {
    use std::collections::BTreeSet;
    use super::binary;

    #[test]
    fn test_empty_least_significant_bit_for_byte() {
        let mut byte = 31;

        binary::empty_least_significant_bit_for_byte(&mut byte);
        assert_eq!(byte, 30);
        binary::empty_least_significant_bit_for_byte(&mut byte);
        assert_eq!(byte, 30);
    }

    #[test]
    fn test_store_bit_in_u8() {
        let mut byte = 31;

        binary::store_bit_in_u8(&mut byte, true);
        assert_eq!(byte, 31);
        binary::store_bit_in_u8(&mut byte, false);
        assert_eq!(byte, 30);
    }

    #[test]
    fn test_store_random_bit_in_u8() {
        let mut byte: u8;
        let mut cases = Vec::<u8>::new();

        for _ in 0..100 {
            byte = 31;
            binary::store_random_bit_in_u8(&mut byte);
            cases.push(byte);
        }

        let set: BTreeSet<_> = cases.drain(..).collect();
        let mut set2 = BTreeSet::new();

        let mut value30:u8 = 30;
        let mut value31:u8 = 31;

        set2.insert(value30);
        set2.insert(value31);

        assert_eq!(set, set2);
    }

    #[test]
    fn test_convert_u8_to_bit_vec() {
        let tab = [false, false, false, true, true, true, true, true];
        let vec = tab.to_vec();

        assert_eq!(binary::convert_u8_to_bit_vec(31), vec);
    }
    
    #[test]
    fn test_convert_bit_string_to_u8_char() {
        assert_eq!(binary::convert_bit_string_to_u8_char("00011111".to_string()), 31);
    }

    #[test]
    fn test_convert_u8_vec_to_bit_vec() {
        let tab = [31, 31, 31];
        let vec = &tab.to_vec();

        let tab2 = [ false, false, false, true, true ,true, true, true
                   , false, false, false, true, true ,true, true, true
                   , false, false, false, true, true ,true, true, true];
                   
        let mut vec2 = tab2.to_vec();

        assert_eq!(binary::convert_u8_vec_to_bit_vec(&vec), vec2);
    }

    #[test]
    fn test_convert_bit_vec_to_u8() {
        let tab = [false, false, false, true, true, true, true, true];

        let vec = &tab.to_vec();

        assert_eq!(binary::convert_bit_vec_to_u8(&vec), 31);
    }
}