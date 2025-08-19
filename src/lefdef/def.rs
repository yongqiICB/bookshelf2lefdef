use std::{io::Write, path::PathBuf};

use crate::{
    lefdef::{components::Components, lef::Lef, net::Nets, pin::DefPins, row::Rows, tracks::Tracks},
    parser::Bookshelf,
};

pub struct Def {
    rows: Rows,
    pins: DefPins,
    nets: Nets,
    components: Components,
    tracks: Tracks,
}

impl Def {
    pub fn build(bookshelf: &Bookshelf, lef: &Lef) -> Self {
        let rows = Rows::build(bookshelf, format!("CoreSite"));
        let tracks = Tracks::build(bookshelf);
        let pins = DefPins::build(bookshelf);
        let nets = Nets::build_net(&lef.macros);
        let components = Components::build(bookshelf);
        Self { pins, nets, rows, components, tracks }
    }
    pub fn write_to_file(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        let to_write = format!("{}{}{}{}{}", self.rows.write(), self.tracks.write(), self.components.write(), self.pins.write(), self.nets.write());
        file.write_all(to_write.as_bytes())?;
        Ok(())
    }
}
