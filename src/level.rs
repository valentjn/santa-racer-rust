/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::assets::Point;

pub struct Level<'a> {
  image: &'a assets::Image<'a>,
  background_object_map: Vec<Vec<f64>>,
  foreground_object_map: Vec<Vec<f64>>,

  offset_x: f64,
  scroll_speed_x: f64,
  last_update_instant: std::time::Instant,

  canvas_size: Point,
  tile_size: Point,
  number_of_visible_map_columns: usize,
  min_scroll_speed_x: f64,
  max_scroll_speed_x: f64,
}

pub struct Landscape<'a> {
  image: &'a assets::Image<'a>,
  offset_x: f64,
  last_update_instant: std::time::Instant,
  size: Point,
}

const NUMBER_OF_MAP_ROWS: usize = 5;
const MIN_SCROLL_SPEED_X: f64 = 40.0;
const MAX_SCROLL_SPEED_X: f64 = 160.0;

impl<'a> Level<'a> {
  pub fn new(canvas_size: Point,
        image: &'a assets::Image<'a>, background_object_map_data: Vec<f64>,
        foreground_object_map_data: Vec<f64>) -> Level {
    let tile_size = image.size();

    return Level{
      image: image,
      background_object_map: Level::convert_data_to_map(background_object_map_data),
      foreground_object_map: Level::convert_data_to_map(foreground_object_map_data),

      offset_x: 0.0,
      scroll_speed_x: 0.0,
      number_of_visible_map_columns: (canvas_size.x / tile_size.x + 1.0) as usize,

      canvas_size: canvas_size,
      tile_size: tile_size,
      last_update_instant: std::time::Instant::now(),
      min_scroll_speed_x: MIN_SCROLL_SPEED_X,
      max_scroll_speed_x: MAX_SCROLL_SPEED_X,
    };
  }

  fn convert_data_to_map(data: Vec<f64>) -> Vec<Vec<f64>> {
    let number_of_map_rows = NUMBER_OF_MAP_ROWS;
    let number_of_map_columns = (data.len() as f64 / number_of_map_rows as f64).ceil() as usize;
    let mut map: Vec<Vec<f64>> = Vec::new();

    for y in 0 .. number_of_map_rows {
      let mut row: Vec<f64> = Vec::new();

      for x in 0 .. number_of_map_columns {
        let i = x + y * number_of_map_columns;
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
    let min_x = (self.offset_x / self.tile_size.x) as usize;

    for y in 0 .. self.background_object_map.len() {
      let row = &self.background_object_map[y];
      let max_x = (min_x + self.number_of_visible_map_columns + 1).min(row.len());

      for x in min_x .. max_x {
        if row[x] < 0.0 { continue; }
        let dst_point = Point::new((x as f64) * self.tile_size.x - self.offset_x,
            (y as f64) * self.tile_size.y);
        self.image.draw(canvas, &dst_point, row[x]);
      }
    }
  }
}

impl<'a> Landscape<'a> {
  pub fn new(image: &'a assets::Image<'a>) -> Landscape {
    return Landscape{
      image: image,
      offset_x: 0.0,
      last_update_instant: std::time::Instant::now(),
      size: image.size(),
    };
  }

  pub fn do_logic(&mut self, level: &level::Level) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = now.duration_since(self.last_update_instant).as_secs_f64();

    let scroll_speed_x = level.scroll_speed_x / 10.0;
    self.offset_x = (self.offset_x + seconds_since_last_update * scroll_speed_x) %
        self.size.x;

    self.last_update_instant = now;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    self.image.draw_blit(canvas, &sdl2::rect::Rect::new(self.offset_x as i32, 0,
        (self.size.x - self.offset_x) as u32, self.size.y as u32),
        &Point::zero(), 0.0);
    self.image.draw_blit(canvas, &sdl2::rect::Rect::new(0, 0,
        self.offset_x as u32, self.size.y as u32),
        &Point::new(self.size.x - self.offset_x, 0.0), 0.0);
  }
}
