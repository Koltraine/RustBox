// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use objects::{EventHandler, Renderable, Updatable};
use piston_window::{Key, Input, Context, G2d, UpdateArgs, Button};
use character::{Action, ActionDirection, ActionName};
use character::Character;

pub struct Player {
    character: Character,
}

impl Player {
    pub fn new(character: Character) -> Player {
        Player {
            character
        }
    }
}

impl Renderable for Player {
    fn render(&self, c: Context, g: &mut G2d) {
        self.character.render(c, g);
    }
}

impl Updatable for Player {
    fn update(&mut self, upd: UpdateArgs) {
        self.character.update(upd);
     }
}

impl EventHandler for Player {
    fn event(&mut self, event: &Input) {
        let mut action = Action::new(ActionName::Running, ActionDirection::N);
        match *event {
            Input::Press(b) => {
                match b {
                    Button::Keyboard(k) => {
                        match k {
                            Key::W => {
                                action.direction = ActionDirection::N;
                            }
                            Key::A => {
                                action.direction = ActionDirection::W;
                            }
                            Key::S => {
                                action.direction = ActionDirection::S;
                            }
                            Key::D => {
                                action.direction = ActionDirection::E;
                            }
                            _ => {} // Catch all keys
                        };
                    }
                    _ => {} // Catch non-keyboard buttons
                };
                self.character.set_action(Some(action));
            }
            _ => {} // Catch unhandled events
        };
    }
}

