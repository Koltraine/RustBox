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
use gfx::Factory;
use gfx_device_gl::Resources;
use std::fs::File;
use std::rc::Rc;
use piston_window::image;
use std::borrow::Borrow;

error_chain!{
    foreign_links {
        InputParsing(::tiled::TiledError);
    }
}

// TODO: Check how fast the render time for this is
// Maybe we could cache the final map,
// so render the whole thing into a final buffer
// and render that as an image
// we then only have to re-render it if the camera moves
pub struct TiledMap {
    map: tiled::Map,
    // This feels a bit like a hack
    // The reason we do this is because we only have access to a
    // Factory<Resources> at the beginning of the program
    //
    // Once we have bigger maps we should change this to create
    // textures at runtime
    texture_buffers: HashMap<String, Vec<Vec<Rc<G2dTexture>>>>,
}

impl TiledMap {
    pub fn new<F: Factory<Resources>>(path: &Path, factory: &mut F) -> Result<TiledMap> {

        // This code is a mess, please clean it up
        let mut pathbuf = path.to_path_buf();
        pathbuf.pop(); // Pop the map name

        let map = parse(File::open(path).unwrap())?;
        let mut texture_buffers = HashMap::new();

        for ref tileset in &map.tilesets {
            let tile_dimensions = [
                tileset.tile_width,
                tileset.tile_height
            ];
            for ref image in &tileset.images {
                let mut image_path = pathbuf.clone();
                image_path.push(image.source.clone());
                let tb = TileBuffer::new(image_path, tile_dimensions);

                let mut col_vec = vec![];
                for i in 0..tb.tiles_available()[0] {
                    let mut row_vec = vec![];

                    for j in 0..tb.tiles_available()[1] {
                        row_vec.push(tb.texture([i,j], factory).unwrap());
                    }

                    col_vec.push(row_vec);
                }

                texture_buffers.insert(
                    image.source.clone(),
                    col_vec
                );

            }
        }

        Ok(TiledMap {
            map,
            texture_buffers
        })
    }
}

impl Renderable for TiledMap {
    fn render(&self, c: Context, g: &mut G2d) {
        for layer in self.map.layers.iter() {
            self.render_layer(layer, c, g);
        }
    }
}
impl TiledMap {
    fn render_layer(&self, layer: &Layer, c: Context, g: &mut G2d) {
        if !layer.visible {
            return;
        }

        for row in 0..layer.tiles.len() {
            for column in 0..layer.tiles[row].len() {
                let tile_id = layer.tiles[row][column];
                self.render_tile(tile_id, [column, row], c, g);
            }
        }
    }

    fn render_tile(&self, tile: u32, render_pos: [usize; 2], c: Context, g: &mut G2d) {
        let tileset = match self.map.get_tileset_by_gid(tile) {
            Some(t) => t,
            None => return,
        };

        let img = &tileset.images[0];
        let t = self.texture_buffers.get(&img.source).unwrap();
        let texture_index = [
            tile as usize % t.len(),
            tile as usize / t.len(),
        ];

        // TODO:
        // This is very wrong
        // but hey
        // it works!
        let tex = &t[texture_index[0]-1][texture_index[1]];
        image(tex.borrow(), c.transform.trans(
            (render_pos[0] * tileset.tile_width as usize) as f64,
            (render_pos[1] * tileset.tile_height as usize) as f64,
        ), g);
    }
}

impl Updatable for TiledMap {
    fn update(&mut self, upd: UpdateArgs) {}
}
