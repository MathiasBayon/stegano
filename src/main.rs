mod lib;
use self::lib::dot_matrix::dot_matrix::DotMatrix;

fn main() {
    let mut image = DotMatrix::new("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test.png");
    image.encode("Hello").unwrap();

    image.write_to_file("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.png");

    let image2 = DotMatrix::new("/Users/mathias/Documents/Devs/Rust/stegano/test_files/test2.png");
    let res = image2.decode().unwrap();

    println!("{}", res);
}
