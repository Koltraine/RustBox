// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// General use
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate enum_primitive;
extern crate find_folder;
extern crate rand;
extern crate itertools;
extern crate num;

// Physics
extern crate ncollide;
extern crate nalgebra;
extern crate nphysics2d;

// Display engine
extern crate piston_window;
extern crate sprite;
extern crate texture;
extern crate gfx;
//TODO: I would really like to make the code generic so that we dont need this
// This would probably involve droping piston_window
extern crate gfx_device_gl;
extern crate gfx_core;
extern crate image;

mod objects;
mod map;
mod player;
mod character;
mod image_ops;

use piston_window::*;
use nphysics2d::world::World;
use nalgebra::{TranslationBase, Vector2};
use nphysics2d::object::{RigidBody, RigidBodyHandle};
use ncollide::shape;

use character::{Action, Character, ActionDirection, ActionName};
use image_ops::{TileMap, Animation};
use player::Player;

use objects::{Ball, Renderable, Updatable, EventHandler, GameObject};
use std::fs::File;
use std::rc::Rc;


struct Game {
    world: World<f32>,
    timer: f32,
    ground_y: f32,
    // TODO: This box isn't very good, can we do better?
    objects: Vec<Box<GameObject>>,
}

impl Game {
    pub fn new() -> Game {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 1.00));

        Game {
            world: world,
            timer: 0.0f32,
            ground_y: 0.0f32,
            objects: vec![],
        }
    }

    pub fn update(&mut self, upd: UpdateArgs) {
        self.timer += upd.dt as f32;
        self.world.step(upd.dt as f32);
        for object in &mut self.objects {
            object.update(upd);
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        clear([0.525, 0.941, 0.945, 1.0], g);
        for object in &self.objects {
            object.render(c, g);
        }
    }

    pub fn keypress(&self, b: Button) {
        match b {
            Button::Keyboard(k) => {
                match k {
                    Key::W => { }
                    Key::S => { }
                    Key::F5 => { }
                    _ => {} // Catch all keys
                };
            }
            _ => {} // Catch non-keyboard buttons
        };
    }

}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "piston: hello_world",
            [640, 480]
        )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    let mut game = Game::new();

    let mut rb = RigidBody::new_dynamic(
        shape::Ball::new(100.0),
        1.0, // Density
        0.3, // Restitution
        0.6  // Friction
    );
    rb.append_translation(&TranslationBase::from_vector(Vector2::new(100.0, 0.0)));
    let handle = game.world.add_rigid_body(rb);
    {
        handle.borrow_mut().set_lin_vel(Vector2::new(2.0, 1.0));
    }
    game.objects.push(Box::new(Ball::new(100.0, handle)));

    //init_world(&mut game.world);
    //
    // TODO: we dont really need this
    // or at least lazy_static! it
    let fonts = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("fonts").unwrap();
    println!("{:?}", fonts);

    let ref font = fonts.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();


    //let map = map::TiledMap::new(9213);
    //game.objects.push(Box::new(map));


    let mut player = gen_player(window.factory.clone());

    while let Some(e) = window.next() {
        player.event(&e);
        match e {
            Input::Render(r) => {
                window.draw_2d(&e, |c, g| {
                    game.render(c, g);
                    player.render(c, g);
                });
            }
            Input::Update(u) => {
                game.update(u);
                player.update(u);
            }
            Input::Press(b) => {
                game.keypress(b);
            }
            _ => {} // Catch unhandled event
        }
    }
}

fn init_world(w: &mut World<f32>) {


    /*
     * First plane
     */
    let mut rb = RigidBody::new_static(
        shape::Plane::new(Vector2::new(0.0, 1.0)),
        0.3,
        0.6
    );
    rb.append_translation(&TranslationBase::from_vector(Vector2::new(0.0, 300.0)));
    w.add_rigid_body(rb);


}

fn gen_player(mut factory: gfx_device_gl::Factory) -> Player {
    let tex_dir = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("textures").unwrap();
    println!("{:?}", tex_dir);

    let mut character = Character::new();

    // These are the directions in the order that they
    // appear on the texture sheet
    let directions = [
        ActionDirection::W,
        ActionDirection::NW,
        ActionDirection::N,
        ActionDirection::NE,
        ActionDirection::E,
        ActionDirection::SE,
        ActionDirection::S,
        ActionDirection::SW,
    ];

    // These are the actions in the order that they
    // appear on the texture sheet
    let actions = [
        // Number of frames, FPS, Action
        ( 4, 8.0, ActionName::Idle),
        ( 8, 8.0, ActionName::Running),
        (10, 8.0, ActionName::Attack),
        ( 6, 8.0, ActionName::Death),
        ( 8, 8.0, ActionName::Headshot),
    ];

    let t = tex_dir.join("zombie").join("zombie_0.png");
    let tilemap = TileMap::new(t, [128, 128]);
    for d in 0..directions.len() {
        for a in 0..actions.len() {
            let mut frames = vec![];
            let dir = directions[d];
            let action = actions[a];
            let frame_count = action.0;
            let fps = action.1;
            let action_struct = Action::new(action.2, dir);

            let row = d;
            let start_column = actions.iter()
                                      .take(a)
                                      .fold(0, |z, i| z + i.0);
            let end_column = start_column + frame_count;

            for c in start_column..end_column {
                frames.push(tilemap.texture(
                        [c as u32, row as u32],
                        &mut factory
                ).unwrap());
            }

            character.set_animation(action_struct, Animation::new(fps, frames));
        }
    }

    character.set_action(Some(Action::new(ActionName::Death, ActionDirection::N)));
    Player::new(character)
}
