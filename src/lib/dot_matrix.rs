pub mod dot_matrix {

    extern crate image;
    use self::image::DynamicImage;
    use self::image::GenericImage;
    use self::image::Pixel;

    use std::fs::File;

    use lib::binary::binary;

    /// Basic structure : a DynamicImage and a filepath
    pub struct DotMatrix {
        image: DynamicImage,
        input_filepath: String
    }

    impl DotMatrix {

        /// Constructor
        pub fn new(filepath: &str) -> DotMatrix {
            DotMatrix {
                input_filepath: filepath.to_string(),
                image: image::open(filepath).unwrap()
            }
        }

        /// Accessor returning picture dimensions as a tuple pixel
        pub fn get_dimensions(&self) -> (u32, u32) {
            self.image.dimensions()
        }

        /// Accessor returning picture's filepath
        pub fn get_input_filepath(&self) -> String {
            self.input_filepath.to_string()
        }

        /// Setter allowing the user to change the source picture
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
        fn store_3bits_at(&mut self, x: u32, y: u32, bits:&[bool]) {
            // Get the pixel at input coordinates
            let ref mut pixel = self.image.get_pixel(x, y);
            let mut red = pixel.to_rgb()[0];
            let mut green = pixel.to_rgb()[1];
            let mut blue = pixel.to_rgb()[2];
            let a = pixel[3];

            // Then binary-OR it with the boolean input values
            if bits.len() >= 1 {
                binary::store_bit_in_u8(&mut red, bits[0]);
            }
            if bits.len() >= 2 {
                binary::store_bit_in_u8(&mut green, bits[1]);
            }
            if bits.len() == 3 {
                binary::store_bit_in_u8(&mut blue, bits[2]);
            }

            self.image.put_pixel(x, y, image::Rgba([red, green, blue, a]));
        }

        fn get_3bits_at(&self, x: u32, y: u32) -> [bool;3] {
            let ref mut pixel = self.image.get_pixel(x, y);
            [   if pixel.to_rgb()[0] % 2 == 0 { false } else { true }
            ,   if pixel.to_rgb()[1] % 2 == 0 { false } else { true }
            ,   if pixel.to_rgb()[2] % 2 == 0 { false } else { true }
            ]
        }

        fn get_msg_w_ending_character(&self, message: &str) -> String {
            let mut message2 = message.to_string();
            message2.push('#');
            message2
        }

        fn x_in_dimensions(&self, x:u32) -> bool {
            x < self.get_dimensions().0
        }

        fn y_in_dimensions(&self, y:u32) -> bool {
            y < self.get_dimensions().1
        }

        pub fn encode(&mut self, message: &str) {
            let message_w_ending_character = self.get_msg_w_ending_character(message);
            let vector = binary::convert_message_to_bit_vec(message_w_ending_character);
            let message_length = vector.len();
            let mut x = 0;
            let mut y = 0;
            let mut parsing_cursor = 0;

            while self.y_in_dimensions(y) {
                while self.x_in_dimensions(x) {
                    if parsing_cursor+2 > message_length { self.store_3bits_at(x,y, &vector[parsing_cursor..]); return; }
                    self.store_3bits_at(x,y, &vector[parsing_cursor..parsing_cursor+3]);
                    // println!("{:?},{:?} : {:?}", x,y,& vector[parsing_cursor..parsing_cursor+3]);
                    x += 1;
                    parsing_cursor +=3
                }
                y += 1;
                x = 0;
            }
        }

        pub fn decode(&self) -> String {
            let mut x = 0;
            let mut y = 0;
            let mut boolean_byte_vector = Vec::<bool>::new();
            let mut remains_of_previous_bit_vector = Vec::<bool>::new();
            let mut charac;
            let mut message = String::new();

            while self.y_in_dimensions(y) {
                while self.x_in_dimensions(x) {
                    if boolean_byte_vector.len() > 5 {
                        if boolean_byte_vector.len() == 6 {
                            boolean_byte_vector.push(self.get_3bits_at(x,y)[0]);
                            boolean_byte_vector.push(self.get_3bits_at(x,y)[1]);
                            remains_of_previous_bit_vector.push(self.get_3bits_at(x,y)[2]);
                        } else if boolean_byte_vector.len() == 7 {
                            boolean_byte_vector.push(self.get_3bits_at(x,y)[0]);
                            remains_of_previous_bit_vector.push(self.get_3bits_at(x,y)[1]);
                            remains_of_previous_bit_vector.push(self.get_3bits_at(x,y)[2]);
                        } else {
                            charac = binary::convert_bit_vec_to_message(&boolean_byte_vector);
                            println!("Char : {}", charac);
                            if charac.eq("#") {
                                return message;
                            } else {
                                message.push(charac.chars().next().unwrap());
                                boolean_byte_vector = Vec::<bool>::new();
                                boolean_byte_vector.append(&mut remains_of_previous_bit_vector);
                                remains_of_previous_bit_vector = Vec::<bool>::new();
                                boolean_byte_vector.push(self.get_3bits_at(x,y)[0]);
                                boolean_byte_vector.push(self.get_3bits_at(x,y)[1]);
                                boolean_byte_vector.push(self.get_3bits_at(x,y)[2]);
                            }
                        }
                    } else {
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[0]);
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[1]);
                        boolean_byte_vector.push(self.get_3bits_at(x,y)[2]);
                    }
                    
                    x += 1;
                }
                y += 1;
                x = 0;
            }
        "ERROR".to_string()
        }
    }
}