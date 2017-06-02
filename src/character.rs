// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use objects::{Renderable, Updatable};
use piston_window::{Context, G2d, UpdateArgs};
use image_ops::Animation;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ActionDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ActionName {
    Running,
    Idle,
    Attack,
    Death,
    Headshot,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Action {
    pub action: ActionName,
    pub direction: ActionDirection,
}

impl Action {
    pub fn new(action: ActionName, direction: ActionDirection) -> Action {
        Action {
            action,
            direction,
        }
    }
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
    fn render(&mut self, c: Context, g: &mut G2d) {
        if let Some(action) = self.cur_action {
            if let Some(anim) = self.animations.get_mut(&action) {
                anim.render(c, g);
            }
        }
    }
}

impl Updatable for Character {
    fn update(&mut self, upd: UpdateArgs) {
        let directions = [
            ActionDirection::N,
            ActionDirection::NE,
            ActionDirection::E,
            ActionDirection::SE,
            ActionDirection::S,
            ActionDirection::SW,
            ActionDirection::W,
            ActionDirection::NW
        ];

        if let Some(action) = self.cur_action {
            let mut action = action.clone();
            for d in directions.into_iter() {
                action.direction = *d;
                if let Some(anim) = self.animations.get_mut(&action) {
                    anim.update(upd);
                }
            }
        }
    }
}
