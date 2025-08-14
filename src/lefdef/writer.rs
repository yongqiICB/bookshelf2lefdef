use std::collections::BTreeMap;

use log::{info, warn};

use crate::{geom::Point, parser::Bookshelf};


/// Pin has no size in ISPD 11.
/// We give it a minimum size to make it useful.
#[derive(Debug, Default)]
pub struct Pin {
    pub name: String,
    pub offset: Point,
}


#[derive(Debug, Default)]
pub struct Macro {
    pub name: String,
    pub size: Point,
    pub pins: Vec<Pin>,
}

#[derive(Debug, Default)]
pub struct Macros(pub BTreeMap<String, Macro>);

impl Macros {
    pub async fn build_macro(bookshelf: &Bookshelf) -> anyhow::Result<Self> {
        warn!(
            "Notification for MACRO!!!\
            \n  Usually, bookshelf does not provide enough information for a macro. I filled it freely.\
            \n  list to say:\
            \n  * SYMMETRY is set to X and Y by default.\
            \n  * SITE is set to core by default.\
            \n  * PIN NAME is set freely with a random suffix.\
            \n  * PORT is set to layer1 for standard cells, as ISPD official required.\
            \n  * PORT SHAPE are 1x1 as we do not check up `bookshelf.masterpin`, i am tired of writing this code.\
            \n  * PORT DIRECTION is specified according to `bookshelf.net` file."
        );

        let mut res = Self::default();
        info!("Building macros...");
        bookshelf.nodes.nodes.iter().for_each(|node| {
            res.0.entry(node.name.clone()).or_insert(Macro { 
                name: node.name.clone(), 
                size: node.size.clone(), 
                pins: vec![] 
            });
        });
        bookshelf.nets.iter().for_each(|net| {
            net.pin.iter().for_each(|pin| {
                res.0.entry(pin.instance_name.clone()).and_modify(|r#macro| {
                    r#macro.pins.push(Pin { 
                        name: pin.pin_name.clone(), 
                        offset:  pin.offset.clone()
                    });
                });
            });
        });
        info!("Finished building macros");        
        Ok(res)
    }
}

#[derive(Default)]
enum LayerType {
    #[default]
    Routing,
    Cut,
}

#[derive(Default)]
enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default)]
pub struct Layer {
    layer_type: LayerType,
    direction: Direction,
    pitch: f64,
    offset: f64,
    width: f64,
    spacing: f64,
}


impl Layer {
    pub async fn build_layer(bookshelf: &Bookshelf) -> anyhow::Result<Vec<Self>> {
        let mut res = vec![];
        let aux_layer = &bookshelf.route;
        let num_layer = aux_layer.vertical_capacity.len();
        
        info!("Layer information: \
            \n  可布导线在 tile 的中线上",
        );
        for layer_id in 0..num_layer {
            let vertical_cap = bookshelf.route.vertical_capacity[layer_id];
            let horizontal_cap = bookshelf.route.horizontal_capacity[layer_id];
            let min_wire_width = bookshelf.route.min_wire_width[layer_id];
            let min_wire_spacing = bookshelf.route.min_wire_spacing[layer_id];
            let pitch = min_wire_width + min_wire_spacing;
            let direction = match (vertical_cap > 0, horizontal_cap > 0) {
                (false, false) => Direction::Horizontal, // M1, usually horizontal.
                (false, true) => Direction::Horizontal, // horizontal
                (true, false) => Direction::Vertical, // vertical
                (true, true) => {
                    panic!("I don't know how to parse a bidirectional lef");
                }
            };
            res.push(Self {
                offset: pitch as f64 / 2.0,
                layer_type: LayerType::Routing,
                direction: direction,
                pitch: pitch as f64,
                width: min_wire_width as f64,
                spacing: min_wire_spacing as f64,
            })
        }
        
        Ok(res)
    }
}