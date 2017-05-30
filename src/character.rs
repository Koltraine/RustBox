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
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    Running,
}

pub struct Character {
    cur_action: Option<Action>,
    animations: HashMap<Action, Animation>,
}

//TODO: Should we implement graphics::Transformed?
impl Character {
    pub fn new() -> Character {
        Character {
            animations: HashMap::new(),
            cur_action: None,
        }
    }

    pub fn set_action(&mut self, action: Option<Action>) {
        self.cur_action = action;
    }

    pub fn set_animation(&mut self, action: Action, anim: Animation) {
        self.animations.insert(action, anim);
    }
}

impl Renderable for Character {
    fn render(&self, c: Context, g: &mut G2d) {
        if let Some(action) = self.cur_action {
            if let Some(anim) = self.animations.get(&action) {
                anim.render(c, g);
            }
        }
    }
}

impl Updatable for Character {
    fn update(&mut self, upd: UpdateArgs) {
        if let Some(action) = self.cur_action {
            if let Some(anim) = self.animations.get_mut(&action) {
                anim.update(upd);
            }
        }
    }
}
