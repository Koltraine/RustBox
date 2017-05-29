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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Action {
    Running,
}

pub struct Character {
    cur_action: Option<Action>,
    animations: Vec<Animation>,
}

//TODO: Should we implement graphics::Transformed?
impl Character {
    pub fn new() -> Character {
        Character {
            animations: vec![],
            cur_action: None,
        }
    }

    pub fn set_action(&mut self, action: Option<Action>) {
        self.cur_action = action;
    }

    pub fn set_animation(&mut self, action: Action, anim: Animation) {
        let id = action as usize;
        self.animations.insert(id, anim);
    }
}

impl Renderable for Character {
    fn render(&self, c: Context, g: &mut G2d) {
        if let Some(action) = self.cur_action {
            let id = action as usize;
            self.animations[id].render(c, g);
        }
    }
}

impl Updatable for Character {
    fn update(&mut self, upd: UpdateArgs) {
        if let Some(action) = self.cur_action {
            let id = action as usize;
            self.animations[id].update(upd);
        }
    }
}