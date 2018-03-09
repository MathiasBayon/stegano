// encrypt and decrypt methods created from examples available in rust-crypto Github page
pub mod cypher {
    extern crate crypto;

    use self::crypto::{ symmetriccipher, buffer, aes, blockmodes };
    use self::crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

    use std::str;

    // Encrypt a buffer with the given key and iv using
    // AES-256/CBC/Pkcs encryption.
    fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        // Create an encryptor instance of the best performing
        // type available for the platform.
        let mut encryptor = aes::cbc_encryptor(
                aes::KeySize::KeySize256,
                key,
                iv,
                blockmodes::PkcsPadding);

        // Each encryption operation encrypts some data from
        // an input buffer into an output buffer. Those buffers
        // must be instances of RefReaderBuffer and RefWriteBuffer
        // (respectively) which keep track of how much data has been
        // read from or written to them.
        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        // Each encryption operation will "make progress". "Making progress"
        // is a bit loosely defined, but basically, at the end of each operation
        // either BufferUnderflow or BufferOverflow will be returned (unless
        // there was an error). If the return value is BufferUnderflow, it means
        // that the operation ended while wanting more input data. If the return
        // value is BufferOverflow, it means that the operation ended because it
        // needed more space to output data. As long as the next call to the encryption
        // operation provides the space that was requested (either more input data
        // or more output space), the operation is guaranteed to get closer to
        // completing the full operation - ie: "make progress".
        //
        // Here, we pass the data to encrypt to the enryptor along with a fixed-size
        // output buffer. The 'true' flag indicates that the end of the data that
        // is to be encrypted is included in the input buffer (which is true, since
        // the input data includes all the data to encrypt). After each call, we copy
        // any output data to our result Vec. If we get a BufferOverflow, we keep
        // going in the loop since it means that there is more work to do. We can
        // complete as soon as we get a BufferUnderflow since the encryptor is telling
        // us that it stopped processing data due to not having any more data in the
        // input buffer.
        loop {
            let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

            // "write_buffer.take_read_buffer().take_remaining()" means:
            // from the writable buffer, create a new readable buffer which
            // contains all data that has been written, and then access all
            // of that data as a slice.
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }

        Ok(final_result)
    }

    // Decrypts a buffer with the given key and iv using
    // AES-256/CBC/Pkcs encryption.
    //
    // This function is very similar to encrypt(), so, please reference
    // comments in that function. In non-example code, if desired, it is possible to
    // share much of the implementation using closures to hide the operation
    // being performed. However, such code would make this example less clear.
    fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut decryptor = aes::cbc_decryptor(
                aes::KeySize::KeySize256,
                key,
                iv,
                blockmodes::PkcsPadding);

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }

        Ok(final_result)
    }

    /// Simple encrypter to encapsulate crypto functions
    pub fn simple_encrypt(message: &str, password: &str) -> Result<Vec<u8>, &'static str> {
        let iv: [u8; 16] = [0; 16];
        let key = password.as_bytes();

        // In a real program, the key and iv may be determined
        // using some other mechanism. If a password is to be used
        // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
        // supported by Rust-Crypto!) would be a good choice to derive
        // a password. For the purposes of this example, the key and
        // iv are just random values.
        // let mut rng = OsRng::new().ok().unwrap();
        // rng.fill_bytes(&mut key);
        // rng.fill_bytes(&mut iv);

        match encrypt(&message.as_bytes(), &key, &iv) {
            Ok(ok) => Ok(ok),
            Err(_) => Err("stegano/simple_encrypt : Unable to encrypt message!")
        }
    }
    
    /// Simple decrypter§è to encapsulate crypto functions
    pub fn simple_decrypt(vector: &Vec<u8>, password: &str) -> Result<String, &'static str> {
        let iv: [u8; 16] = [0; 16];
        let key = password.as_bytes();

        // In a real program, the key and iv may be determined
        // using some other mechanism. If a password is to be used
        // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
        // supported by Rust-Crypto!) would be a good choice to derive
        // a password. For the purposes of this example, the key and
        // iv are just random values.
        // let mut rng = OsRng::new().ok().unwrap();
        // rng.fill_bytes(&mut key);
        // rng.fill_bytes(&mut iv);

        println!("vec {:?}", vector);

        match decrypt(vector, &key, &iv) {
            Ok(decrypted_message) => {
                match str::from_utf8(decrypted_message.as_slice()) {
                    Ok(decrypted_message_as_str) => Ok(decrypted_message_as_str.to_string()),
                    Err(_) => { Err("stegano/simple_decrypt : Unable to convert decrypted message to UTF8") }
                }
            },
            Err(_) => { Err("stegano/simple_decrypt : Unable to decrypt message") }
        }
    }
}

// Tests
#[cfg(test)]
pub mod tests {
    use super::cypher;

    #[test]
    fn test_simple_encrypt_decrypt() {
        let encrypted = cypher::simple_encrypt("Hello, how is the weather today", "Pass").unwrap();

        assert_eq!(cypher::simple_decrypt(&encrypted, "Pass"), Ok("Hello, how is the weather today".to_string()));
    }
}