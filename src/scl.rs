use std::path::PathBuf;

use crate::io::reader::CommonReader;

#[derive(Default, Clone, Debug)]
pub enum RowOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default, Clone, Debug)]
pub enum SiteOrient {
    #[default]
    N,
    FS,
}

impl From<&str> for SiteOrient {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_bytes() {
            b"N" => Self::N,
            b"FS" => Self::FS,
            _ => panic!("found {} only support N or S in SiteOrient", value),
        }
    }
}

#[derive(Default, Clone,Debug)]
pub enum SiteSymmetry {
    X,
    #[default]
    Y,
}

impl From<&str> for SiteSymmetry {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_bytes() {
            b"X" => Self::X,
            b"Y" => Self::Y,
            _ => panic!("found {} only support N or S in SiteOrient", value),
        }
    }
}
#[derive(Default, Clone, Debug)]
pub struct Row {
    pub coordinate: i64,
    pub height: i64,
    pub site_width: i64,
    pub site_spacing: i64,
    pub site_orient: SiteOrient,
    pub site_symmetry: SiteSymmetry,
    pub subrow_origin: i64,
    pub num_sites: i64,
    pub orientation: RowOrientation,
}

impl Row {
    pub async fn read(reader: &mut CommonReader) -> anyhow::Result<Self> {
        let mut res = Self::default();
        while let Some(token) = reader.next_token()? {
            match token.to_ascii_uppercase().as_bytes() {
                b"END" => {
                    break;
                }
                b"COREROW" => {
                    continue;
                }
                b"HORIZONTAL" => {
                    res.orientation = RowOrientation::Horizontal;
                }
                b"VERTICAL" => {
                    res.orientation = RowOrientation::Vertical;
                }
                b"COORDINATE" => {
                    assert_eq!(b":", reader.next_token().unwrap().unwrap().as_bytes());
                    res.coordinate = str::parse::<i64>(reader.next_token()?.unwrap())?;
                }
                b"HEIGHT" => {
                    assert_eq!(b":", reader.next_token().unwrap().unwrap().as_bytes());
                    res.height = str::parse::<i64>(reader.next_token()?.unwrap())?;
                }
                b"SITEWIDTH" => {
                    assert_eq!(b":", reader.next_token().unwrap().unwrap().as_bytes());
                    res.site_width = str::parse::<i64>(reader.next_token()?.unwrap())?;
                }
                b"SITESPACING" => {
                    assert_eq!(b":", reader.next_token().unwrap().unwrap().as_bytes());
                    res.site_spacing = str::parse::<i64>(reader.next_token()?.unwrap())?;
                }
                b"SITEORIENT" => {
                    assert_eq!(b":", reader.next_token().unwrap().unwrap().as_bytes());
                    res.site_orient = reader.next_token()?.unwrap().into();
                }
                b"SITESYMMETRY" => {
                    assert_eq!(b":", reader.next_token().unwrap().unwrap().as_bytes());
                    res.site_symmetry = reader.next_token()?.unwrap().into();
                }
                b"SUBROWORIGIN" => {
                    assert_eq!(b":", reader.next_token().unwrap().unwrap().as_bytes());
                    res.subrow_origin = str::parse::<i64>(reader.next_token()?.unwrap())?;
                }
                b"NUMSITES" => {
                    assert_eq!(b":", reader.next_token().unwrap().unwrap().as_bytes());
                    res.num_sites = str::parse::<i64>(reader.next_token()?.unwrap())?;
                }
                _ => {
                    println!("Unexpected token");
                }
            }
        }
        Ok(res)
    }
}

#[derive(Default)]
pub struct Scl {
    rows: Vec<Row>,
}

impl Scl {
    pub fn iter(&self) -> std::slice::Iter<'_, Row> {
        self.rows.iter()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub async fn read_from_file(scl_path: &PathBuf) -> anyhow::Result<Self> {
        let reader = &mut CommonReader::new_from_path(scl_path);
        let mut res = Scl::default();
        while let Some(token) = reader.peek_token()? {
            match token.to_ascii_uppercase().as_bytes() {
                b"UCLA" | b"#" | b"NUMROWS" => {
                    reader.swallow_line()?;
                }
                b"COREROW" => {
                    res.rows.push(Row::read(reader).await?);
                }
                _ => {
                    panic!("Unexpected token: {}", token);
                }
            }
        }
        Ok(res)
    }
}
