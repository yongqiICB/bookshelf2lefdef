use std::collections::BTreeMap;

use log::{info, warn};

use crate::{geom::Point, lefdef::net::Node, parser::Bookshelf};

/// Pin has no size in ISPD 11.
/// We give it a minimum size to make it useful.
#[derive(Debug, Default)]
pub struct Pin {
    pub name: String,
    pub offset: Point,
    pub direction: String,
}

#[derive(Debug, Default)]
pub struct Macro {
    pub name: String,
    pub size: Point,
    pub pins: Vec<Pin>,
}

impl Macro {
    pub fn format_to_lef(&self) -> String {
        let mut res = format!(
            "\nMACRO {}\
            \n  CLASS CORE ;\
            \n  ORIGIN 0 0 ;\
            \n  SIZE {} {} ;\
            \n  SYMMETRY X Y ;\
            \n  SITE coresite ;",
            self.name, self.size.x, self.size.y,
        );
        let center = Point {
            x: self.size.x / 2.0,
            y: self.size.y / 2.0,
        };
        for (pin_cnt, pin) in self.pins.iter().enumerate() {
            res += &format!(
                "\
                \n  PIN {}_{}\
                \n      DIRECTION {} ;\
                \n      USE SIGNAL ; \
                \n      PORT\
                \n          LAYER metal1 ; \
                \n              RECT {} {} {} {} ;\
                \n      END\
                \n  END {}_{}",
                pin.name,
                pin_cnt,
                pin.direction,
                pin.offset.x + center.x - 0.5,
                pin.offset.y + center.y - 0.5,
                pin.offset.x + center.x + 0.5,
                pin.offset.y + center.y + 0.5,
                pin.name,
                pin_cnt
            );
        }
        res += &format!("\n END {}", self.name);
        res
    }
}
#[derive(Debug, Default)]
pub struct Macros {
    pub macros: BTreeMap<String, Macro>,
    pub net_to_nodes: BTreeMap<String, Vec<Node>>, // aux net name to pins
}

impl Macros {
    pub fn write_all(&self) -> String {
        let mut res = String::new();
        for r#macro in self.macros.values() {
            res += &r#macro.format_to_lef();
        }
        res
    }
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
        bookshelf.nodes.nodes.iter().for_each(|(_, node)| {
            res.macros.entry(node.name.clone()).or_insert(Macro {
                name: node.name.clone(),
                size: node.size,
                pins: vec![],
            });
        });
        bookshelf.nets.iter().for_each(|net| {
            net.pin.iter().for_each(|pin| {
                let r#macro = res.macros.get_mut(&pin.instance_name).unwrap();
                let pin_id = r#macro.pins.len();
                let pin_name = format!("{}_{}", pin.pin_name, pin_id);
                r#macro.pins.push(Pin {
                    name: pin_name.clone(),
                    offset: pin.offset,
                    direction: match pin.pin_name.as_str() {
                        "I" => "INPUT".to_string(),
                        "O" => "OUTPUT".to_string(),
                        _ => panic!("Unable to translate direction"),
                    },
                });
                let nodes_in_net = res.net_to_nodes.entry(net.name.clone()).or_insert(vec![]);

                if bookshelf.nodes.is_terminal_ni(&pin.instance_name) {
                    nodes_in_net.push(Node::Pin(pin.instance_name.clone()));
                } else {
                    nodes_in_net.push(Node::InstancePin(pin.instance_name.clone(), pin_name));
                }
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
}

#[derive(Default)]
enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Horizontal => write!(f, "HORIZONTAL"),
            Direction::Vertical => write!(f, "VERTICAL"),
        }
    }
}

#[derive(Default)]
pub struct RoutingLayer {
    name: String,
    direction: Direction,
    pitch: f64,
    offset: f64,
    width: f64,
    spacing: f64,
}

#[derive(Default)]
pub struct CutLayer;
impl CutLayer {
    pub fn format_a_default_one(name: String) -> String {
        let mut res = String::new();
        res += &format!(
            "\n\
            \nLAYER {}\
            \n  TYPE CUT ;\
            \n  SPACING 0.05 ;\
            \n  WIDTH 0.05 ;\
            \nEND {}",
            name, name
        );
        res
    }
}

#[derive(Default)]
pub struct OverlapLayer;
impl OverlapLayer {
    pub fn format_a_default_one() -> String {
        let mut res = String::new();
        res += "\n\
            \nLAYER OVERLAP\
            \n  TYPE OVERLAP ;\
            \n END OVERLAP";
        res
    }
}

impl RoutingLayer {
    pub async fn build_routing_layers(bookshelf: &Bookshelf) -> anyhow::Result<Vec<Self>> {
        let mut res = vec![];
        let aux_layer = &bookshelf.route;
        let num_layer = aux_layer.vertical_capacity.len();
        for layer_id in 0..num_layer {
            let layer_name = format!("metal{}", layer_id + 1);
            let vertical_cap = bookshelf.route.vertical_capacity[layer_id];
            let horizontal_cap = bookshelf.route.horizontal_capacity[layer_id];
            let direction = match (vertical_cap > 0, horizontal_cap > 0) {
                (false, false) => Direction::Horizontal, // M1, usually horizontal.
                (false, true) => Direction::Horizontal,  // horizontal
                (true, false) => Direction::Vertical,    // vertical
                (true, true) => {
                    panic!("I don't know how to parse a bidirectional lef");
                }
            };
            let min_wire_width = bookshelf.route.min_wire_width[layer_id];
            let min_wire_spacing = bookshelf.route.min_wire_spacing[layer_id];
            let pitch = min_wire_width + min_wire_spacing;
            {
                // LEFDEF does not support partial routing blockage.
                let cap_restriction = vertical_cap.max(horizontal_cap);
                let tile_len = match direction {
                    Direction::Horizontal => bookshelf.route.tile_size.y,
                    Direction::Vertical => bookshelf.route.tile_size.x,
                };
                // we must prove (tile_len / (min_width + spacing)) > cap_restriction.
                if (cap_restriction as f64) < (tile_len / pitch as f64) {
                    warn!(
                        "we OVERLOOK capacity restriction. on layer {}\
                        \n  this restrication can not be translated into LEFDEF soundly.
                        \n  if WIDTH and SPACING already satisfied, nothing happens.
                        \n  otherwise you will see this note.",
                        layer_id + 1
                    );
                }
            }
            res.push(Self {
                name: layer_name,
                offset: pitch as f64 / 2.0,
                direction,
                pitch: pitch as f64,
                width: min_wire_width as f64,
                spacing: min_wire_spacing as f64,
            })
        }
        Ok(res)
    }
    pub fn format(&self) -> String {
        let mut res = String::new();
        res += &format!(
            "\nLAYER {}\
            \n  TYPE ROUTING ;\
            \n  DIRECTION {} ;\
            \n  WIDTH {} ;\
            \n  SPACING {} ;\
            \n  PITCH {} ;\
            \n  OFFSET {} ;\
            \nEND {}",
            self.name, self.direction, self.width, self.spacing, self.pitch, self.offset, self.name,
        );
        res
    }
}
