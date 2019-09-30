//! Various String / Vec utilities to manage bit streams, via Byte struct abstraction
extern crate rand;

use self::rand::Rng;
use std::{
    fmt,
    io::{Error, ErrorKind},
};

/// A Byte, containing a value...
#[derive(Debug)]
pub struct Byte {
    value: u8,
}

impl Byte {
    /// Constructor
    pub fn new(value: u8) -> Byte {
        Byte { value }
    }

    /// Public accessor for value
    pub fn get_value(&self) -> u8 {
        self.value
    }

    /// Convert bool vector into Byte
    pub fn from_bool_array(bit_array: &[bool]) -> Result<Byte, Error> {
        if bit_array.len() > 8 {
            return Err(Error::new(
                ErrorKind::Other,
                "Input bit_vec is too long to be stored in one Byte",
            ));
        }

        Ok(Byte {
            value: bit_array
                .iter()
                .map(|b| self::convert_bool_to_u8(*b))
                .fold(0, |acc, b| acc * 2 + b as u8),
        })
    }

    /// Sets the least significant bit to 0 (input parameter)
    pub fn empty_least_significant_bit(&mut self) {
        if self.value % 2 == 1 {
            self.value -= 1 // Minus one to set it empty, do nothing if already empty
        }
    }

    /// Store bit in self (Input parameters)
    /// empties least significant one and sets 0 or 1 into it
    pub fn store_bit(&mut self, bit: bool) {
        self.empty_least_significant_bit();

        if bit {
            self.value += 1;
        }
    }

    /// Store random bit in self (Input parameter)
    pub fn store_random_bit(&mut self) {
        self.empty_least_significant_bit();

        let random_bit_value = rand::thread_rng().gen_range(0, 2);

        if random_bit_value > 0 {
            self.store_bit(true);
        } else {
            self.store_bit(false);
        }
    }

    /// Converts self into a boolean (~= binary) vector
    pub fn to_bit_vec(&self) -> Vec<bool> {
        self.to_string()
            .chars()
            .map(|c| c == '1')
            .collect()
    }

    /// Concerts byte into String (binary value)
    pub fn to_string(&self) -> String {
        format!("{:08b}", self.value)
    }
}

/// Return true if chr is a one-byte
pub fn is_one_byte_char(chr: char) -> bool {
    (chr as u8) < 128
}

/// Return true is msg is only made of one-byte chars
pub fn is_one_byte_chars_message(msg: &str) -> bool {
    msg.chars().all(is_one_byte_char)
}

/// Convert boolean value to 1 / 0, as u8
pub fn convert_bool_to_u8(bool_value: bool) -> u8 {
    if bool_value {
        1
    } else {
        0
    }
}

/// Convert Byte vector into boolean (~= binary) vector
/// TODO : refactor with functional programming
pub fn convert_byte_vec_to_bit_array(byte_array: &[Byte]) -> Vec<bool> {
    // Initialize output boolean vector
    let mut message_as_binary_vector = Vec::<bool>::new();

    // Parse message byte by byte until end
    for i in 0..byte_array.len() {
        // Convert byte into byte bit vector
        let mut bitvec = byte_array[i].to_bit_vec();

        // Append byte bit vector to main output vit vector
        message_as_binary_vector.append(&mut bitvec);
    }

    // And return it
    message_as_binary_vector
}

/// Convert u8 vector into Byte vector
pub fn convert_u8_vec_to_byte_array(u8_array: &[u8]) -> Vec<Byte> {
    u8_array.iter().map(|&e| Byte::new(e)).collect()
}

/// Traits implementation
impl PartialEq for Byte {
    fn eq(&self, other: &Byte) -> bool {
        self.value == other.value
    }
}

impl Eq for Byte {}

impl Clone for Byte {
    fn clone(&self) -> Byte {
        Byte::new(self.get_value())
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.value)
    }
}

impl std::str::FromStr for Byte {
    type Err = ErrorKind;
    /// Convert binary-byte-looking input string to byte
    fn from_str(bit_str: &str) -> Result<Self, Self::Err> {
        // Slice the string into chars
        // Map '0' and '1' chars to 0s and 1s
        // Fold the whole "byte-string" into a number
        // And return it
        match bit_str.len() {
            8 => Ok(Byte {
                value: bit_str
                    .chars()
                    .map(|x| match x {
                        '1' => 1,
                        _ => 0,
                    })
                    .fold(0, |acc, b| acc * 2 + b as u8),
            }),
            _ => Err(ErrorKind::InvalidData),
        }
    }
}

// Tests
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::str::FromStr;

    #[test]
    fn test_constructor_accessor() {
        let byte = Byte::new(31);
        assert_eq!(byte.get_value(), 31);
    }

    #[test]
    fn test_from_bool_vec() {
        let tab = [false, false, false, true, true, true, true, true];

        let vec = &tab.to_vec();

        assert_eq!(
            Byte::from_bool_array(&vec).expect("Unable to convert bitvec to Byte!"),
            Byte::new(31)
        );
    }

    #[test]
    fn test_empty_least_significant_bit() {
        let mut byte = Byte::new(31);

        byte.empty_least_significant_bit();
        assert_eq!(byte, Byte::new(30));
        byte.empty_least_significant_bit();
        assert_eq!(byte, Byte::new(30));
    }

    #[test]
    fn test_store_bit() {
        let mut byte = Byte::new(31);

        byte.store_bit(true);
        assert_eq!(byte, Byte::new(31));
        byte.store_bit(false);
        assert_eq!(byte, Byte::new(30));
    }

    #[test]
    fn test_store_random_bit() {
        let mut cases = Vec::<u8>::new();

        for _ in 0..100 {
            let mut byte = Byte::new(31);
            byte.store_random_bit();
            cases.push(byte.get_value());
        }

        let set: BTreeSet<_> = cases.drain(..).collect();
        let mut set2 = BTreeSet::new();

        let value30 = 30;
        let value31 = 31;

        set2.insert(value30);
        set2.insert(value31);

        assert_eq!(set, set2);
    }

    #[test]
    fn test_to_bit_vec() {
        let bit_vec = vec![false, false, false, true, true, true, true, true];

        assert_eq!(Byte::new(31).to_bit_vec(), bit_vec);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            Byte::from_str("00011111").expect("Unable to convert bitstring to Byte!"),
            Byte::new(31)
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Byte::from_str("00011111")
                .expect("Unable to convert bitstring to Byte!")
                .to_string(),
            "00011111".to_owned()
        );
    }

    #[test]
    fn test_convert_byte_vec_to_bit_vec() {
        let byte_vec = vec![Byte::new(31), Byte::new(31), Byte::new(31)];

        let bit_vec = vec![
            false, false, false, true, true, true, true, true, false, false, false, true, true,
            true, true, true, false, false, false, true, true, true, true, true,
        ];

        assert_eq!(convert_byte_vec_to_bit_array(&byte_vec), bit_vec);
    }

    #[test]
    fn test_convert_u8_vec_to_byte_vec() {
        let u8_vec = vec![128, 11, 135];
        let byte_vec = vec![Byte::new(128), Byte::new(11), Byte::new(135)];

        assert_eq!(convert_u8_vec_to_byte_array(&u8_vec), byte_vec);
    }

    #[test]
    fn test_is_one_byte_char() {
        assert_eq!(is_one_byte_char('A'), true);
        assert_eq!(is_one_byte_char('€'), false);
    }

    #[test]
    fn test_is_one_byte_chars_message() {
        assert_eq!(is_one_byte_chars_message("Very nice message"), true);
        assert_eq!(is_one_byte_chars_message("Véry ugly méssàge !!!"), false);
    }

    #[test]
    fn test_convert_bool_to_u8() {
        assert_eq!(convert_bool_to_u8(true), 1);
    }
}
