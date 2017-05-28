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

pub struct Character {
    scene: Scene<G2dTexture>,
}

impl Character {
    pub fn new<F: Factory<Resources>>(tex_file: PathBuf, factory: &mut F) -> Character {
        let mut p = Character {
            scene: Scene::new(),
        };
        let texture = Rc::new(Texture::from_path(
                factory,
                tex_file,
                Flip::None,
                &TextureSettings::new()
        ).unwrap());

        p.scene.add_child(Sprite::from_texture(texture));
        p
    }
}

impl Renderable for Character {
    fn render(&self, c: Context, g: &mut G2d) {
        self.scene.draw(c.transform, g);
    }
}

impl Updatable for Character {
    fn update(&mut self, upd: UpdateArgs) { }
}