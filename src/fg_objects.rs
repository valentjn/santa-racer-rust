/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::assets::Point;

pub struct Sleigh<'a> {
  canvas_size: assets::Point,
  sleigh_image: assets::Image<'a>,
  reindeer_image: assets::Image<'a>,

  pub size: Point,
  pub position: Point,
  speed: Point,
  acceleration: Point,
  sleigh_frame: f64,
  reindeer_frame: f64,
  last_update_instant: std::time::Instant,

  max_speed: Point,
  reindeer_offset: Point,
  frame_speed: f64,
}

impl<'a> Sleigh<'a> {
  pub fn new(canvas_size: Point, sleigh_image: assets::Image<'a>,
        reindeer_image: assets::Image<'a>) -> Sleigh<'a> {
    let reindeer_offset = Point::new(10.0, 3.0);
    let size = Point::new(sleigh_image.height() + reindeer_image.height() +
        reindeer_offset.x, sleigh_image.height());

    return Sleigh{
      canvas_size: canvas_size,
      sleigh_image: sleigh_image,
      reindeer_image: reindeer_image,

      size: size,
      position: Point::zero(),
      speed: Point::zero(),
      acceleration: Point::new(25.0, 25.0),
      sleigh_frame: 0.0,
      reindeer_frame: 0.0,
      last_update_instant: std::time::Instant::now(),

      max_speed: Point::new(200.0, 200.0),
      reindeer_offset: reindeer_offset,
      frame_speed: 13.0,
    };
  }

  pub fn check_keys(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
    let drunk_factor = 1.0;

    if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
      self.update_speed_x(-drunk_factor);
    } else if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
      self.update_speed_x(drunk_factor);
    } else {
      self.update_speed_x(0.0);
    }

    if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
      self.update_speed_y(-drunk_factor);
    } else if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
      self.update_speed_y(drunk_factor);
    } else {
      self.update_speed_y(0.0);
    }
  }

  fn update_speed_x(&mut self, sign: f64) {
    self.speed.x = (self.speed.x + sign * self.acceleration.x)
        .max(-self.max_speed.x).min(self.max_speed.x);
  }

  fn update_speed_y(&mut self, sign: f64) {
    self.speed.y = (self.speed.y + sign * self.acceleration.y)
        .max(-self.max_speed.y).min(self.max_speed.y);
  }

  pub fn do_logic(&mut self) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = now.duration_since(self.last_update_instant).as_secs_f64();

    self.position.x = (self.position.x
        + (seconds_since_last_update * (self.speed.x as f64)) as f64)
        .max(0.0).min(self.canvas_size.x - self.size.x);
    self.position.y = (self.position.y
        + (seconds_since_last_update * (self.speed.y as f64)) as f64)
        .max(0.0).min(self.canvas_size.y - self.size.y);

    self.sleigh_frame += seconds_since_last_update * self.frame_speed;
    self.reindeer_frame = self.sleigh_frame +
        (self.reindeer_image.total_number_of_frames() as f64) / 2.0;

    self.last_update_instant = now;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    let mut position = self.position;
    self.sleigh_image.draw(canvas, &position, self.sleigh_frame);
    position.x += self.sleigh_image.width() + self.reindeer_offset.x;
    position.y += self.reindeer_offset.y;
    self.reindeer_image.draw(canvas, &position, self.reindeer_frame);
    position.x -= self.reindeer_offset.x;
    self.reindeer_image.draw(canvas, &position, self.reindeer_frame);
  }
}
