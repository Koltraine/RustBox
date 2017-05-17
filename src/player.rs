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


pub struct Player {
    scene: Scene<Texture<Resources>>,
}

impl Player {
    pub fn new(texture: Rc<Texture<Resources>>) -> Player {
        let mut p = Player {
            scene: Scene::new(),
        };

        p.scene.add_child(Sprite::from_texture(texture));
        p
    }
}

impl Renderable for Player {
    fn render(&self, c: Context, g: &mut G2d) {
        self.scene.draw(c.transform, g);
    }
}

impl Updatable for Player {
    fn update(&mut self, upd: UpdateArgs) { }
}
