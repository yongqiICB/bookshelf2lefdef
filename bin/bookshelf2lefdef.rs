use std::{path::PathBuf, time::Instant};

use bookshelf2lefdef::{
    aux::Aux,
    parser,
};
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

#[tokio::main]
pub async fn main() {
    let start = Instant::now();
    let args = Args::parse();
    let aux_path = PathBuf::from(args.input);
    let aux = Aux::build(aux_path).await.unwrap();
    let _ = parser::Bookshelf::build_from_aux(aux).await.unwrap();
    let end = Instant::now();
    println!("time: {} ms", (end - start).as_millis());
}
