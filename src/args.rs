use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Encode {
    #[structopt(short, long)]
    pub file: PathBuf,
    #[structopt(short, long)]
    pub chunk_name: String,
    #[structopt(short, long)]
    pub message: String,
    #[structopt(short, long)]
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct Decode {
    #[structopt(short, long)]
    pub file: PathBuf,
    #[structopt(short, long)]
    pub chunk_name: String,
}

#[derive(Debug, StructOpt)]
pub struct Remove {
    #[structopt(short, long)]
    pub file: PathBuf,
    #[structopt(short, long)]
    pub chunk_name: String,
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
