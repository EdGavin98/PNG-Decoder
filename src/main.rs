use structopt::StructOpt;
use crate::args::Command;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

fn main() {
    let args = Command::from_args();
    match args {
        Command::Encode(arg) => commands::encode(arg),
        Command::Decode(arg) => commands::decode(arg),
        Command::Remove(arg) => commands::delete(arg)
    }
}





