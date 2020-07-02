use std::path::PathBuf;
use structopt::StructOpt;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, StructOpt)]
pub struct Encode {
    #[structopt(short, long)]
    file: PathBuf,
    #[structopt(short, long)]
    chunk_name: String,
    #[structopt(short, long)]
    message: String,
    #[structopt(short, long)]
    output_file: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct Decode {
    #[structopt(short, long)]
    file: PathBuf,
    #[structopt(short, long)]
    chunk_name: String,
}

#[derive(Debug, StructOpt)]
pub struct Remove {
    #[structopt(short, long)]
    file: PathBuf,
    #[structopt(short, long)]
    chunk_name: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme")]
pub enum Command {
    #[structopt(name = "encode")]
    Encode(Encode),
    #[structopt(name = "decode")]
    Decode(Decode),
    #[structopt(name = "remove")]
    Remove(Remove)
}

fn main() {
    let args = Command::from_args();
    match args {
        Command::Encode(arg) => commands::encode(arg),
        Command::Decode(arg) => commands::decode(arg),
        Command::Remove(arg) => println!("{:?}", arg),
    }
}





