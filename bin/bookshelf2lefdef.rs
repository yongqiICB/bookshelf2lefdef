use std::{fs::File, path::PathBuf, time::Instant};

use bookshelf2lefdef::{io::reader::TokenReader, nodes::{Node, Nodes}};
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
    let mut reader = TokenReader::new(std::io::BufReader::new(File::open(aux_path).unwrap()));
    let res = Nodes::read(&mut reader).await.unwrap();
    let end = Instant::now();
    println!("read {} nodes", res.nodes.len());
    println!("time: {}", (end-start).as_nanos());
}