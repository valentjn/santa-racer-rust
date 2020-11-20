/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use rand::Rng;

use crate::*;
use crate::assets::Point;

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

impl<'a> Gift<'a> {
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>, level: &level::Level<'_>,
        sleigh: &sleigh::Sleigh<'_>, canvas_size: Point, difficulty: game::Difficulty) -> Gift<'a> {
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
      let frame = level.tile_map[tile_y][tile_x];
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
