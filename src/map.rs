// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use piston_window::*;
use objects::{Renderable, Updatable, GameObject};
use tiled::parse;
use tiled;
use std::io::Read;
use rand;
use rand::Rng;

error_chain!{
    foreign_links {
        InputParsing(::tiled::TiledError);
    }
}

pub struct TiledMap {
    map: tiled::Map,
}

impl TiledMap {
    pub fn new<R: Read>(input: R) -> Result<TiledMap> {
        Ok(TiledMap {
            map: parse(input)?
        })
    }

}

impl Renderable for TiledMap {
    fn render(&self, c: Context, g: &mut G2d) {

    }
}

impl Updatable for TiledMap {
    fn update(&mut self, upd: UpdateArgs) {}
}
