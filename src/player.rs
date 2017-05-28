// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use sprite::{Scene, Sprite};
use texture::ImageSize;
use piston_window::Texture;
use std::rc::Rc;
use objects::{Renderable, Updatable};
use piston_window::{Context, G2d, UpdateArgs};
use image::ImageBuffer;
use character::Character;
use gfx::{Resources, Factory};

pub struct Player {
    character: Character,
}

impl Player {
    pub fn new(character: Character) -> Player {
        Player {
            character
        }
    }
}

impl Renderable for Player {
    fn render(&self, c: Context, g: &mut G2d) {
        self.character.render(c, g);
    }
}

impl Updatable for Player {
    fn update(&mut self, upd: UpdateArgs) {
        self.character.update(upd);
     }
}
