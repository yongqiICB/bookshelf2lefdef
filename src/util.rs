use std::str::FromStr;

pub fn parse<R: FromStr>(x: &str) -> R {
    str::parse::<R>(x).ok().unwrap()
}