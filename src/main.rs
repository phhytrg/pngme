use args::{Cli, EncodeArgs};
use clap::Parser;
use commands::{decode, encode, print_png, remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(command) => match command {
            args::PngMeArgs::Encode(args) => encode(
                &args.file_path,
                &args.chunk_type,
                &args.message,
                args.output_file.as_deref(),
            ),
            args::PngMeArgs::Decode(args) => decode(&args.file_path, &args.chunk_type),
            args::PngMeArgs::Remove(args) => remove(&args.file_path, &args.chunk_type),
            args::PngMeArgs::Print(args) => print_png(&args.file_path),
        },
        None => {}
    }
}
