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
  sleigh_image: assets::Image<'a>,
  reindeer_image: assets::Image<'a>,
  canvas_size: assets::Point,

  collided_with_level_sound1: &'a assets::Sound,
  collided_with_level_sound2: &'a assets::Sound,

  pub size: Point,
  pub position: Point,
  velocity: Point,
  acceleration: Point,
  sleigh_frame: f64,
  reindeer_frame: f64,
  invincible: bool,
  immobile: bool,
  invincible_remaining_duration: std::time::Duration,
  immobile_remaining_duration: std::time::Duration,
  last_update_instant: std::time::Instant,

  max_velocity: Point,
  reindeer_offset: Point,
  frame_speed: f64,
  level_collision_damage_points: f64,
  invincible_duration: std::time::Duration,
  immobile_duration: std::time::Duration,
  invincible_blink_periods: i32,
}

pub struct Chimney {
  pub position: Point,
  pub size: Point,
  pub frame: f64,
}

pub struct Gift<'a> {
  image: &'a assets::Image<'a>,
  star_image: &'a assets::Image<'a>,
  points10_image: &'a assets::Image<'a>,
  points15_image: &'a assets::Image<'a>,
  points20_image: &'a assets::Image<'a>,
  canvas_size: assets::Point,

  collided_with_chimney_sound: &'a assets::Sound,
  collided_with_ground_sound: &'a assets::Sound,

  pub mode: GiftMode,

  pub position: Point,
  velocity: Point,
  acceleration: Point,
  frame: f64,
  last_update_instant: std::time::Instant,

  star1_offset: Point,
  star2_offset: Point,
  star3_offset: Point,
  points_offset: Point,
  star1_frame_offset: f64,
  star2_frame_offset: f64,
  star3_frame_offset: f64,
  frame_speed: f64,
  showing_points_frame_speed: f64,
  damage_points: f64,
}

#[derive(PartialEq)]
pub enum GiftMode {
  Falling,
  ShowingPoints(f64),
  CanBeDeleted,
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
      sleigh_image: sleigh_image,
      reindeer_image: reindeer_image,
      canvas_size: canvas_size,

      collided_with_level_sound1: asset_library.get_sound("sleighCollidedWithLevel1"),
      collided_with_level_sound2: asset_library.get_sound("sleighCollidedWithLevel2"),

      size: size,
      position: Point::zero(),
      velocity: Point::zero(),
      acceleration: Point::new(25.0, 25.0),
      sleigh_frame: 0.0,
      reindeer_frame: 0.0,
      invincible: false,
      immobile: false,
      invincible_remaining_duration: std::time::Duration::from_millis(0),
      immobile_remaining_duration: std::time::Duration::from_millis(0),
      last_update_instant: std::time::Instant::now(),

      max_velocity: Point::new(200.0, 200.0),
      reindeer_offset: reindeer_offset,
      frame_speed: 13.0,
      level_collision_damage_points: 50.0,
      invincible_duration: std::time::Duration::from_millis(8000),
      immobile_duration: std::time::Duration::from_millis(5000),
      invincible_blink_periods: 16,
    };
  }

  pub fn check_keyboard_state(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
    if self.immobile { return; }
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

    if ((self.velocity.x < 0.0) && (self.position.x <= 0.0))
          || ((self.velocity.x > 0.0) && (self.position.x >= self.canvas_size.x - self.size.x)) {
      self.velocity.x = 0.0;
    }
  }

  fn update_velocity_y(&mut self, sign: f64) {
    self.velocity.y = (self.velocity.y + sign * self.acceleration.y)
        .max(-self.max_velocity.y).min(self.max_velocity.y);

    if ((self.velocity.y < 0.0) && (self.position.y <= 0.0))
          || ((self.velocity.y > 0.0) && (self.position.y >= self.canvas_size.y - self.size.y)) {
      self.velocity.y = 0.0;
    }
  }

  pub fn do_logic(&mut self, score: &mut ui::Score, landscape: &mut level::Landscape,
        level: &mut level::Level) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = now.duration_since(self.last_update_instant).as_secs_f64();

    self.position.x = (self.position.x
        + (seconds_since_last_update * (self.velocity.x as f64)) as f64)
        .max(0.0).min(self.canvas_size.x - self.size.x);
    self.position.y = (self.position.y
        + (seconds_since_last_update * (self.velocity.y as f64)) as f64)
        .max(0.0).min(self.canvas_size.y - self.size.y);

    if !self.immobile {
      self.sleigh_frame += seconds_since_last_update * self.frame_speed;
      self.reindeer_frame = self.sleigh_frame +
          (self.reindeer_image.total_number_of_frames() as f64) / 2.0;
    }

    if self.invincible || self.immobile {
      if self.invincible {
        if self.invincible_remaining_duration > now - self.last_update_instant {
          self.invincible_remaining_duration -= now - self.last_update_instant;
        } else {
          self.invincible = false;
        }
      }

      if self.immobile {
        if self.immobile_remaining_duration > now - self.last_update_instant {
          self.immobile_remaining_duration -= now - self.last_update_instant;
        } else {
          self.immobile = false;
        }
      }
    } else if self.collides_with_level(level) {
      let collided_with_level_sound = match rand::thread_rng().gen_range(0, 2) {
        0 => self.collided_with_level_sound1,
        _ => self.collided_with_level_sound2,
      };

      collided_with_level_sound.play_with_position(level, self.position.x);
      self.invincible = true;
      self.immobile = true;
      self.invincible_remaining_duration = self.invincible_duration;
      self.immobile_remaining_duration = self.immobile_duration;
      self.velocity = Point::new(0.0, -self.max_velocity.y);
      score.add_damage_points(self.level_collision_damage_points);
      landscape.pause_scrolling(now + self.immobile_duration);
      level.pause_scrolling(now + self.immobile_duration);
    }

    self.last_update_instant = now;
  }

  fn collides_with_level(&self, level: &level::Level) -> bool {
    for (tile_x, tile_y) in level.visible_tiles_iter() {
      let tile_frame = level.background_object_map[tile_y][tile_x];
      if tile_frame < 0.0 { continue; }
      let tile_position = Point::new((tile_x as f64) * level.tile_size.x - level.offset_x,
          (tile_y as f64) * level.tile_size.y);

      if self.sleigh_image.collides(self.position, self.sleigh_frame, level.image,
            tile_position, tile_frame) || self.reindeer_image.collides(Point::new(
              self.position.x + self.sleigh_image.width() + self.reindeer_offset.x,
              self.position.y + self.reindeer_offset.y),
            self.sleigh_frame, level.image, tile_position, tile_frame) {
        return true;
      }
    }

    return false;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    if self.invincible && ((self.invincible_remaining_duration.as_secs_f64()
            / self.invincible_duration.as_secs_f64())
          * (self.invincible_blink_periods as f64)) % 1.0 >= 0.5 {
      return;
    }

    let mut position = self.position;
    self.sleigh_image.draw(canvas, position, self.sleigh_frame);
    position.x += self.sleigh_image.width() + self.reindeer_offset.x;
    position.y += self.reindeer_offset.y;
    self.reindeer_image.draw(canvas, position, self.reindeer_frame);
    position.x -= self.reindeer_offset.x;
    self.reindeer_image.draw(canvas, position, self.reindeer_frame);
  }
}

impl<'a> Gift<'a> {
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>, level: &level::Level<'_>,
        sleigh: &Sleigh<'_>, canvas_size: Point, difficulty: game::Difficulty) -> Gift<'a> {
    let number_of_gift_types = 4;
    let image = asset_library.get_image(format!("gift{}",
        rand::thread_rng().gen_range(1, number_of_gift_types)));
    let velocity_y = 50.0;
    let velocity = match difficulty {
      game::Difficulty::Easy => Point::new(level.scroll_speed_x, velocity_y),
      game::Difficulty::Hard => Point::new(sleigh.velocity.x + level.scroll_speed_x,
        velocity_y + sleigh.velocity.y),
    };

    return Gift{
      image: image,
      star_image: asset_library.get_image("bigStar"),
      points10_image: asset_library.get_image("points10"),
      points15_image: asset_library.get_image("points15"),
      points20_image: asset_library.get_image("points20"),
      canvas_size: canvas_size,

      collided_with_chimney_sound: asset_library.get_sound("giftCollidedWithChimney"),
      collided_with_ground_sound: asset_library.get_sound("giftCollidedWithGround"),

      mode: GiftMode::Falling,

      position: Point::new(sleigh.position.x + level.offset_x, sleigh.position.y + sleigh.size.y),
      velocity: velocity,
      acceleration: Point::new(0.0, 200.0),
      frame: rand::thread_rng().gen_range(0, image.total_number_of_frames()) as f64,
      last_update_instant: std::time::Instant::now(),

      star1_offset: Point::new(10.0, 10.0),
      star2_offset: Point::new(25.0, 15.0),
      star3_offset: Point::new(15.0, 25.0),
      points_offset: Point::new(10.0, 10.0),
      star1_frame_offset: 0.0,
      star2_frame_offset: 2.0,
      star3_frame_offset: 4.0,
      frame_speed: 15.0,
      showing_points_frame_speed: 15.0,
      damage_points: 15.0,
    };
  }

  pub fn do_logic(&mut self, score: &mut ui::Score<'_>, level: &level::Level<'_>,
        chimneys: &Vec<Chimney>) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = now.duration_since(self.last_update_instant).as_secs_f64();

    match self.mode {
      GiftMode::Falling => {
        self.position.x += seconds_since_last_update * self.velocity.x;
        self.position.y += seconds_since_last_update * self.velocity.y;
        self.velocity.x += seconds_since_last_update * self.acceleration.x;
        self.velocity.y += seconds_since_last_update * self.acceleration.y;
        self.frame += seconds_since_last_update * self.frame_speed;

        if let Some(chimney_tile_y) = self.has_collided_with_chimney(level, chimneys) {
          let gift_points = if chimney_tile_y <= 1 { 10.0 }
              else if chimney_tile_y == 2 { 15.0 } else { 20.0 };
          self.mode = GiftMode::ShowingPoints(gift_points);
          self.frame = 0.0;
          self.collided_with_chimney_sound.play_with_level_position(level, self.position.x);
          score.add_gift_points(gift_points);
        } else if self.has_collided_with_ground() {
          self.mode = GiftMode::CanBeDeleted;
          self.collided_with_ground_sound.play_with_level_position(level, self.position.x);
          score.add_damage_points(self.damage_points);
        }
      },
      GiftMode::ShowingPoints(_) => {
        self.frame += seconds_since_last_update * self.showing_points_frame_speed;

        if self.frame >= self.star3_frame_offset
              + (self.star_image.total_number_of_frames() as f64) {
          self.mode = GiftMode::CanBeDeleted;
        }
      },
      _ => {},
    }

    self.last_update_instant = now;
  }

  fn has_collided_with_chimney(&self, level: &level::Level<'_>,
        chimneys: &Vec<Chimney>) -> Option<usize> {
    for (tile_x, tile_y) in level.visible_tiles_iter() {
      let frame = level.background_object_map[tile_y][tile_x];
      if frame < 0.0 { continue; }
      let tile_position = Point::new((tile_x as f64) * level.tile_size.x,
          (tile_y as f64) * level.tile_size.y);

      for chimney in chimneys.iter() {
        if (chimney.frame == frame)
              && (self.position.x >= tile_position.x + chimney.position.x)
              && (self.position.x <= tile_position.x + chimney.position.x + chimney.size.x)
              && (self.position.y >= tile_position.y + chimney.position.y)
              && (self.position.y <= tile_position.y + chimney.position.y + chimney.size.y) {
          return Some(tile_y);
        }
      }
    }

    return None;
  }

  fn has_collided_with_ground(&self) -> bool {
    return self.position.y >= self.canvas_size.y;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>, level: &level::Level<'_>) {
    let position: Point = Point::new(self.position.x - level.offset_x, self.position.y);

    match self.mode {
      GiftMode::Falling => {
        self.image.draw(canvas, position, self.frame);
      },
      GiftMode::ShowingPoints(gift_points) => {
        let position: Point = Point::new(position.x - self.star_image.width() / 2.0,
            position.y - self.star_image.height() / 2.0);
        self.draw_star(position, canvas);
        self.draw_points(position, gift_points, canvas);
      },
      _ => {},
    }
  }

  pub fn draw_star<RenderTarget: sdl2::render::RenderTarget>(
        &self, position: Point, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    let number_of_star_frames = self.star_image.total_number_of_frames() as f64;

    if (self.frame >= self.star1_frame_offset)
          && (self.frame < self.star1_frame_offset + number_of_star_frames) {
      self.star_image.draw(canvas, Point::new(position.x + self.star1_offset.x,
          position.y + self.star1_offset.y), self.frame - self.star1_frame_offset);
    }

    if (self.frame >= self.star2_frame_offset)
          && (self.frame < self.star2_frame_offset + number_of_star_frames) {
      self.star_image.draw(canvas, Point::new(position.x + self.star2_offset.x,
          position.y + self.star2_offset.y), self.frame - self.star2_frame_offset);
    }

    if (self.frame >= self.star3_frame_offset)
          && (self.frame < self.star3_frame_offset + number_of_star_frames) {
      self.star_image.draw(canvas, Point::new(position.x + self.star3_offset.x,
          position.y + self.star3_offset.y), self.frame - self.star3_frame_offset);
    }
  }

  pub fn draw_points<RenderTarget: sdl2::render::RenderTarget>(&self, position: Point,
        gift_points: f64, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    let points_image = match gift_points as i32 {
      10 => self.points10_image,
      15 => self.points15_image,
      20 => self.points20_image,
      _ => self.points10_image,
    };

    points_image.draw(canvas, Point::new(
        position.x + self.points_offset.x + self.star_image.width() / 2.0,
        position.y + self.points_offset.y + self.star_image.height()), 0.0)
  }
}
