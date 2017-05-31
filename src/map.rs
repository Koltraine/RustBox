// Copyright (C) 2017 Afonso Bordado
//
// Licensed under the MIT license <http://opensource.org/licenses/MIT>,
// This file may not be copied, modified, or distributed except according
// to the terms of that license.

use piston_window::*;
use objects::{Renderable, Updatable, GameObject};
use tiled;
use rand;
use rand::Rng;

error_chain!{
    errors { Fail }
}

pub struct TiledMap {
    tiles: Vec<Vec<u32>>,
    total_size_x: usize,
    total_size_y: usize,
    i: usize,
}

impl TiledMap {
    pub fn new(seed: u64) -> TiledMap {
        let initial_map = rand::thread_rng().gen::<[u8; 4]>();
        TiledMap {
            tiles: vec![
                vec![initial_map[0] as u32, initial_map[1] as u32],
                vec![initial_map[2] as u32, initial_map[3] as u32],
            ],
            total_size_x: 300,
            total_size_y: 300,
            i: 0,
        }
    }

    fn update_map(&mut self) {
        let next_size = (self.tiles.len() * 2) - 1;
        for row in 0..(self.tiles.len() - 1) {
            for tile in 0..(self.tiles.len() - 1) {
            }
        }
        //self.tiles.iter().intersperse()
        //unimplemented!();
    }

    fn avg_square(s: [f64; 4]) -> f64 {
        let res: f64 = s.iter().sum();
        res / 4.0
    }
}

impl Renderable for TiledMap {
    fn render(&self, c: Context, g: &mut G2d) {
        //TODO: For now lets assume the thing is squared
        let tile_size = self.total_size_x / self.tiles.len();

        for row in 0..self.tiles.len() {
            for tile in 0..self.tiles.len() {
                let color = 1.0 / ((self.tiles[row][tile] % 9 + 1) as f32);
                let rectangle = Rectangle::new([color, color, color, 1.0]);
                rectangle.draw(
                    [0.0, 0.0, tile_size as f64, tile_size as f64],
                    &c.draw_state,
                    c.transform.trans(
                        (tile_size * tile) as f64,
                        (tile_size * row) as f64
                    ),
                    g
                );
            }
        }
    }
}

impl Updatable for TiledMap {
    fn update(&mut self, upd: UpdateArgs) {
        self.i += 1;
        if self.i % 300 == 0 {
            self.update_map();
        }
    }
}

impl GameObject for TiledMap {}
