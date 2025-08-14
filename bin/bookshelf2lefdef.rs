use std::{path::PathBuf, time::Instant};

use bookshelf2lefdef::{
    aux::Aux, io::logger::init_logger, parser
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
    init_logger();
    let start = Instant::now();
    let args = Args::parse();
    let aux_path = PathBuf::from(args.input);
    let aux = Aux::build(aux_path).await.unwrap();
    let bookshelf = parser::Bookshelf::build_from_aux(aux).await.unwrap();
    let mut end = Instant::now();
    println!("milestone, finished reading. time: {} ms", (end - start).as_millis());
    let _ = bookshelf.parse().await;
    end = Instant::now();
    println!("milestone, finished parsing. time: {} ms", (end - start).as_millis());
    
    
}
