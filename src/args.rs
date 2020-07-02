use structopt::StructOpt;
use std::path::PathBuf;

// #[derive(StructOpt, Debug)]
// #[structopt(name = "pngme")]
// pub enum Command {
//     #[structopt(name = "encode")]
//     Encode {
//         #[structopt(short, long)]
//         file: PathBuf,
//         #[structopt(short, long)]
//         chunk_name: String,
//         #[structopt(short, long)]
//         message: String,
//         #[structopt(short, long)]
//         output_file: Option<PathBuf>,
//     },
//     #[structopt(name = "decode")]
//     Decode {
//         #[structopt(short, long)]
//         file: PathBuf,
//         #[structopt(short, long)]
//         chunk_name: String,
//     },
//     #[structopt(name = "remove")]
//     Remove {
//         #[structopt(short, long)]
//         file: PathBuf,
//         #[structopt(short, long)]
//         chunk_name: String,
//     },
//     #[structopt(name = "print")]
//     Print {
//         #[structopt(short, long)]
//         file: PathBuf,
//     }
// }