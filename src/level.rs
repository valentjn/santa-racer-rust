/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::assets::Point;

pub struct Landscape<'a> {
  image: &'a assets::Image<'a>,
  offset_x: f64,
  last_update_instant: std::time::Instant,
  size: Point,
  scroll_speed_factor_x: f64,
}

pub struct Level<'a> {
  pub image: &'a assets::Image<'a>,
  pub background_object_map: Vec<Vec<f64>>,
  foreground_object_map: Vec<Vec<f64>>,
  pub canvas_size: Point,

  pub offset_x: f64,
  pub scroll_speed_x: f64,
  last_update_instant: std::time::Instant,

  pub tile_size: Point,
  number_of_tiles: (usize, usize),
  number_of_visible_tiles_x: usize,
  min_scroll_speed_x: f64,
  max_scroll_speed_x: f64,
}

pub struct TileIterator {
  tile_x: usize,
  tile_y: usize,

  min_tile_x: usize,
  max_tile_x: usize,
  min_tile_y: usize,
  max_tile_y: usize,
}

impl<'a> Landscape<'a> {
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>) -> Landscape {
    let image = asset_library.get_image("landscape");

    return Landscape{
      image: image,
      offset_x: 0.0,
      last_update_instant: std::time::Instant::now(),
      size: image.size(),
      scroll_speed_factor_x: 0.1,
    };
  }

  pub fn do_logic(&mut self, level: &level::Level) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = now.duration_since(self.last_update_instant).as_secs_f64();

    let scroll_speed_x = self.scroll_speed_factor_x * level.scroll_speed_x;
    self.offset_x = (self.offset_x + seconds_since_last_update * scroll_speed_x) %
        self.size.x;

    self.last_update_instant = now;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    self.image.draw_blit(canvas, sdl2::rect::Rect::new(self.offset_x as i32, 0,
        (self.size.x - self.offset_x) as u32, self.size.y as u32),
        Point::zero(), 0.0);
    self.image.draw_blit(canvas, sdl2::rect::Rect::new(0, 0,
        self.offset_x as u32, self.size.y as u32),
        Point::new(self.size.x - self.offset_x, 0.0), 0.0);
  }
}

impl<'a> Level<'a> {
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>, canvas_size: Point) -> Level {
    let image = asset_library.get_image("level");
    let tile_size = image.size();
    let background_object_map = Level::convert_data_to_map(
      asset_library.get_data("backgroundObjectMap").to_vec());
    let foreground_object_map = Level::convert_data_to_map(
      asset_library.get_data("foregroundObjectMap").to_vec());

    assert!(background_object_map.len() == foreground_object_map.len(),
        "Lengths of background and foreground object maps are not equal");
    assert!(background_object_map.len() > 0, "Background object map is empty");

    let number_of_tiles_x = background_object_map[0].len();
    let number_of_tiles_y = background_object_map.len();

    for tile_y in 0 .. number_of_tiles_y {
      assert!(background_object_map[tile_y].len() == number_of_tiles_x,
          "Rows of background object map do not have equal length");
      assert!(foreground_object_map[tile_y].len() == number_of_tiles_x,
          "Rows of foreground object map do not have equal length");
    }

    return Level{
      image: image,
      background_object_map: background_object_map,
      foreground_object_map: foreground_object_map,
      canvas_size: canvas_size,

      offset_x: 0.0,
      scroll_speed_x: 0.0,

      tile_size: tile_size,
      number_of_tiles: (number_of_tiles_x, number_of_tiles_y),
      number_of_visible_tiles_x: (canvas_size.x / tile_size.x + 1.0) as usize,
      last_update_instant: std::time::Instant::now(),
      min_scroll_speed_x: 40.0,
      max_scroll_speed_x: 160.0,
    };
  }

  fn convert_data_to_map(data: Vec<f64>) -> Vec<Vec<f64>> {
    let number_of_tiles_y = 5;
    let number_of_tiles_x = (data.len() as f64 / number_of_tiles_y as f64).ceil() as usize;
    let mut map: Vec<Vec<f64>> = Vec::new();

    for tile_y in 0 .. number_of_tiles_y {
      let mut row: Vec<f64> = Vec::new();

      for tile_x in 0 .. number_of_tiles_x {
        let i = tile_x + tile_y * number_of_tiles_x;
        row.push(if i < data.len() { data[i] } else { 0.0 });
      }

      map.push(row);
    }

    return map;
  }

  pub fn do_logic(&mut self, sleigh: &fg_objects::Sleigh) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = now.duration_since(self.last_update_instant).as_secs_f64();

    self.scroll_speed_x = self.min_scroll_speed_x + sleigh.position.x
        / (self.canvas_size.x - sleigh.size.x)
        * (self.max_scroll_speed_x - self.min_scroll_speed_x);
    self.offset_x += seconds_since_last_update * self.scroll_speed_x;

    self.last_update_instant = now;
  }

  pub fn draw_background<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    for (tile_x, tile_y) in self.visible_tiles_iter() {
      let frame = self.background_object_map[tile_y][tile_x];
      if frame < 0.0 { continue; }
      let dst_point = Point::new((tile_x as f64) * self.tile_size.x - self.offset_x,
          (tile_y as f64) * self.tile_size.y);
      self.image.draw(canvas, dst_point, frame);
    }
  }

  pub fn visible_tiles_iter(&self) -> TileIterator {
    let min_tile_x = (self.offset_x / self.tile_size.x) as usize;
    let max_tile_x = (min_tile_x + self.number_of_visible_tiles_x + 1).min(self.number_of_tiles.0);
    let min_tile_y = 0;
    let max_tile_y = self.background_object_map.len();

    return TileIterator {
      tile_x: min_tile_x,
      tile_y: min_tile_y,

      min_tile_x: min_tile_x,
      max_tile_x: max_tile_x,
      min_tile_y: min_tile_y,
      max_tile_y: max_tile_y,
    };
  }
}

impl Iterator for TileIterator {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<Self::Item> {
    if self.tile_y >= self.min_tile_y {
      if self.tile_x >= self.min_tile_x {
        self.tile_x += 1;
      } else {
        self.tile_x = self.min_tile_x;
      }
    } else {
      self.tile_x = self.min_tile_x;
      self.tile_y = self.min_tile_y;
    }

    if self.tile_x >= self.max_tile_x {
      self.tile_x = self.min_tile_x;
      self.tile_y += 1;
    }

    if self.tile_y >= self.max_tile_y {
      return None;
    } else {
      return Some((self.tile_x, self.tile_y));
    }
  }
}
