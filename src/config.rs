#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate serde;

use toml::Value;
use nya::{create_middleware, SimpleFile};

pub fn middleware() -> Box<FnMut(&mut Vec<SimpleFile>)> {
  create_middleware(|files: &mut Vec<SimpleFile>| {
    // do stuff here
  });
}