use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme")]
pub enum Command {
    #[structopt(name = "encode")]
    Encode {
        file: PathBuf,
        chunk_name: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    #[structopt(name = "decode")]
    Decode {
        file: PathBuf,
        chunk_name: String,
    },
    #[structopt(name = "remove")]
    Remove {
        file: PathBuf,
        chunk_name: String,
    },
    #[structopt(name = "print")]
    Print {
        file: PathBuf,
    }
}