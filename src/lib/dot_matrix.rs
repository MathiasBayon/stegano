pub mod dot_matrix {

    extern crate image;
    use self::image::{DynamicImage, GenericImage, ImageError, Pixel};

    use std::{fmt, str};

    use std::io::{Error, ErrorKind};

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    use lib::binary::{binary, binary::Byte};

    use lib::cypher::cypher;

    // TODO : put this in external file, or as input parameter
    const ENDING_CHAR: char = '~';

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
            self.input_filepath.clone()
        }

        /// Setter allowing the user to change the source picture
        pub fn read_from_file(&mut self, filepath: &str) {
            self.input_filepath = filepath.to_string();
            self.image = image::open(filepath);
        }

        /// Function to write the picture into target file
        /// TODO : refactor
        pub fn write_to_file(&self, filepath: &str) -> Result<(), Error> {
            match &self.image {
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
            let ref mut image_unwrapped;
            let pixel;

            match self.image {
                Ok(ref mut image_unwrapped_temp) => image_unwrapped = image_unwrapped_temp,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "stegano/store_3bits_at : Unable to open inner image!",
                    ))
                }
            }

            pixel = image_unwrapped.get_pixel(x, y);

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
            image_unwrapped.put_pixel(
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

            match &self.image {
                Ok(ref image_unwrapped) => {
                    pixel = image_unwrapped.get_pixel(x, y);
                }
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "stegano/get_3bits_at : Unable to get pixel at coordinates ({},{})",
                            x, y
                        ),
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
            self.get_dimensions().0 * self.get_dimensions().1 * 3 + 1 /* ENDING_CHAR */ > message_len
        }

        /// Store random bits from input x and y coordinates, to hide encrypted message length
        fn store_random_from(&mut self, mut x: u32, mut y: u32) -> Result<(), Error> {
            let image_unwrapped;
            let (max_x, max_y) = self.get_dimensions();

            match self.image {
                Ok(ref mut image_unwrapped_temp) => image_unwrapped = image_unwrapped_temp,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "stegano/store_random_from : Unable to open inner image!",
                    ))
                }
            }

            // Ensure to stays within picture boundaries
            while {
                let pixel_unwrapped = image_unwrapped.get_pixel(x, y);

                let mut red = Byte::new(pixel_unwrapped.to_rgb()[0]);
                let mut green = Byte::new(pixel_unwrapped.to_rgb()[1]);
                let mut blue = Byte::new(pixel_unwrapped.to_rgb()[2]);

                // Store random bits in least significant bit of each color RGB component
                red.store_random_bit();
                green.store_random_bit();
                blue.store_random_bit();

                // Create new pixel from altered RGB values and put it in image
                // Alpha is not altered
                image_unwrapped.put_pixel(
                    x,
                    y,
                    image::Rgba([
                        red.get_value(),
                        green.get_value(),
                        blue.get_value(),
                        pixel_unwrapped[3],
                    ]),
                );

                match move_cursor_to_next_pixel(&mut x, &mut y, (max_x, max_y)) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            } {}

            Ok(())
        }

        /// Encode given file in self image
        pub fn encode_file(&mut self, filepath: &str, password: &str) -> Result<(), Error> {
            let input_file = File::open(filepath)?;
            let mut buf_reader = BufReader::new(input_file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;
            self.encode(contents.as_str(), password)
        }

        /// Encode given message in self image
        pub fn encode(&mut self, message: &str, password: &str) -> Result<(), Error> {
            if !binary::is_one_byte_chars_message(message) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input message must be 1 byte chars",
                ));
            }
            if !binary::is_one_byte_chars_message(password) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input password must be 1 byte chars",
                ));
            }
            if password.len() < 8 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input password must be at least 8 letters long",
                ));
            }

            // Add ending character to input message
            let mut encrypted_message = cypher::simple_encrypt(message, password)?;

            add_ending_char(&mut encrypted_message);

            // Convert message to binary vector
            let vector = binary::convert_byte_vec_to_bit_vec(&binary::convert_u8_vec_to_byte_vec(
                &encrypted_message,
            ));

            // Get binary vector length
            let message_length = vector.len();

            // Check if picture is big enough to store binary vector
            if !self.is_big_enough_to_store_message(message_length as u32) {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "stegano/encode : Input file not big enough to store message!",
                ));
            }

            // Position indexes
            let mut x = 0;
            let mut y = 0;

            let (max_x, max_y) = self.get_dimensions();

            // Parsing cursor : as we can store 3 bits in a pixel
            // this index ensures that we progress 3 bits by 3 bits in the message converted as vector
            // which we want to encore
            let mut parsing_cursor = 0;

            // Ensure to stays within picture boundaries
            while {
                // Check if message end is reached and if there is less than 3 bits to write into picture
                if parsing_cursor + 2 >= message_length {
                    // If so, dump remaining vector bits into last pixel
                    self.store_3bits_at(x, y, &vector[parsing_cursor..])?;

                    // Store random things to hide picture alteration from picture analysers
                    if self.x_in_dimensions(x + 1) {
                        self.store_random_from(x + 1, y)?;
                    } else if self.y_in_dimensions(y + 1) {
                        self.store_random_from(0, y + 1)?;
                    } else {
                    }

                    return Ok(()); // Hell yeah, it's finished !
                } else {
                    // If not, store 3 bits in current pixel
                    self.store_3bits_at(x, y, &vector[parsing_cursor..parsing_cursor + 3])?;
                }

                parsing_cursor += 3;

                match move_cursor_to_next_pixel(&mut x, &mut y, (max_x, max_y)) {
                    Ok(_) => true,
                    Err(_) => false,
                } // Error seems to be here !
            } {}

            Err(Error::new(
                ErrorKind::Other,
                "stegano/encode : Input file not big enough to store message",
            )) // Should not happen
        }

        /// Decodes image and return result file
        pub fn decode_file(&self, filepath: &str, password: &str) -> Result<(), Error> {
            let decoded_string = &self.decode(password)?;
            let mut output_file = File::create(filepath)?;
            output_file.write_all(decoded_string.as_bytes())
        }

        /// Decodes image and return result string
        pub fn decode(&self, password: &str) -> Result<String, Error> {
            if !binary::is_one_byte_chars_message(password) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Input password must be 1 byte chars",
                ));
            }

            // Position indexes
            let mut x = 0;
            let mut y = 0;

            let (max_x, max_y) = self.get_dimensions();

            let mut bool_triplet_unwrapped;

            // Vector containing consolidated bits, before conversion in byte, then in char
            let mut boolean_byte_vector = Vec::<bool>::new();

            // Remains of previous bit vector, belonging to the next character
            let mut remains_of_previous_bit_vector = Vec::<bool>::new();

            // The decoded character
            let mut charac;

            // The message hidden in picture
            let mut message = Vec::<u8>::new();

            // Keep cursor within picture boundaries
            while {
                bool_triplet_unwrapped = self.get_3bits_at(x, y)?;

                if boolean_byte_vector.len() == 6 {
                    push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                        &mut boolean_byte_vector,
                        &bool_triplet_unwrapped,
                        0,
                        1,
                    );
                    push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                        &mut remains_of_previous_bit_vector,
                        &bool_triplet_unwrapped,
                        2,
                        2,
                    );
                } else if boolean_byte_vector.len() == 7 {
                    push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                        &mut boolean_byte_vector,
                        &bool_triplet_unwrapped,
                        0,
                        0,
                    );
                    push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                        &mut remains_of_previous_bit_vector,
                        &bool_triplet_unwrapped,
                        1,
                        2,
                    );
                } else if boolean_byte_vector.len() == 8 {
                    // Complete byte formed!
                    // Convert byte to character
                    charac = Byte::from_bool_vec(&boolean_byte_vector)?.get_value();

                    // Check if read character is the ending one
                    if charac == ENDING_CHAR as u8 {
                        match cypher::simple_decrypt(&message, password) {
                            Ok(str_str) => return Ok(str_str.to_owned()),
                            Err(err) => {
                                return Err(Error::new(ErrorKind::InvalidData, err.to_string()))
                            }
                        };
                    } else {
                        // Continue fetching pixels to retrieve the missing characters
                        message.push(charac);

                        boolean_byte_vector = Vec::<bool>::new();
                        boolean_byte_vector.append(&mut remains_of_previous_bit_vector);

                        remains_of_previous_bit_vector = Vec::<bool>::new();

                        push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                            &mut boolean_byte_vector,
                            &bool_triplet_unwrapped,
                            0,
                            2,
                        );
                    }
                } else {
                    push_bits_in_vector_from_bool_triplet_from_n_up_to_m(
                        &mut boolean_byte_vector,
                        &bool_triplet_unwrapped,
                        0,
                        2,
                    );
                }

                match move_cursor_to_next_pixel(&mut x, &mut y, (max_x, max_y)) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            } {}

            Err(Error::new(
                ErrorKind::InvalidData,
                "stegano/decode : Nothing hidden in this file!",
            )) // Ending character never reached, return None
        }
    }

    /// Append ending character to message
    fn add_ending_char(message: &mut Vec<u8>) {
        message.push(ENDING_CHAR as u8);
    }

    fn move_cursor_to_next_pixel(
        x: &mut u32,
        y: &mut u32,
        dimensions: (u32, u32),
    ) -> Result<(), Error> {
        if *x + 1 == dimensions.0 {
            if *y + 1 == dimensions.1 {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "No more pixels available in image!",
                ))
            } else {
                *x = 0;
                *y += 1;
                Ok(())
            }
        } else {
            *x += 1;
            Ok(())
        }
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
    use super::dot_matrix::DotMatrix;
    use std::{
        fs::{self, File}, io::{Read, Write}, process,
    };

    #[test]
    // TODO : unable to store special characters
    // TODO : errors triggering in a very useless order
    fn test_global() {
        let _ = fs::remove_file("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.png");

        // TODO : relative path
        let mut image =
            DotMatrix::new("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test.png");

        image
            .encode("Hello how is the weather today", "Password")
            .unwrap_or_else(|err| {
                eprintln!("Error in test_global: {}", err);
                process::exit(1);
            });

        image
            .write_to_file("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.png")
            .unwrap_or_else(|err| {
                eprintln!("Error in test_global: {}", err);
                process::exit(1);
            });

        let image2 =
            DotMatrix::new("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.png");
        let res = image2.decode("Password").unwrap_or_else(|err| {
            eprintln!("Error in test_global: {}", err);
            process::exit(1);
        });

        assert_eq!(res, "Hello how is the weather today".to_string());
    }

    #[test]
    fn test_global_with_file_encoding() {
        let _ = fs::remove_file("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test_global.png");
        let _ = fs::remove_file("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test.txt");
        let _ = fs::remove_file("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.txt");

        // TODO : relative path
        let mut image =
            DotMatrix::new("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test.png");
        
        let mut file = File::create(
            "/Users/mathias/Documents/Devs/Rust/stegano/test_files/test.txt",
        ).unwrap_or_else(|err| {
            eprintln!("Error in test_global: {}", err);
            process::exit(1);
        });

        file.write_all("Test message within file".as_bytes())
            .unwrap_or_else(|err| {
                eprintln!("Error in test_global: {}", err);
                process::exit(1);
            });

        image
            .encode_file(
                "/Users/mathias/Documents/Devs/Rust/stegano/test_files/test.txt",
                "Password",
            )
            .unwrap_or_else(|err| {
                eprintln!("Error in test_global: {}", err);
                process::exit(1);
            });

        image
            .write_to_file("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test_global.png")
            .unwrap_or_else(|err| {
                eprintln!("Error in test_global: {}", err);
                process::exit(1);
            });

        let image2 =
            DotMatrix::new("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test_global.png");
        let _ = image2
            .decode_file(
                "/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.txt",
                "Password",
            )
            .unwrap_or_else(|err| {
                eprintln!("Error in test_global: {}", err);
                process::exit(1);
            });

        let mut file = File::open(
            "/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.txt",
        ).unwrap_or_else(|err| {
            eprintln!("Error in test_global: {}", err);
            process::exit(1);
        });

        let mut result_string = String::new();
        let _ = file
            .read_to_string(&mut result_string)
            .unwrap_or_else(|err| {
                eprintln!("Error in test_global: {}", err);
                process::exit(1);
            });

        assert_eq!("Test message within file".to_string(), result_string);
    }
}
