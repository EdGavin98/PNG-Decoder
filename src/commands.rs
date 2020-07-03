use crate::args::{Decode, Encode, Remove};
use std::fs;
use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use std::str::FromStr;
use std::convert::TryFrom;
use std::path::PathBuf;

pub fn encode(args: Encode) {
    let mut image = load_to_png(&args.file);

    let chunk_type = ChunkType::from_str(&args.chunk_name).unwrap();
    let data = args.message.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, data);

    image.append_chunk(chunk);
    match args.output_file {
        Some(out) => fs::write(out, image.as_bytes()).unwrap(),
        None => fs::write(args.file, image.as_bytes()).unwrap()
    }
}

pub fn decode(args: Decode) {
    let image = load_to_png(&args.file);
    let message_chunk = image.chunk_by_type(args.chunk_name.as_str());

    match message_chunk {
        Some(chunk) => println!("{}", chunk.data_as_string().unwrap()),
        None => println!("No chunk found with this ID")
    }
}

pub fn delete(args: Remove) {
    let mut image = load_to_png(&args.file);
    image.remove_chunk(args.chunk_name.as_str()).unwrap();
}

fn load_to_png(path: &PathBuf) -> Png {
    let file = fs::read(path).expect("File does not exist");
    Png::try_from(file.as_slice()).expect("Invalid file")
}