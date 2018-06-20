#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate serde;

use toml::Value;
use nya::{create_middleware, SimpleFile, MiddlewareFunction};

pub fn middleware() -> MiddlewareFunction {
  create_middleware(|files: &mut Vec<SimpleFile>| {
    // do stuff here
  });
}