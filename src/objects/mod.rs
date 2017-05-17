// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

mod ball;

pub use self::ball::Ball;
use piston_window::{Context, G2d, UpdateArgs};

pub trait Renderable {
     fn render(&self, c: Context, g: &mut G2d);
}

pub trait Updatable {
    fn update(&mut self, upd: UpdateArgs);
}

pub trait GameObject: Updatable + Renderable {}
