use std::{io::Write, path::PathBuf};

use crate::{
    lefdef::writer::{CutLayer, OverlapLayer, RoutingLayer},
    parser::Bookshelf,
};

pub struct TechLef {
    pub layer: Vec<RoutingLayer>,
}

impl TechLef {
    pub async fn build(bookshelf: &Bookshelf) -> anyhow::Result<Self> {
        let routing_layers = RoutingLayer::build_routing_layers(bookshelf).await?;
        Ok(Self {
            layer: routing_layers,
        })
    }

    pub async fn write_to_file(&self, file_path: &PathBuf) {
        let mut file = std::fs::File::create(file_path).unwrap();
        let mut res = format!(
            "VERSION 5.8 ;\
            \nBUSBITCHARS \"[]\" ;\
            \nDIVIDERCHAR \"/\" ;\
            \nUNITS\
            \n  DATABASE MICRONS 1000 ;\
            \nEND UNITS\
            \nMANUFACTURINGGRID 0.005 ;\
            {}", OverlapLayer::format_a_default_one());
        for (id, layer) in self.layer.iter().enumerate() {
            res += &format!("{}", layer.format());
            if id != self.layer.len() - 1 {
                res += &format!(
                    "{}",
                    CutLayer::format_a_default_one(format!("CUT{}", id + 1))
                );
            }
        }
        for (id, _) in self.layer.iter().skip(1).enumerate() {
            let id = id + 1;
            res += &format!(
                "\nVIA V{} DEFAULT\
                \n    LAYER metal{} ;\
                \n        RECT -0.500 -0.500 0.500 0.500 ;\
                \n    LAYER CUT{} ;\
                \n        RECT -0.500 -0.500 0.500 0.500 ;\
                \n    LAYER metal{} ;\
                \n        RECT -0.500 -0.500 0.500 0.500 ;\
                \nEND V{}", 
                id, id, id, id + 1, id);
        }
        res += &format!("\nEND LIBRARY");
        file.write_all(res.as_bytes()).unwrap();
    }
}
