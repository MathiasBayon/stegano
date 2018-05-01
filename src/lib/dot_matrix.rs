pub mod dot_matrix {

    extern crate image;
    use self::image::{DynamicImage, GenericImage, ImageError, Pixel};

    use std::{fmt,
              io::{Error, ErrorKind}};

    use lib::binary::{binary, binary::Byte};
    use lib::cypher::*;

    // TODO : put this in external file, or as input parameter
    const ENDING_CHAR: char = '‰';

    /// Basic structure : a DynamicImage and a filepath
    pub struct DotMatrix {
        image: Result<DynamicImage, ImageError>,
        input_filepath: String,
    }

    impl DotMatrix {
        /// Constructor
        pub fn new(filepath: &str) -> DotMatrix {
            DotMatrix {
                input_filepath: filepath.to_string(),
                image: image::open(filepath),
            }
        }

        /// Accessor returning picture dimensions as a tuple pixel
        pub fn get_dimensions(&self) -> (u32, u32) {
            match self.image {
                Ok(ref image) => image.dimensions(),
                Err(_) => (0, 0),
            }
        }

        /// Accessor returning picture's filepath
        pub fn get_input_filepath(&self) -> String {
            self.input_filepath.to_string()
        }

        /// Setter allowing the user to change the source picture
        pub fn read_from_file(&mut self, filepath: &str) {
            self.input_filepath = filepath.to_string();
            self.image = image::open(filepath);
        }

        /// Function to write the picture into target unexisting file
        pub fn write_to_file(&self, filepath: &str) -> Result<(), Error> {
            match self.image {
                Ok(ref image_inner) => match image_inner.save(filepath) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::new(
                        ErrorKind::InvalidInput,
                        "stegano/write_to_file : Unable to save output file!",
                    )),
                },
                Err(_) => Err(Error::new(
                    ErrorKind::InvalidData,
                    "stegano/write_to_file : Unable to open inner image!",
                )),
            }
        }

        /// Function to store 3 bits, hidden into pixel at input coordinates
        fn store_3bits_at(&mut self, x: u32, y: u32, bits: &[bool]) -> Result<(), Error> {
            // Get the pixel at input coordinates
            let ref mut image_unwraped;
            let pixel;

            // TODO : ???!!!
            match self.image {
                Ok(ref mut image_unwraped_temp) => image_unwraped = image_unwraped_temp,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "stegano/store_3bits_at : Unable to open inner image!",
                    ))
                }
            }

            pixel = image_unwraped.get_pixel(x, y);

            // Retrieve pixel components
            let mut red = Byte::new(pixel.to_rgb()[0]);
            let mut green = Byte::new(pixel.to_rgb()[1]);
            let mut blue = Byte::new(pixel.to_rgb()[2]);

            // Then binary-OR it with the boolean input values
            // If the end of the encryption message is reached,
            // then the bit array may have less than 3 elements, so, store what is storable
            if bits.len() >= 1 {
                red.store_bit(bits[0]);
            }
            if bits.len() >= 2 {
                green.store_bit(bits[1]);
            }
            if bits.len() == 3 {
                blue.store_bit(bits[2]);
            }

            // Create new pixel from altered RGB values and put it in image
            // Alpha is not altered
            image_unwraped.put_pixel(
                x,
                y,
                image::Rgba([
                    red.get_value(),
                    green.get_value(),
                    blue.get_value(),
                    pixel[3],
                ]),
            );

            Ok(())
        }

        /// Returns bits stored in pixel, at given position
        fn get_3bits_at(&self, x: u32, y: u32) -> Result<[bool; 3], Error> {
            let pixel;

            match self.image {
                Ok(ref image_unwraped) => {
                    pixel = image_unwraped.get_pixel(x, y);
                }
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "stegano/get_3bits_at :Unable to get pixel at coordinates",
                    ))
                }
            }

            // Use modulo to know whether value is odd or not
            Ok([
                if pixel.to_rgb()[0] % 2 == 0 {
                    false
                } else {
                    true
                },
                if pixel.to_rgb()[1] % 2 == 0 {
                    false
                } else {
                    true
                },
                if pixel.to_rgb()[2] % 2 == 0 {
                    false
                } else {
                    true
                },
            ])
        }

        /// Append ending character to message
        fn get_vector_w_ending_char(message: &Vec<u8>) -> Vec<u8> {
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
            self.get_dimensions().0 * self.get_dimensions().1 * 3 > message_len
        }

        /// Store random bits from input x and y coordinates, to hide encrypted message length
        fn store_random_from(&mut self, x: u32, y: u32) -> Result<(), Error> {
            // Shadow x and y into local mutable variables
            let mut x = x.clone();
            let mut y = y.clone();

            let image_unwraped;

            match self.image {
                Ok(ref mut image_unwraped_temp) => image_unwraped = image_unwraped_temp,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "stegano/store_random_from : Unable to open inner image!",
                    ))
                }
            }

            let dimension_x = image_unwraped.dimensions().0;
            let dimension_y = image_unwraped.dimensions().1;

            // Ensure to stays within picture boundaries
            while y < dimension_y {
                while x < dimension_x {
                    let pixel_unwraped = image_unwraped.get_pixel(x, y);

                    let mut red = Byte::new(pixel_unwraped.to_rgb()[0]);
                    let mut green = Byte::new(pixel_unwraped.to_rgb()[1]);
                    let mut blue = Byte::new(pixel_unwraped.to_rgb()[2]);

                    // Store random bits in least significant bit of each color RGB component
                    red.store_random_bit();
                    green.store_random_bit();
                    blue.store_random_bit();

                    // Create new pixel from altered RGB values and put it in image
                    // Alpha is not altered
                    image_unwraped.put_pixel(
                        x,
                        y,
                        image::Rgba([
                            red.get_value(),
                            green.get_value(),
                            blue.get_value(),
                            pixel_unwraped[3],
                        ]),
                    );

                    x += 1;
                }

                y += 1;
                x = 0;
            }

            Ok(())
        }

        fn push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
            vector: &mut Vec<bool>,
            bool_triplet: &[bool; 3],
            n: usize,
            m: usize,
        ) {
            for i in n..m + 1 {
                vector.push(bool_triplet[i]);
            }
        }

        /// Encode given message in self image
        pub fn encode(&mut self, message: &str, password: &str) -> Result<(), Error> {
            if !message.is_ascii() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input message must be ASCII",
                ));
            }
            if !password.is_ascii() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input password must be ASCII",
                ));
            }
            if password.len() < 8 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input password must be at least 8 letters long",
                ));
            }

            // TODO : to improve
            if message.contains(",;:=?./+ù`%£^$¨*-_°&@#‰|)àç!è§('<>Êêaæî") {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input message must not contain special characters",
                ));
            }

            // Add ending charadter to input message
            let encrypted_message = cypher::simple_encrypt(message, password);
            let message_w_ending_character;

            match encrypted_message {
                Ok(encrypted_message) => {
                    message_w_ending_character =
                        DotMatrix::get_vector_w_ending_char(&encrypted_message);
                }
                Err(error) => {
                    return Err(error);
                }
            }

            // Convert message to binary vetor
            let vector = binary::convert_byte_vec_to_bit_vec(&binary::convert_u8_vec_to_byte_vec(
                &message_w_ending_character,
            ));

            // Get binary vector length
            let message_length = vector.len();

            // Check if picture is big enoug to store binary vector
            if !self.is_big_enough_to_store_message(message_length as u32) {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "stegano/encode : Input file not big enough to store message!",
                ));
            }

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
                    if parsing_cursor + 2 > message_length {
                        // If so, dump remaining vector bits into last pixel
                        match self.store_3bits_at(x, y, &vector[parsing_cursor..]) {
                            Ok(_) => {}
                            Err(error) => return Err(error),
                        }

                        // Store random things to hide picture alteration from picture analysers
                        if self.x_in_dimensions(x + 1) {
                            match self.store_random_from(x + 1, y) {
                                Ok(_) => {}
                                Err(error) => return Err(error),
                            }
                        } else if self.y_in_dimensions(y + 1) {
                            match self.store_random_from(0, y + 1) {
                                Ok(_) => {}
                                Err(error) => return Err(error),
                            }
                        }

                        return Ok(()); // Hell yeah, it's finished !
                    }
                    // If not, store 3 bits in current pixel
                    match self.store_3bits_at(x, y, &vector[parsing_cursor..parsing_cursor + 3]) {
                        Ok(_) => {}
                        Err(error) => return Err(error),
                    }

                    x += 1;
                    parsing_cursor += 3
                }

                // Reset x coordinates to left and increase y
                y += 1;
                x = 0;
            }

            Err(Error::new(
                ErrorKind::Other,
                "stegano/encode : Input file not big enough to store message",
            )) // Should not happen
        }

        /// Decodes image and return result string
        pub fn decode(&self, password: &str) -> Result<String, Error> {
            if !password.is_ascii() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input password must be ASCII",
                ));
            }

            // Position indexes
            let mut x = 0;
            let mut y = 0;

            let mut bool_triplet_unwraped;

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
                    match self.get_3bits_at(x, y) {
                        Ok(bool_triplet_temp) => bool_triplet_unwraped = bool_triplet_temp,
                        Err(error) => return Err(error),
                    }

                    if boolean_byte_vector.len() == 6 {
                        DotMatrix::push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                            &mut boolean_byte_vector,
                            &bool_triplet_unwraped,
                            0,
                            1,
                        );
                        DotMatrix::push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                            &mut remains_of_previous_bit_vector,
                            &bool_triplet_unwraped,
                            2,
                            2,
                        );
                    } else if boolean_byte_vector.len() == 7 {
                        DotMatrix::push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                            &mut boolean_byte_vector,
                            &bool_triplet_unwraped,
                            0,
                            0,
                        );
                        DotMatrix::push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                            &mut remains_of_previous_bit_vector,
                            &bool_triplet_unwraped,
                            1,
                            2,
                        );
                    } else if boolean_byte_vector.len() == 8 {
                        // Complete byte formed!
                        // Convert byte to character
                        charac = Byte::from_bit_vec(&boolean_byte_vector)
                            .unwrap()
                            .get_value();

                        // Check if read character is the ending one
                        if charac == ENDING_CHAR as u8 {
                            let result = cypher::simple_decrypt(&message, password);

                            match result {
                                Ok(result) => return Ok(result),
                                Err(error) => return Err(error),
                            }
                        } else {
                            // Continue fetching pixels to retrieve the missing characters
                            message.push(charac);

                            boolean_byte_vector = Vec::<bool>::new();
                            boolean_byte_vector.append(&mut remains_of_previous_bit_vector);

                            remains_of_previous_bit_vector = Vec::<bool>::new();

                            DotMatrix::push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                                &mut boolean_byte_vector,
                                &bool_triplet_unwraped,
                                0,
                                2,
                            );
                        }
                    } else {
                        DotMatrix::push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                            &mut boolean_byte_vector,
                            &bool_triplet_unwraped,
                            0,
                            2,
                        );
                    }

                    x += 1;
                }

                // Reset x coordinates to left and increase y
                y += 1;
                x = 0;
            }

            Err(Error::new(
                ErrorKind::InvalidData,
                "stegano/decode : Nothing hidden in this file!",
            )) // Ending character never reached, return None
        }
    }

    /// Traits implementation
    impl Clone for DotMatrix {
        fn clone(&self) -> DotMatrix {
            DotMatrix::new(self.get_input_filepath().as_str())
        }
    }

    impl fmt::Display for DotMatrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Filepath: {} \n Contents: {} \n Dimensions : {:?}",
                self.get_input_filepath(),
                match self.image {
                    Ok(_) => "Yes",
                    Err(_) => "Invalid content",
                },
                self.get_dimensions()
            )
        }
    }
}

// Tests
#[cfg(test)]
pub mod tests {
    use super::dot_matrix;

    #[test]
    // TODO : unable to store special characters
    // TODO : errors triggering in a very unuseful order
    fn test_global() {
        // TODO : relative path
        let mut image = dot_matrix::DotMatrix::new(
            "/Users/mathias/Documents/Devs/Rust/stegano/test_files/test.png",
        );
        image
            .encode("Hello, how is the weather today", "Passesazeaze")
            .unwrap();

        assert_eq!(
            image
                .write_to_file("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.png")
                .unwrap(),
            ()
        );

        let image2 = dot_matrix::DotMatrix::new(
            "/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.png",
        );
        let res = image2.decode("Passesazeaze").unwrap();

        assert_eq!(res, "Hello, how is the weather today".to_string());
    }
}
