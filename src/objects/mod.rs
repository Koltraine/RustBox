// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

mod ball;

pub use self::ball::Ball;
use piston_window::{Input, Context, G2d, UpdateArgs};

//TODO: We should move these out of there
pub trait Renderable {
    // Please don't use this mut self to update heavy logic
    fn render(&mut self, c: Context, g: &mut G2d);
}

pub trait Updatable {
    fn update(&mut self, upd: UpdateArgs);
}

pub trait EventHandler {
    fn event(&mut self, event: &Input);
}

pub trait GameObject: Updatable + Renderable {}
