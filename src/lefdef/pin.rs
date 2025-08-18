use std::collections::{BTreeMap, BTreeSet};

use log::info;

use crate::{geom::Point, parser::Bookshelf};

#[derive(Default)]
pub struct DefPin {
    pub name: String,              // aux.route
    pub layer: String,             // aux.route
    pub orientation: String,       // aux.pl
    pub place: Point,              // aux.pl
    pub shape: Point,              // aux.node
    pub net: Option<String>,       // aux.net
    pub direction: Option<String>, // aux.net
}

impl DefPin {
    fn write_to_string(&self) -> String {
        match (&self.net, &self.direction) {
            (Some(net), Some(direction)) => {
                format!(
                    "\
                \n- {}\
                \n    + NET {}\
                \n    + DIRECTION {}\
                \n    + USE SIGNAL\
                \n    + PORT\
                \n        + LAYER {} ( 0 0 ) ( {} {} )\
                \n        + FIXED ( {} {} ) N ;",
                    self.name,
                    net,
                    direction,
                    self.layer,
                    (self.shape.x * 1000.0) as i64,
                    (self.shape.y * 1000.0) as i64,
                    (self.place.x * 1000.0) as i64,
                    (self.place.y * 1000.0) as i64
                )
            }
            _ => {
                format!(
                    "\
                \n - {}\
                \n + USE SIGNAL\
                \n + PORT\
                \n   + LAYER {} ( 0 0 ) ( {} {} )\
                \n   + FIXED ( {} {} ) N ;",
                    self.name,
                    self.layer,
                    (self.shape.x * 1000.0) as i64,
                    (self.shape.y * 1000.0) as i64,
                    (self.place.x * 1000.0) as i64,
                    (self.place.y * 1000.0) as i64
                )
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct PinValidator {
    pub pin_in_route: BTreeSet<String>,
    pub pin_in_pl: BTreeSet<String>,
    pub pin_in_net: BTreeMap<String, i64>,
}

/// bookshelf 使用 Terminal_NI 之规定
/// “Fixed pins reside on a metal layer above M1
/// used within standard-cells for pins/internal routing.“
///
impl PinValidator {
    fn build(bookshelf: &Bookshelf) -> Self {
        let by_route: BTreeSet<String> = bookshelf
            .route
            .ni_terminal_to_layer
            .keys()
            .cloned()
            .collect();
        let by_pl: BTreeSet<String> = bookshelf
            .pls
            .iter()
            .filter(|x| matches!(x.r#type, crate::pl::Type::FixedNotInImage))
            .map(|x| x.name.clone())
            .collect();
        let mut by_net: BTreeMap<String, i64> = BTreeMap::new();
        for net in bookshelf.nets.iter() {
            for net_pin in net.pin.iter() {
                if let Some(instance_name) = by_route.get(&net_pin.instance_name) {
                    let x = by_net.entry(instance_name.clone()).or_insert(0);
                    *x += 1;
                    if *x >= 2 {
                        panic!("top_level_pin only has one port, however it has more than one");
                    }
                }
            }
        }
        for (pin_name, count) in by_net.iter().filter(|(_, count)| **count > 1) {
            info!("pin {} has {} port, which is illegal.", pin_name, count);
        }
        Self {
            pin_in_route: by_route,
            pin_in_pl: by_pl,
            pin_in_net: by_net,
        }
    }
    fn is_valid(&self) {
        assert_eq!(self.pin_in_route.len(), self.pin_in_pl.len());
        for name in self.pin_in_route.iter() {
            assert!(self.pin_in_pl.get(name).is_some());
        }
        assert!(self.pin_in_pl.len() >= self.pin_in_net.len());
        for (pin_name, port_cnt) in self.pin_in_net.iter() {
            if *port_cnt >= 2 {
                panic!(
                    "pin {} has more than one port. I think this is not valid",
                    pin_name
                );
            }
        }
        info!("Passed pin validity test");
    }
}
pub struct DefPins(BTreeMap<String, DefPin>);

impl DefPins {
    pub fn get(&self, name: &str) -> Option<&DefPin> {
        self.0.get(name)
    }

    pub fn is_defpin(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn write(&self) -> String {
        let mut res = format!("\nPINS {}", self.0.len());
        for def_pin in self.0.values() {
            res += &def_pin.write_to_string();
        }
        res += "\nEND PINS";
        res
    }
    pub fn build(bookshelf: &Bookshelf) -> Self {
        PinValidator::build(bookshelf).is_valid();
        let mut res = BTreeMap::new();
        let cnt = bookshelf.route.ni_terminal_len();
        for (name, layer_id) in bookshelf.route.ni_terminal_to_layer.iter() {
            res.insert(
                name.clone(),
                DefPin {
                    name: name.clone(),
                    layer: format!("metal{}", *layer_id),
                    ..Default::default()
                },
            );
        }
        let mut cnt_pl = 0;
        for pl in bookshelf
            .pls
            .iter()
            .filter(|x| matches!(x.r#type, crate::pl::Type::FixedNotInImage))
        {
            cnt_pl += 1;
            let pin = res.get_mut(&pl.name).unwrap();
            pin.place = pl.place;
            pin.orientation = pl.orientation.clone();
        }

        let mut cnt_node = 0;
        for node in bookshelf
            .nodes
            .iter()
            .filter(|x| matches!(x.moveable, crate::nodes::Movable::FixedButOverlapAllowed))
        {
            cnt_node += 1;
            let pin = res.get_mut(&node.name).unwrap();
            pin.shape = node.size;
        }
        for net in bookshelf.nets.iter() {
            for net_pin in net.pin.iter() {
                if let Some(pin) = res.get_mut(&net_pin.instance_name) {
                    pin.net = Some(net.name.clone());
                    pin.direction = Some(match net_pin.pin_name.as_bytes() {
                        b"I" => "INPUT".to_string(),
                        b"O" => "OUTPUT".to_string(),
                        _ => panic!(
                            "Terminal_NI considered as PIN which usually have only one direction, INPUT or OUTPUT"
                        ),
                    });
                }
            }
        }
        assert_eq!(cnt, cnt_pl);
        assert_eq!(cnt, cnt_node);
        Self(res)
    }
}
