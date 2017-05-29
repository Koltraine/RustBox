// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use sprite::{Scene, Sprite};
use texture::ImageSize;
use piston_window::{Flip, Texture, TextureSettings};
use std::rc::Rc;
use objects::{Renderable, Updatable};
use piston_window::{G2dTexture, Context, G2d, UpdateArgs};
use image::ImageBuffer;
use std::path::PathBuf;
use gfx::Factory;
use gfx_device_gl::Resources;
use piston_window::{Input, clear, Transformed};
use image_ops::Animation;

pub struct Character {
    running: Animation,
}

//TODO: Should we implement graphics::Transformed?
impl Character {
    pub fn new(frames: Vec<Rc<G2dTexture>>) -> Character {
        let mut c = Character {
            running: Animation::new(8.0, frames),
        };
        c.running.transform = c.running.transform.trans(200.0, 200.0);
        c
    }
}

impl Renderable for Character {
    fn render(&self, c: Context, g: &mut G2d) {
        self.running.render(c, g);
    }
}

impl Updatable for Character {
    fn update(&mut self, upd: UpdateArgs) {
        self.running.update(upd);
    }
}
