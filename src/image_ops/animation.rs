// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use std::rc::Rc;
use piston_window::{image, G2dTexture, UpdateArgs, G2d, Context};
use piston_window::math::Matrix2d;
use objects::{Renderable, Updatable};
use std::mem;
use std::borrow::Borrow;

pub struct Animation {
    frames: Vec<Rc<G2dTexture>>,
    fps: f64,
    current_timer: f64,
    pub transform: Matrix2d,
}

//TODO: We should make an option run_once or run_times
impl Animation {
    pub fn new(fps: f64, frames: Vec<Rc<G2dTexture>>) -> Animation {
        Animation {
            fps,
            frames,
            transform: [[0.0; 3]; 2],
            current_timer: 0.0,
        }
    }

    /// Gets the current frame in the animation
    //TODO: At some point we should make a test for this
    pub fn current_frame(&self) -> usize {
        (self.fps * self.current_timer) as usize % self.frames.len()
    }

}

impl Renderable for Animation {
     fn render(&self, c: Context, g: &mut G2d) {
         let i = self.current_frame();
         let tex = self.frames[i].borrow();
         image(tex, self.transform, g)
     }
}

impl Updatable for Animation {
    fn update(&mut self, upd: UpdateArgs) {
        self.current_timer += upd.dt;
    }
}
