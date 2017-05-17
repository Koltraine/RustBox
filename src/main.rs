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
extern crate find_folder;
extern crate rand;
extern crate itertools;

// Physics
extern crate ncollide;
extern crate nalgebra;
extern crate nphysics2d;

// Display engine
extern crate piston_window;
extern crate sprite;
extern crate texture;
extern crate gfx_device_gl; //TODO: Do we need this?
extern crate gfx_core;

mod objects;
mod map;
mod player;

use piston_window::*;
use nphysics2d::world::World;
use nalgebra::{TranslationBase, Vector2};
use nphysics2d::object::{RigidBody, RigidBodyHandle};
use ncollide::shape;
//use nalgebra::geometry::Similarity;

use objects::{Ball, Renderable, GameObject};
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

    let tex_dir = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("textures").unwrap();
    println!("{:?}", tex_dir);

    let texture = Rc::new(Texture::from_path(
        &mut window.factory,
        tex_dir.join("zombie").join("zombie_0.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap());
    let player = player::Player::new(texture.clone());

    while let Some(e) = window.next() {
        match e {
            Input::Render(r) => {
                window.draw_2d(&e, |c, g| {
                    game.render(c, g);
                    player.render(c, g);
                });
            }
            Input::Update(u) => {
                game.update(u);
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
