// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use piston_window::{Ellipse, rectangle, Context, G2d, UpdateArgs, Transformed};
use nphysics2d::object::{RigidBodyHandle};
use super::{Renderable, Updatable, GameObject};

pub struct Ball {
    body: RigidBodyHandle<f32>,
    radius: f32,
}

impl Ball {
    pub fn new(radius: f32, body: RigidBodyHandle<f32>) -> Ball {
        Ball {
            body: body,
            radius: radius,
        }
    }

}

impl Renderable for Ball {
     fn render(&self, c: Context, g: &mut G2d) {
        let body = self.body.borrow();

        let pos = body.position().translation.vector.data;
        let rot = body.position().rotation.unwrap();
        let circle = Ellipse::new([0.906, 0.298, 0.235, 1.0]);
        let rectangle = rectangle::square(0.0, 0.0, self.radius as f64);
        circle.draw(
            rectangle,
            &c.draw_state,
            c.transform.trans(pos[0] as f64, pos[1] as f64)
                        .rot_rad(rot.re as f64)
                        .trans(
                            (-self.radius/2.0) as f64,
                            (-self.radius/2.0) as f64
                        ),
            g
        );
    }
}

impl Updatable for Ball {
    fn update(&mut self, upd: UpdateArgs) {}
}

impl GameObject for Ball {}
