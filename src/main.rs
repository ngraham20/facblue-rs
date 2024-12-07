use std::io::Read;

use base64::{engine::general_purpose, Engine};
use flate2::{bufread::ZlibDecoder, read::DeflateDecoder};

fn main() {
    let blueprint = String::from("0eNptj9EKgzAMRf8lz53gtFP7K0OkzrAFalpqlYn039e6h73sMeHec5IDRrOi88QB1AH0sLyAuh+w0JO1yTvWM4KCiRZn9H5xmtFAFEA84RtUGXsByIEC4bd6DvvA6zyiTwHxHyHA2SW1LGdLJlWykAJ2UE1TyGRIxwRvzTDiS29kfc457RMsoD9dsY9ZTwHnxP+9ImBLiRMtb9eu7jpZV1Xblm2MH8/RUbc=");
    let bytes = general_purpose::STANDARD.decode(&blueprint[1..]).unwrap();
    let mut deflater = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    deflater.read_to_string(&mut s).expect("failed to read string");
    println!("{}", s);
}

enum Signal {
    Virtual(String),
    Recipe(String),
    Entity(String),
    SpaceLocation(String),
    Item(String),
    Quality(String),
    AsteroidChunk(String),
}

