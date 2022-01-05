use std::io::Write;

use deku::{DekuContainerRead, DekuContainerWrite};
use rust_tdf::TdfHeader;

const FILENAME: &str = "C:/Users/marcha/Desktop/Marcos/rust-tdf/src/test.tdf";

// Read the file and return a binary buffer
fn main() {
    match std::fs::read(FILENAME) {
        Ok(bytes) => {
            let (_rest, header) = TdfHeader::from_bytes((bytes.as_ref(), 0)).unwrap_or_else(|e| {
                eprintln!("{}", e);
                panic!("{}", e);
            });
            println!("{:?}", header);

            // write header bytes to new file
            match header.to_bytes() {
                Ok(bytes) => {
                    let mut file = std::fs::File::create("gato.tdf").unwrap();
                    file.write_all(&bytes).unwrap();
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Err(error) => {
            if error.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("You don't have permission to read the file");
                return;
            } else {
                eprintln!("An error occurred while reading the file: {:?}", error);
                panic!("{:?}", error);
            }
        }
    }
}
