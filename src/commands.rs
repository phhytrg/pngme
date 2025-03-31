use std::fs;

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

fn read_png(file_path: &str) -> Vec<u8> {
    match fs::read(file_path) {
        Err(err) => panic!("Error while read file: {}", err),
        Ok(bytes) => bytes,
    }
}

pub fn encode(file_path: &str, chunk_type: &ChunkType, message: &str, output: Option<&str>) {
    let mut png = Png::try_from(read_png(&file_path).as_slice()).unwrap();
    let new_chunk = Chunk::new(chunk_type.clone(), message.chars().map(|c| c as u8).collect());
    png.append_chunk(new_chunk);
    fs::write(output.unwrap_or(&file_path), png.as_bytes()).unwrap();
}

pub fn decode(file_path: &str, chunk_type: &ChunkType) {
    let png = Png::try_from(read_png(&file_path).as_slice()).unwrap();
    let Some(chunk) = png.chunk_by_chunk_type(chunk_type) else {
        panic!();
    };
    let message = chunk.data_as_string().unwrap();
    println!("message from \"{}\" is \"{}\"", chunk_type, message);
}

pub fn remove(file_path: &str, chunk_type: &ChunkType) {
    let mut png = Png::try_from(read_png(&file_path).as_slice()).unwrap();
    png.remove_chunks(chunk_type);
    fs::write(file_path, png.as_bytes()).unwrap();
}

pub fn print_png(file_path: &str) {
    let png = Png::try_from(read_png(&file_path).as_slice()).unwrap();
    println!("{}", png)
}
