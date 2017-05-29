// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use sprite::{Scene, Sprite};
use texture::ImageSize;
use piston_window::{Flip, Texture, TextureSettings};
use std::rc::Rc;
use objects::{Renderable, Updatable};
use piston_window::{G2dTexture, Context, G2d, UpdateArgs};
use std::path::PathBuf;
use gfx::Factory;
use gfx_device_gl::Resources;
use image::{Rgba, Pixel, ImageBuffer, GenericImage};
use image;

error_chain! {
    foreign_links {
        TextureCreation(::gfx_core::factory::CombinedError);
    }
}

/// Produces textures from a tilemap texture
pub struct TileMap {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    tile_dimensions: [u32; 2],
}

impl TileMap {
    pub fn new(file: PathBuf, tile_dimensions: [u32; 2]) -> TileMap {
        let img = image::open(file).unwrap();

        TileMap {
            image: img.to_rgba(),
            tile_dimensions,
        }
    }

    pub fn texture<F: Factory<Resources>, I: Into<[u32; 2]>>(&mut self, index: I, factory: &mut F) -> Result<Rc<G2dTexture>> {
        let r = index.into();
        let x = r[0] * self.tile_dimensions[0];
        let y = r[1] * self.tile_dimensions[1];
        let w = self.tile_dimensions[0];
        let h = self.tile_dimensions[1];

        let new_img = self.image
            .sub_image(x,y,w,h)
            .to_image();

        let tx_res = Texture::from_image(
                factory,
                &new_img,
                &TextureSettings::new()
        );

        match tx_res {
            Ok(tx) => Ok(Rc::new(tx)),
            Err(err) => Err(ErrorKind::TextureCreation(err).into()),
        }
    }

    /// Creates a texture from the image giving absolute coordinates instead
    /// of indices
    pub fn texture_coord<F: Factory<Resources>, R: Into<[u32; 4]>>(&mut self, rect: R, factory: &mut F) -> Result<Rc<G2dTexture>> {
        let r = rect.into();
        let (x, y, w, h) = (r[0], r[1], r[2], r[3]);

        let new_img = self.image
            .sub_image(x,y,w,h)
            .to_image();

        let tx_res = Texture::from_image(
                factory,
                &new_img,
                &TextureSettings::new()
        );

        match tx_res {
            Ok(tx) => Ok(Rc::new(tx)),
            Err(err) => Err(ErrorKind::TextureCreation(err).into()),
        }
    }
}
