use std::path::PathBuf;

use bookshelf2lefdef::{aux::Aux, io::logger::init_logger, lefdef, parser};
use clap::Parser;
use log::info;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug)]
struct OutPaths {
    techlef: PathBuf,
    lef: PathBuf,
    def: PathBuf,
}

impl OutPaths {
    pub fn build(aux_path: &PathBuf) -> Self {
        let mut techlef = aux_path.clone();
        techlef.set_extension("tech.lef");

        let mut lef = aux_path.clone();
        lef.set_extension("lef");

        let mut def = aux_path.clone();
        def.set_extension("def");
        Self {
            techlef,
            lef,
            def,
        }
        
    }
}
#[tokio::main]
pub async fn main() {
    init_logger();
    let args = Args::parse();
    let aux_path = PathBuf::from(args.input);
    let aux = Aux::build(&aux_path).await.unwrap();
    let bookshelf = parser::Bookshelf::build_from_aux(aux).await.unwrap();
    let techlef = lefdef::techlef::TechLef::build(&bookshelf).await.unwrap();
    let out_paths = OutPaths::build(&aux_path);
    techlef.write_to_file(&out_paths.techlef).await;
    let lef = lefdef::lef::Lef::build(&bookshelf).await.unwrap();
    lef.write(&out_paths.lef).await.unwrap();
    let def = lefdef::def::Def::build(&bookshelf, &lef);
    def.write_to_file(&out_paths.def).unwrap();
    info!("Wrote output to: {:?}", out_paths);
}
