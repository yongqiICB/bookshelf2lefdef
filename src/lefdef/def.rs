use std::{io::Write, path::PathBuf};

use crate::{lefdef::pin::DefPins, parser::Bookshelf};

pub struct Def {
    pins: DefPins,
}

impl Def {
    pub fn build(bookshelf: &Bookshelf) -> Self {
        let pins = DefPins::build(bookshelf);
        Self {
            pins
        }
    }
    pub fn write_to_file(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        let to_write = self.pins.write();
        file.write_all(to_write.as_bytes())?;
        Ok(())
    }
}