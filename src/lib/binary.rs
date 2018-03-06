pub mod binary {

    extern crate rand;
    use self::rand::Rng;

    /// Sets to 0 the least significant bit of an byte (input parameter)
    fn empty_least_significant_bit_for_byte(byte: &mut u8) {
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
    #[ignore(dead_code)]
    pub fn store_random_bit_in_u8(number: &mut u8) {
        let random_bit_value = rand::thread_rng().gen_range(0, 2);

        if random_bit_value > 0 {
            store_bit_in_u8(number, true);
        }
    }

    /// Converts byte (input) into a boolean (~= binary) vector
    fn convert_u8_to_bit_vec(byte: u8) -> Vec<bool> {
        // Convert input byte to binary string
        let mut byte_in_binary: String = format!("{:08b}", byte);

        // Initializing output vector
        let mut result_binary_array = Vec::<bool>::new();

        // Convert zerones and ones to booleans and push them into the output vector
        while let Some(c) = byte_in_binary.pop() {
            match c {
                '1' => result_binary_array.push(true),
                _   => result_binary_array.push(false)
            }
        }

        // Return output vector
        result_binary_array
    }

    /// Convert binary byte looking input string to byte
    fn bit_string_to_u8_char(slice: &String) -> u8 {
        // Slice the string into chars
        // Map '0' and '1' chars to 0s and 1s
        // Fold the whole "byte-string" into a number
        // And return it 
        slice.chars().map(|x| match x { '1' => 1, _ => 0}).rev().fold(0, |acc, b| acc * 2 + b as u8)
    }

    /// Convert boolean (~= binary) vector into String
    pub fn convert_bit_vec_to_message(bit_vec: &Vec<bool>) -> String {
        let mut byte_as_string;          // Byte to string conversion variable
        let mut read_char: char;          // Parsed char
        let mut result = String::new();  // Output String
        let mut read_byte_slice: &[bool]; // input vector parser
        let mut last_index = 0;          // Parsing head position

        // Parse boolean (~= binary) vector until end
        while last_index < bit_vec.len() {

            // Init or reinit read byte
            byte_as_string = "".to_string();

            // Parse input vector byte by byte
            read_byte_slice = &bit_vec[last_index..last_index+7];

            // convert boolean values to '0' and '1' chars
            for i in 0..7 {
                match read_byte_slice[i] {
                    true    => byte_as_string.push('1'),
                    _       => byte_as_string.push('0')
                }
            }

            // Convert "byte-string" to a ASCII char
            read_char = format!("{}", bit_string_to_u8_char(&byte_as_string) as char).chars().next().unwrap();
            
            // Add read char to output string
            result.push(read_char);

            // Prepare next byte for read
            last_index += 8;
        }
        result
    }

    /// Convert String into boolean (~= binary) vector
    pub fn convert_message_to_bit_vec(message: String) -> Vec<bool> {
        // Convert message into byte array
        let message_as_u8_array = message.as_bytes();

        // Initialize output boolean vector
        let mut message_as_binary_vector = Vec::<bool>::new();

        // Parse message byte by byte until end
        for i in 0..message_as_u8_array.len() {
            // Convert byte into byte bit vector
            let mut bitvec = convert_u8_to_bit_vec(message_as_u8_array[i]);

            // Append byte bit vector to main output vit vector
            message_as_binary_vector.append(&mut bitvec);
        }

        // And return it
        message_as_binary_vector
    }
}