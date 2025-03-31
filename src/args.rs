use clap::{arg, Parser, Subcommand};

use crate::chunk_type::ChunkType;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Commands (encode, decode, remove, print)
    #[command(subcommand)]
    pub command: Option<PngMeArgs>,
}

#[derive(Subcommand, Debug)]
pub enum PngMeArgs {
    /// Encodes a custom message into a PNG file.
    Encode(EncodeArgs),
    /// Decodes and retrieves a hidden message from a PNG file.
    Decode(DecodeArgs),
    /// Removes a custom chunk from a PNG file.
    Remove(RemoveArgs),
    /// Prints all recognized chunks in a PNG file.
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    /// File path
    #[arg(short, long)]
    pub file_path: String,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: ChunkType,

    /// Message
    #[arg(short, long)]
    pub message: String,

    /// Output file path
    #[arg(short, long)]
    pub output_file: Option<String>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    #[arg(short, long)]
    pub file_path: String,
    #[arg(short, long)]
    pub chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    #[arg(short, long)]
    pub file_path: String,
    #[arg(short, long)]
    pub chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    #[arg(short, long)]
    pub file_path: String,
}
