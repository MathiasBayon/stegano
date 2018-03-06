mod lib;
use self::lib::dot_matrix::dot_matrix::DotMatrix;
use self::lib::binary::binary;

fn main() {
    let mut image = DotMatrix::new("/Users/mathias/Documents/Devs/Rust/stegano/test.png");
    image.encode("Hello").unwrap();

    println!("{:?}", image.write_to_file("/Users/mathias/Documents/Devs/Rust/stegano/test2.png"));

    let vector = binary::convert_message_to_bit_vec("Hello".to_string());
    println!("{:?}", vector);

    let image2 = DotMatrix::new("/Users/mathias/Documents/Devs/Rust/stegano/test2.png");
    let res = image2.decode().unwrap();

    println!("{}", res);
}
