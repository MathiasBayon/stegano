pub mod dot_matrix {

    extern crate image;
    use self::image::DynamicImage;
    use self::image::GenericImage;
    use self::image::Pixel;

    use std::fs::File;

    use lib::binary::*;
    use lib::cypher::*;

    const ENDING_CHAR: char = '#';

    /// Basic structure : a DynamicImage and a filepath
    pub struct DotMatrix {
        image:          DynamicImage,
        #[allow(dead_code)]
        input_filepath: String
    }

    impl DotMatrix {

        /// Constructor
        pub fn new(filepath: &str) -> DotMatrix {
            DotMatrix {
                input_filepath: filepath.to_string(),
                image:          image::open(filepath).unwrap()
            }
        }

        /// Accessor returning picture dimensions as a tuple pixel
        pub fn get_dimensions(&self) -> (u32, u32) {
            self.image.dimensions()
        }

        /// Accessor returning picture's filepath
        #[allow(dead_code)]
        pub fn get_input_filepath(&self) -> String {
            self.input_filepath.to_string()
        }

        /// Setter allowing the user to change the source picture
        #[allow(dead_code)]
        pub fn read_from_file(&mut self, filepath: &str) {
            self.input_filepath = filepath.to_string();
            self.image = image::open(filepath).unwrap();
        }

        /// Function to write the picture into target unexisting file
        pub fn write_to_file(&self, filepath: &str) {
            let ref mut output_file = File::create(filepath).unwrap();
            self.image.save(output_file, image::PNG).unwrap();
        }

        /// Function to store 3 bits, hidden into pixel at input coordinates
        fn store_3bits_at(&mut self, x: u32, y: u32, bits: &[bool]) {
            // Get the pixel at input coordinates
            let ref mut pixel = self.image.get_pixel(x, y);
            let mut red = pixel.to_rgb()[0];
            let mut green = pixel.to_rgb()[1];
            let mut blue = pixel.to_rgb()[2];

            // Then binary-OR it with the boolean input values
            // If the end of the encryption message is reached,
            // then the bit array may have less than 3 elements, so, store what is storable
            if bits.len() >= 1 {
                binary::store_bit_in_u8(&mut red, bits[0]);
            }
            if bits.len() >= 2 {
                binary::store_bit_in_u8(&mut green, bits[1]);
            }
            if bits.len() == 3 {
                binary::store_bit_in_u8(&mut blue, bits[2]);
            }

            // Create new pixel from altered RGB values and put it in image
            // Alpha is not altered
            self.image.put_pixel(x, y, image::Rgba([red, green, blue, pixel[3]]));
        }

        /// Returns bits stored in pixel, at given position
        fn get_3bits_at(&self, x: u32, y: u32) -> [bool;3] {
            let ref mut pixel = self.image.get_pixel(x, y);
            // Use modulo to know whether value is odd or not
            [   if pixel.to_rgb()[0] % 2 == 0 { false } else { true }
            ,   if pixel.to_rgb()[1] % 2 == 0 { false } else { true }
            ,   if pixel.to_rgb()[2] % 2 == 0 { false } else { true }
            ]
        }

        /// Append ending character to message
        /// TODO : Transform into static function
        /// TODO : Correct ugly cast
        fn get_vector_w_ending_char(&self, message: &Vec<u8>) -> Vec<u8>{
            let mut message2 = message.clone();
            message2.push(ENDING_CHAR as u8);
            message2
        }

        /// Returns true if given x is within picture boundaries
        fn x_in_dimensions(&self, x: u32) -> bool {
            x < self.get_dimensions().0
        }

        /// Returns true if given y is within picture boundaries
        fn y_in_dimensions(&self, y: u32) -> bool {
            y < self.get_dimensions().1
        }

        /// Returns true if image is big enough to store given message
        fn is_big_enough_to_store_message(&self, message_len: u32) -> bool {
            self.get_dimensions().0 * self.get_dimensions().1 > message_len
        }

        fn store_random_from(&mut self, x: u32, y: u32) {
            // Shadow x and y into local mutable variables
            let mut x = x.clone();
            let mut y = y.clone();

            // Ensure to stays within picture boundaries
            while self.y_in_dimensions(y) {
                while self.x_in_dimensions(x) {
                    let ref mut pixel = self.image.get_pixel(x, y);
                    let mut red = pixel.to_rgb()[0];
                    let mut green = pixel.to_rgb()[1];
                    let mut blue = pixel.to_rgb()[2];

                    binary::store_random_bit_in_u8(&mut red);
                    binary::store_random_bit_in_u8(&mut green);
                    binary::store_random_bit_in_u8(&mut blue);

                    // Create new pixel from altered RGB values and put it in image
                    // Alpha is not altered
                    self.image.put_pixel(x, y, image::Rgba([red, green, blue, pixel[3]]));

                    x += 1;
                }

                y += 1;
                x = 0;
            }
        }

        /// Encode given message in self image
        pub fn encode(&mut self, message: &str) -> Result<&str, &str> {
            // Add ending charadter to input message
            let encryp = cypher::simple_encrypt(message);
            let message_w_ending_character = self.get_vector_w_ending_char(&encryp);

            // Convert message to binary vetor
            let vector = binary::convert_u8_vec_to_bit_vec(&message_w_ending_character);
            
            // Get binary vector length
            let message_length = vector.len();
            
            // Check if picture is big enoug to store binary vector
            if !self.is_big_enough_to_store_message(message_length as u32) { return Err("Input file not big enough to store message"); }
            
            // Position indexes
            let mut x = 0;
            let mut y = 0;

            // Parsing cursor : as we can store 3 bits in a pixel
            // this index ensures that we progress 3 bits by 3 bits in the message converted as vector
            // which we want to encore
            let mut parsing_cursor = 0;

            // Ensure to stays within picture boundaries
            while self.y_in_dimensions(y) {
                while self.x_in_dimensions(x) {
                    // Check if message end is reached and if there is less than 3 bits to write into picture
                    if parsing_cursor+2 > message_length {
                        // If so, dump remaining vector bits into last pixel
                        self.store_3bits_at(x, y, &vector[parsing_cursor..]);

                        // Store random things to hide picture alteration from picture analysers
                        if self.x_in_dimensions(x+1) {
                            self.store_random_from(x+1, y);
                        } else if self.y_in_dimensions(y+1) {
                            self.store_random_from(0, y+1);
                        }

                        return Ok("Done"); // Hell yeah, it's finished !
                    }
                    // If not, store 3 bits in current pixel
                    self.store_3bits_at(x,y, &vector[parsing_cursor..parsing_cursor+3]);

                    x += 1;
                    parsing_cursor +=3
                }

                // Reset x coordinates to left and increase y
                y += 1;
                x = 0;
            }

            return Err("Input file not big enough to store message"); // Should not happen
        }

        /// Decodes image and return result string
        pub fn decode(&self) -> Option<String> {
            // Position indexes
            let mut x = 0;
            let mut y = 0;

            // Vector containing consolidated bits, before conversion in byte, then in char
            let mut boolean_byte_vector = Vec::<bool>::new();

            // Remains of previous bit vector, belonging to the next character
            let mut remains_of_previous_bit_vector = Vec::<bool>::new();

            // The decoded character
            let mut charac;

            // The message hidden in picture
            let mut message = Vec::<u8>::new();

            // Keep cursor within picture boundaries
            while self.y_in_dimensions(y) {
                while self.x_in_dimensions(x) {
                    
                    // TODO : Refactor <here>, ugly code spotted
                    if boolean_byte_vector.len() == 6 {
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[0]);
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[1]);
                        remains_of_previous_bit_vector.push(self.get_3bits_at(x,y)[2]);
                    } else if boolean_byte_vector.len() == 7 {
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[0]);
                        remains_of_previous_bit_vector.push(self.get_3bits_at(x,y)[1]);
                        remains_of_previous_bit_vector.push(self.get_3bits_at(x,y)[2]);
                    } else if boolean_byte_vector.len() == 8 { // Complete byte formed!
                        // Convert byte to character
                        charac = binary::convert_bit_vec_to_u8(&boolean_byte_vector);
                        
                        // Check if read character is the ending one
                        if charac == ENDING_CHAR as u8 {
                            let result = cypher::simple_decrypt(&message);
                            return Some(result); // If so, return message
                        } else { // Continue fetching pixels to retrieve the missing characters
                            message.push(charac);

                            boolean_byte_vector = Vec::<bool>::new();
                            boolean_byte_vector.append(&mut remains_of_previous_bit_vector);

                            remains_of_previous_bit_vector = Vec::<bool>::new();

                            boolean_byte_vector.push(self.get_3bits_at(x,y)[0]);
                            boolean_byte_vector.push(self.get_3bits_at(x,y)[1]);
                            boolean_byte_vector.push(self.get_3bits_at(x,y)[2]);
                        }
                    } else {
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[0]);
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[1]);
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[2]);
                    }
                    
                    x += 1;
                }

                // Reset x coordinates to left and increase y
                y += 1;
                x = 0;
            }

        return None; // Ending character never reached, return None
        }
    }
}