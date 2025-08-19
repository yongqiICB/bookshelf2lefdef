use std::{io::Write, path::PathBuf};

use crate::{lefdef::writer::Macros, parser::Bookshelf};

pub struct Lef {
    pub macros: Macros,
}

impl Lef {
    pub async fn build(bookshelf: &Bookshelf) -> anyhow::Result<Self> {
        Ok(Self {
            macros: Macros::build_macro(bookshelf).await?,
        })
    }

    pub async fn write(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        let mut to_write = format!(
r#"VERSION 5.8 ;
BUSBITCHARS "[]" ;
DIVIDERCHAR "/" ;

SITE CoreSite 
    CLASS CORE ;
    SYMMETRY Y ;
    SIZE 1.000 1.000 ;
"#);
        to_write += &self.macros.write_all();
        let mut f = std::fs::File::create(file_path)?;
        f.write_all(to_write.as_bytes())?;
        Ok(())
    }
}
