// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use sprite::{Scene, Sprite};
use texture::ImageSize;
use piston_window::Texture;
use gfx_device_gl::Resources;
use std::rc::Rc;
use objects::{Renderable, Updatable};
use piston_window::{Context, G2d, UpdateArgs};
use image::ImageBuffer;


pub struct Character {
    scene: Scene<Texture<Resources>>,
}

impl Character {
    pub fn new() -> Character {
        let mut p = Character {
            scene: Scene::new(),
        };
        let texture = Rc::new(Texture::from_path(
                &mut window.factory,
                tex_dir.join("zombie").join("zombie_0.png"),
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
