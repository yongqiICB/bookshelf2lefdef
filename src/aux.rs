use std::{fmt, path::PathBuf};

use crate::io::reader::TokenReader;

#[derive(Default, Debug)]
pub struct Aux {
    pub me: Option<PathBuf>,
    pub nodes: Option<PathBuf>,
    pub nets: Option<PathBuf>,
    pub wts: Option<PathBuf>,
    pub pl: Option<PathBuf>,
    pub scl: Option<PathBuf>,
    pub shapes: Option<PathBuf>,
    pub route: Option<PathBuf>,
}

impl fmt::Display for Aux {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Aux: {:?}", self.me.as_ref().map(|x| x.display()))?;
        writeln!(
            f,
            "Node file: {:?}",
            self.nodes.as_ref().map(|x| x.display())
        )?;
        writeln!(f, "Net file: {:?}", self.nets.as_ref().map(|x| x.display()))?;
        writeln!(f, "Wts file: {:?}", self.wts.as_ref().map(|x| x.display()))?;
        writeln!(f, "Pl file: {:?}", self.pl.as_ref().map(|x| x.display()))?;
        writeln!(f, "Scl file: {:?}", self.scl.as_ref().map(|x| x.display()))?;
        writeln!(
            f,
            "Shape file: {:?}",
            self.shapes.as_ref().map(|x| x.display())
        )?;
        writeln!(
            f,
            "Route file: {:?}",
            self.route.as_ref().map(|x| x.display())
        )?;
        Ok(())
    }
}
impl Aux {
    pub async fn build(aux_path: &PathBuf) -> anyhow::Result<Self> {
        let mut res = Aux::default();
        res.me = Some(aux_path.clone());
        let mut reader = TokenReader::new_from_path(&aux_path);
        while let Some(token) = reader.next_token()? {
            if token.as_bytes() == b":" {
                break;
            }
        }
        while let Some(token) = reader.next_token()? {
            let raw_file_path = PathBuf::from(token);
            let file_path = if raw_file_path.is_relative() {
                res.me
                    .as_ref()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(raw_file_path)
            } else {
                raw_file_path
            };
            match file_path.extension() {
                Some(x) => match x.as_encoded_bytes() {
                    b"nets" => {
                        res.nets = Some(file_path);
                    }
                    b"wts" => {
                        res.wts = Some(file_path);
                    }
                    b"scl" => {
                        res.scl = Some(file_path);
                    }
                    b"pl" => {
                        res.pl = Some(file_path);
                    }
                    b"nodes" => res.nodes = Some(file_path),
                    b"route" => res.route = Some(file_path),
                    b"shapes" => res.shapes = Some(file_path),
                    _ => {
                        println!("[Error] Unknown extension format: {:?}, skip it.", x);
                    }
                },
                None => {
                    println!("[Error] not a path.");
                }
            }
        }
        Ok(res)
    }
}
