// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use std::rc::Rc;
use piston_window::rectangle::square;
use piston_window::{DrawState, Image, G2d, Context, G2dTexture};
use objects::Renderable;
use texture::ImageSize;

use std::borrow::Borrow;

error_chain! {
    foreign_links {
        TextureCreation(::gfx_core::factory::CombinedError);
    }
}

/// A TextureAtlas can be used to efficiently draw sub textures
/// from a large texture sheet
pub struct TextureAtlas {
    texture: Rc<G2dTexture>,

    pub draw_state: DrawState,

    image: Image,
}

impl TextureAtlas {
    pub fn new(texture: Rc<G2dTexture>) -> TextureAtlas {
        let image = {
            let tx: &G2dTexture = texture.borrow();
            Image::new().rect(
                square(0.0, 0.0, tx.get_size().0 as f64)
            )
        };

        TextureAtlas {
            texture,
            draw_state: DrawState {
                scissor: None,
                stencil: None,
                blend: None,
            },
            image
        }
    }

    pub fn set_coords(&mut self, scissor: Option<[u32; 4]>) {
        self.draw_state.scissor = scissor;
    }
}

impl Renderable for TextureAtlas {
    fn render(&mut self, c: Context, g: &mut G2d) {
        let image = Image::new().rect(square(0.0, 0.0, 1000.0));
        let draw_state = DrawState {
            scissor: Some([50, 100, 16, 16]),
            stencil: None,
            blend: None,
        };
        image.draw(
            self.texture.borrow(),
            &self.draw_state,
            c.transform,
            g
        );
    }
}
