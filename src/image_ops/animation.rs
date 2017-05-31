// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use std::rc::Rc;
use piston_window::{image, G2dTexture, UpdateArgs, G2d, Context};
use piston_window::math::Matrix2d;
use objects::{Renderable, Updatable};
use std::borrow::Borrow;

//TODO: Is this constant?
const INIT_TRANSFORM: Matrix2d = [
    [0.003125, 0.0, -1.0],
    [0.0, -0.004166666666666667, 1.0]
];

pub struct Animation {
    frames: Vec<Rc<G2dTexture>>,
    fps: f64,
    current_timer: f64,
    pub transform: Matrix2d,
    run_once: bool,
}

//TODO: We should make an option run_once or run_times
impl Animation {
    pub fn new(fps: f64, frames: Vec<Rc<G2dTexture>>) -> Animation {
        Animation {
            fps,
            frames,
            transform: INIT_TRANSFORM,
            current_timer: 0.0,
            run_once: false,
        }
    }

    /// Gets the current frame in the animation
    //TODO: At some point we should make a test for this
    pub fn current_frame(&self) -> usize {
        (self.fps * self.current_timer) as usize % self.frames.len()
    }

    /// Sets if the animation should run once or repeat in a loop
    pub fn run_once(mut self, ro: bool) -> Self {
        self.run_once = ro;
        self
    }

    /// Resume the animation if it has stopped due to `run_once`
    pub fn resume(&mut self) {
        self.current_timer = 0.0;
    }
}

impl Renderable for Animation {
     fn render(&self, _: Context, g: &mut G2d) {
         let i = self.current_frame();
         let tex = self.frames[i].borrow();
         image(tex, self.transform, g);
     }
}

impl Updatable for Animation {
    fn update(&mut self, upd: UpdateArgs) {
        if self.run_once && self.current_frame() == self.frames.len()-1 {
            return;
        }

        self.current_timer += upd.dt;
    }
}

