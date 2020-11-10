/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use rand::Rng;

use crate::*;
use crate::assets::Point;

pub struct Sleigh<'a> {
  canvas_size: assets::Point,
  sleigh_image: assets::Image<'a>,
  reindeer_image: assets::Image<'a>,

  pub size: Point,
  pub position: Point,
  velocity: Point,
  acceleration: Point,
  sleigh_frame: f64,
  reindeer_frame: f64,
  last_update_instant: std::time::Instant,

  max_velocity: Point,
  reindeer_offset: Point,
  frame_speed: f64,
}

pub struct Gift<'a> {
  image: &'a assets::Image<'a>,

  position: Point,
  velocity: Point,
  acceleration: Point,
  frame: f64,
  last_update_instant: std::time::Instant,

  frame_speed: f64,
}

impl<'a> Sleigh<'a> {
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>, canvas_size: Point,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) ->
        Sleigh<'a> {
    let sleigh_image = asset_library.get_image("sleigh").clone(texture_creator);
    let reindeer_image = asset_library.get_image("reindeer").clone(texture_creator);
    let reindeer_offset = Point::new(10.0, 3.0);
    let size = Point::new(sleigh_image.height() + reindeer_image.height() +
        reindeer_offset.x, sleigh_image.height());

    return Sleigh{
      canvas_size: canvas_size,
      sleigh_image: sleigh_image,
      reindeer_image: reindeer_image,

      size: size,
      position: Point::zero(),
      velocity: Point::zero(),
      acceleration: Point::new(25.0, 25.0),
      sleigh_frame: 0.0,
      reindeer_frame: 0.0,
      last_update_instant: std::time::Instant::now(),

      max_velocity: Point::new(200.0, 200.0),
      reindeer_offset: reindeer_offset,
      frame_speed: 13.0,
    };
  }

  pub fn check_keyboard_state(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
    let drunk_factor = 1.0;

    if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
      self.update_velocity_x(-drunk_factor);
    } else if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
      self.update_velocity_x(drunk_factor);
    } else {
      self.update_velocity_x(0.0);
    }

    if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
      self.update_velocity_y(-drunk_factor);
    } else if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
      self.update_velocity_y(drunk_factor);
    } else {
      self.update_velocity_y(0.0);
    }
  }

  fn update_velocity_x(&mut self, sign: f64) {
    self.velocity.x = (self.velocity.x + sign * self.acceleration.x)
        .max(-self.max_velocity.x).min(self.max_velocity.x);
  }

  fn update_velocity_y(&mut self, sign: f64) {
    self.velocity.y = (self.velocity.y + sign * self.acceleration.y)
        .max(-self.max_velocity.y).min(self.max_velocity.y);
  }

  pub fn do_logic(&mut self) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = now.duration_since(self.last_update_instant).as_secs_f64();

    self.position.x = (self.position.x
        + (seconds_since_last_update * (self.velocity.x as f64)) as f64)
        .max(0.0).min(self.canvas_size.x - self.size.x);
    self.position.y = (self.position.y
        + (seconds_since_last_update * (self.velocity.y as f64)) as f64)
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

impl<'a, 'b> Gift<'a> {
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>, level: &'b level::Level<'a>,
        sleigh: &'b Sleigh<'a>) -> Gift<'a> {
    let number_of_gift_types = 4;
    let image = asset_library.get_image(format!("gift{}",
        rand::thread_rng().gen_range(1, number_of_gift_types)));

    return Gift{
      image: image,

      position: Point::new(sleigh.position.x + level.offset_x, sleigh.position.y + sleigh.size.y),
      velocity: Point::new(level.scroll_speed_x, 50.0),
      acceleration: Point::new(0.0, 200.0),
      frame: rand::thread_rng().gen_range(0, image.total_number_of_frames()) as f64,
      last_update_instant: std::time::Instant::now(),

      frame_speed: 15.0,
    };
  }

  pub fn do_logic(&mut self) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = now.duration_since(self.last_update_instant).as_secs_f64();

    self.position.x += seconds_since_last_update * self.velocity.x;
    self.position.y += seconds_since_last_update * self.velocity.y;
    self.velocity.x += seconds_since_last_update * self.acceleration.x;
    self.velocity.y += seconds_since_last_update * self.acceleration.y;
    self.frame += seconds_since_last_update * self.frame_speed;

    self.last_update_instant = now;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>, level: &'b level::Level<'a>) {
    self.image.draw(canvas, &Point::new(self.position.x - level.offset_x, self.position.y),
        self.frame);
  }
}
