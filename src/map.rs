// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use piston_window::*;
use objects::{Renderable, Updatable};
use tiled::{parse, Layer};
use tiled;
use std::path::Path;
use image_ops::TileBuffer;
use std::collections::HashMap;
use gfx::{self, Factory, Encoder};
use gfx::traits::FactoryExt;
use gfx_device_gl::{self, Resources, CommandBuffer};
use std::fs::File;
use std::rc::Rc;
use piston_window::image;
use std::borrow::Borrow;
use image::{Rgba, ImageBuffer};
use gfx_core::texture::NewImageInfo;
use utils::TextureAtlas;

error_chain!{
    foreign_links {
        InputParsing(::tiled::TiledError);
        TextureCreation(::gfx_core::factory::CombinedError);
    }
}

// TODO: Check how fast the render time for this is
// Maybe we could cache the final map,
// so render the whole thing into a final buffer
// and render that as an image
// we then only have to re-render it if the camera moves
pub struct TiledMap {
    map: tiled::Map,

    texture_atlas: TextureAtlas,
}

impl TiledMap {
    pub fn new<F>(path: &Path, factory: &mut F) -> Result<TiledMap>
        where F: Factory<Resources> {

        let map = parse(File::open(path).unwrap())?;
        let tex_dir = {
            let mut p = path.to_path_buf();
            p.pop();
            p.push("dungeon_tiles.png");
            p
        };
        let texture_atlas = TextureAtlas::new(
            Rc::new(Texture::from_path(
                factory,
                tex_dir,
                Flip::None,
                &TextureSettings::new()
            )?)
        );

        Ok(TiledMap {
            map,
            texture_atlas
        })
    }

    fn split_image(tb: TileBuffer) -> Vec<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>> {
        let mut col_vec = vec![];
        for i in 0..tb.tiles_available()[0] {
            let mut row_vec = vec![];

            for j in 0..tb.tiles_available()[1] {
                row_vec.push(tb.image([i,j]));
            }

            col_vec.push(row_vec);
        }
        col_vec
    }
}

impl Renderable for TiledMap {
    fn render(&mut self, c: Context, g: &mut G2d) {
        //self.encoder.clear(&self.data.out_color, [CLEAR_COLOR.0, CLEAR_COLOR.1, CLEAR_COLOR.2, CLEAR_COLOR.3]);


        self.texture_atlas.render(c, g);
        //for layer in self.map.layers.iter() {
        //    self.render_layer(layer, c, g);
        //}

        //image(self.render_cache.borrow(), c.transform, g);
    }
}
impl TiledMap {
    fn render_layer(&self, layer: &Layer, c: Context, g: &mut G2d) {
        if !layer.visible {
            return;
        }

        for (row, rt) in layer.tiles.iter().enumerate() {
            for (column, ct) in rt.iter().enumerate() {
                //self.render_tile(*ct, [column, row], c, g);
            }
        }
    }

    fn render_tile(&self, tile: u32, render_pos: [usize; 2], c: Context, g: &mut G2d) {
        let tileset = match self.map.get_tileset_by_gid(tile) {
            Some(t) => t,
            None => return,
        };

        let img = &tileset.images[0];
        //let t = self.image_buffers.get(&img.source).unwrap();
        //let texture_index = [
        //    tile as usize % t.len(),
        //    tile as usize / t.len(),
        //];

        // TODO:
        // This is very wrong
        // but hey
        // it works!
        //let tex = &t[texture_index[0]-1][texture_index[1]];
        //let rc = self.render_cache;
        //let tex_vec: &[u8] =
        //    &tex.into_vec()[..]; // TODO: This is cloning (I think), NOT GOOD
        //g.encoder.update_texture(
        //    &rc.surface,
        //    None,
        //    // TODO: We should store this at Creation time
        //    NewImageInfo {
        //        xoffset: (render_pos[0] * tileset.tile_width as usize) as u16,
        //        yoffset: (render_pos[1] * tileset.tile_height as usize) as u16,
        //        zoffset: 0,
        //        width:   16,
        //        height:  16,
        //        depth:   0,
        //        format:  (),
        //        mipmap:  0, // No idea what this means
        //    },
        //    tex_vec
        //);
        //self.render_cache.update();
        //image(tex.borrow(), c.transform.trans(
        //    (render_pos[0] * tileset.tile_width as usize) as f64,
        //    (render_pos[1] * tileset.tile_height as usize) as f64,
        //), g);
    }
}

impl Updatable for TiledMap {
    fn update(&mut self, upd: UpdateArgs) {}
}
