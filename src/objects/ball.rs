// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use piston_window::{Ellipse, rectangle, Context, G2d, UpdateArgs, Transformed};
use nphysics2d::object::{RigidBodyHandle};
use super::{Renderable, Updatable, GameObject};

pub struct Ball {
    body: RigidBodyHandle<f64>,
    radius: f64,
}

impl Ball {
    pub fn new(radius: f64, body: RigidBodyHandle<f64>) -> Ball {
        Ball {
            body: body,
            radius: radius,
        }
    }

}

impl Renderable for Ball {
     fn render(&mut self, c: Context, g: &mut G2d) {
        let body = self.body.borrow();

        let pos = body.position().translation.vector.data;
        let rot = body.position().rotation.unwrap();
        let circle = Ellipse::new([0.906, 0.298, 0.235, 1.0]);
        let rectangle = rectangle::square(0.0, 0.0, self.radius as f64);
        circle.draw(
            rectangle,
            &c.draw_state,
            c.transform.trans(pos[0], pos[1])
                        .rot_rad(rot.re)
                        .trans(-self.radius / 2.0, -self.radius / 2.0),
            g
        );
    }
}

impl Updatable for Ball {
    fn update(&mut self, upd: UpdateArgs) {}
}

impl GameObject for Ball {}
