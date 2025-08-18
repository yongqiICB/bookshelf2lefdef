use std::{io::Write, path::PathBuf};

use crate::{
    lefdef::{components::Components, lef::Lef, net::Nets, pin::DefPins, row::Rows},
    parser::Bookshelf,
};

pub struct Def {
    rows: Rows,
    pins: DefPins,
    nets: Nets,
    components: Components
}

impl Def {
    pub fn build(bookshelf: &Bookshelf, lef: &Lef) -> Self {
        let pins = DefPins::build(bookshelf);
        let nets = Nets::build_net(&lef.macros);
        let rows = Rows::build(bookshelf, format!("CoreSite"));
        let components = Components::build(bookshelf);
        Self { pins, nets, rows, components }
    }
    pub fn write_to_file(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        let to_write = format!("{}{}{}{}", self.rows.write(), self.components.write(), self.pins.write(), self.nets.write());
        file.write_all(to_write.as_bytes())?;
        Ok(())
    }
}
