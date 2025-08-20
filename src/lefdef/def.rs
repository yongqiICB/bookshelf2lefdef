use std::{io::Write, path::PathBuf};

use crate::{
    lefdef::{components::Components, die_area::DieArea, lef::Lef, net::Nets, pin::DefPins, row::Rows, tracks::Tracks},
    parser::Bookshelf,
};

pub struct Def {
    rows: Rows,
    pins: DefPins,
    nets: Nets,
    components: Components,
    tracks: Tracks,
    die_area: DieArea,
}

impl Def {
    pub fn build(bookshelf: &Bookshelf, lef: &Lef) -> Self {
        let die_area = DieArea::build(bookshelf);
        let rows = Rows::build(bookshelf, format!("CoreSite"));
        let tracks = Tracks::build(bookshelf);
        let pins = DefPins::build(bookshelf);
        let nets = Nets::build_net(&lef.macros);
        let components = Components::build(bookshelf);
        Self { pins, nets, rows, components, tracks, die_area }
    }
    pub fn write_to_file(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        let to_write = format!(
            "VERSION 5.8 ;\
            \nDIVIDERCHAR \"/\" ;\
            \nBUSBITCHARS \"[]\" ;\
            \nDESIGN auto_generated ;\
            \nUNITS DISTANCE MICRONS 1000 ;\
            {}{}{}{}{}{}
            \nEND DESIGN
            ",self.die_area.write(), self.rows.write(), self.tracks.write(), self.components.write(), self.pins.write(), self.nets.write());
        file.write_all(to_write.as_bytes())?;
        Ok(())
    }
}
