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

impl Character {
    pub fn new(frames: Vec<Rc<G2dTexture>>) -> Character {
        let mut c = Character {
            running: Animation::new(4.0, frames),
        };
        c.running.transform = [[0.003125, 0.0, -0.375], [0.0, -0.004166666666666667, 0.16666666666666663]];
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
