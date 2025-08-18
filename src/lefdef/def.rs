use std::{io::Write, path::PathBuf};

use crate::{
    lefdef::{lef::Lef, net::Nets, pin::DefPins},
    parser::Bookshelf,
};

pub struct Def {
    pins: DefPins,
    nets: Nets,
}

impl Def {
    pub fn build(bookshelf: &Bookshelf, lef: &Lef) -> Self {
        let pins = DefPins::build(bookshelf);
        let nets = Nets::build_net(&lef.macros);
        Self { pins, nets }
    }
    pub fn write_to_file(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        let to_write = format!("{}{}", self.pins.write(), self.nets.write());
        file.write_all(to_write.as_bytes())?;
        Ok(())
    }
}
