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

pub struct Character {
    scene: Scene<G2dTexture>,
}

impl Character {
    pub fn new(animation: Vec<Rc<G2dTexture>>) -> Character {
        let mut p = Character {
            scene: Scene::new(),
        };

        for t in animation {
            let mut sprite = Sprite::from_texture(t);
            sprite.set_position(200.0, 200.0);
            let id = p.scene.add_child(sprite);
        }
        p
    }
}

impl Renderable for Character {
    fn render(&self, c: Context, g: &mut G2d) {
        self.scene.draw(c.transform, g);
    }
}

impl Updatable for Character {
    fn update(&mut self, upd: UpdateArgs) {
        self.scene.event(&Input::Update(upd));
    }
}