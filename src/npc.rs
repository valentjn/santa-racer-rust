/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::asset::Point;

pub trait Npc {
  fn do_logic(&mut self, level_offset_x: f64, sleigh: &sleigh::Sleigh);
  fn collides_with_sleigh(&self, sleigh: &sleigh::Sleigh) -> bool;
  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level: &level::Level);

  fn tile(&self) -> (usize, usize);
  fn z_order(&self) -> f64;
}

struct NpcBase<'a> {
  image: &'a asset::Image<'a>,
  canvas_size: Point,
  level_tile_size: Point,

  tile: (usize, usize),
  position: Point,
  velocity: Point,
  acceleration: Point,
  frame: f64,
  last_update_instant: std::time::Instant,

  frame_speed: f64,
}

struct Angel<'a> {
  npc_base: NpcBase<'a>,
}

struct Balloon<'a> {
  npc_base: NpcBase<'a>,

  balloon_type: BalloonType,
  frame_increasing: bool,
}

struct Cloud<'a> {
  npc_base: NpcBase<'a>,
}

struct Finish<'a> {
  npc_base: NpcBase<'a>,
}

struct Goblin<'a> {
  npc_base: NpcBase<'a>,
}

struct Snowman<'a> {
  npc_base: NpcBase<'a>,
}

enum BalloonType {
  Cash,
  Gift,
  Heart,
  Shield,
  Wine,
}

pub fn new_npc<'a>(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
      tile: (usize, usize), frame: f64) -> Box<dyn Npc + 'a> {
  if frame == 70.0 {
    return Box::new(Angel::new(asset_library, level, tile));
  } else if frame == 70.0 {
    return Box::new(Balloon::new(asset_library, level, tile, BalloonType::Cash));
  } else if frame == 71.0 {
    return Box::new(Balloon::new(asset_library, level, tile, BalloonType::Heart));
  } else if frame == 72.0 {
    return Box::new(Balloon::new(asset_library, level, tile, BalloonType::Wine));
  } else if frame == 73.0 {
    return Box::new(Balloon::new(asset_library, level, tile, BalloonType::Gift));
  } else if frame == 75.0 {
    return Box::new(Balloon::new(asset_library, level, tile, BalloonType::Shield));
  } else if frame == 74.0 {
    return Box::new(Cloud::new(asset_library, level, tile));
  } else if frame == 76.0 {
    return Box::new(Finish::new(asset_library, level, tile));
  } else if frame == 68.0 {
    return Box::new(Goblin::new(asset_library, level, tile));
  } else if frame == 29.0 {
    return Box::new(Snowman::new(asset_library, level, tile));
  } else {
    return Box::new(Angel::new(asset_library, level, tile));
  }
}

impl<'a> NpcBase<'a> {
  pub fn new(image: &'a asset::Image<'a>, level: &level::Level, tile: (usize, usize),
        frame_speed: f64) -> NpcBase<'a> {
    return NpcBase{
      image: image,
      canvas_size: level.canvas_size,
      level_tile_size: level.tile_size,

      tile: tile,
      position: Point::new(((tile.0 as f64) + 0.5) * level.tile_size.x - image.width() / 2.0,
        ((tile.1 as f64) + 0.5) * level.tile_size.y - image.height() / 2.0),
      velocity: Point::zero(),
      acceleration: Point::zero(),
      frame: 0.0,
      last_update_instant: std::time::Instant::now(),

      frame_speed: frame_speed,
    };
  }
}

impl<'a> NpcBase<'a> {
  fn do_logic(&mut self) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = (now - self.last_update_instant).as_secs_f64();

    self.velocity.x = self.velocity.x + seconds_since_last_update * self.acceleration.x;
    self.velocity.y = self.velocity.y + seconds_since_last_update * self.acceleration.y;

    self.position.x = self.position.x + seconds_since_last_update * self.velocity.x;
    self.position.y = self.position.y + seconds_since_last_update * self.velocity.y;

    self.frame += seconds_since_last_update * self.frame_speed;

    self.last_update_instant = now;
  }

  fn collides_with_sleigh(&self, sleigh: &sleigh::Sleigh) -> bool {
    return false;
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level: &level::Level) {
    self.image.draw(canvas, Point::new(self.position.x - level.offset_x, self.position.y),
        self.frame);
  }
}

impl<'a> Angel<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Angel<'a> {
    return Angel{
      npc_base: NpcBase::new(asset_library.get_image("angel"), level, tile, 13.0),
    };
  }
}

impl<'a> Npc for Angel<'a> {
  fn do_logic(&mut self, _level_offset_x: f64, _sleigh: &sleigh::Sleigh) {
    self.npc_base.do_logic();
  }

  fn collides_with_sleigh(&self, sleigh: &sleigh::Sleigh) -> bool {
    return self.npc_base.collides_with_sleigh(sleigh);
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level: &level::Level) {
    self.npc_base.draw(canvas, level);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }
}

impl<'a> Balloon<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize), balloon_type: BalloonType) -> Balloon<'a> {
    let image = match balloon_type {
      BalloonType::Cash => asset_library.get_image("cashBalloon"),
      BalloonType::Gift => asset_library.get_image("giftBalloon"),
      BalloonType::Heart => asset_library.get_image("heartBalloon"),
      BalloonType::Shield => asset_library.get_image("shieldBalloon"),
      BalloonType::Wine => asset_library.get_image("wineBalloon"),
    };

    return Balloon{
      npc_base: NpcBase::new(image, level, tile, 10.0),

      balloon_type: balloon_type,
      frame_increasing: true,
    };
  }
}

impl<'a> Npc for Balloon<'a> {
  fn do_logic(&mut self, level_offset_x: f64, _sleigh: &sleigh::Sleigh) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = (now - self.npc_base.last_update_instant).as_secs_f64();

    if (level_offset_x + self.npc_base.canvas_size.x) / self.npc_base.level_tile_size.x
          >= self.npc_base.tile.0 as f64 {
      self.npc_base.velocity.y = -50.0;
    }

    let frame = self.npc_base.frame;
    self.npc_base.do_logic();

    let number_of_frames = self.npc_base.image.total_number_of_frames() as f64;
    let sign = if self.frame_increasing { 1.0 } else { -1.0 };
    self.npc_base.frame = frame + sign * seconds_since_last_update * self.npc_base.frame_speed;

    while (self.npc_base.frame < 0.0) || (self.npc_base.frame > number_of_frames) {
      self.npc_base.frame = if self.npc_base.frame < 0.0 { -self.npc_base.frame }
          else { 2.0 * number_of_frames - self.npc_base.frame };
      self.frame_increasing = !self.frame_increasing;
    }

    self.npc_base.last_update_instant = now;
  }

  fn collides_with_sleigh(&self, sleigh: &sleigh::Sleigh) -> bool {
    return self.npc_base.collides_with_sleigh(sleigh);
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level: &level::Level) {
    self.npc_base.draw(canvas, level);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return -1.0;
  }
}

impl<'a> Cloud<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Cloud<'a> {
    return Cloud{
      npc_base: NpcBase::new(asset_library.get_image("cloud"), level, tile, 0.0),
    };
  }
}

impl<'a> Npc for Cloud<'a> {
  fn do_logic(&mut self, _level_offset_x: f64, _sleigh: &sleigh::Sleigh) {
    self.npc_base.do_logic();
  }

  fn collides_with_sleigh(&self, sleigh: &sleigh::Sleigh) -> bool {
    return self.npc_base.collides_with_sleigh(sleigh);
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level: &level::Level) {
    self.npc_base.draw(canvas, level);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 1.0;
  }
}

impl<'a> Finish<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Finish<'a> {
    return Finish{
      npc_base: NpcBase::new(asset_library.get_image("finish"), level, tile, 0.0),
    };
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }
}

impl<'a> Npc for Finish<'a> {
  fn do_logic(&mut self, level_offset_x: f64, sleigh: &sleigh::Sleigh) {
    self.npc_base.do_logic();
  }

  fn collides_with_sleigh(&self, sleigh: &sleigh::Sleigh) -> bool {
    return self.npc_base.collides_with_sleigh(sleigh);
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level: &level::Level) {
    self.npc_base.draw(canvas, level);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }
}

impl<'a> Goblin<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Goblin<'a> {
    return Goblin{
      npc_base: NpcBase::new(asset_library.get_image("goblin"), level, tile, 12.0),
    };
  }
}

impl<'a> Npc for Goblin<'a> {
  fn do_logic(&mut self, level_offset_x: f64, _sleigh: &sleigh::Sleigh) {
    self.npc_base.do_logic();
  }

  fn collides_with_sleigh(&self, sleigh: &sleigh::Sleigh) -> bool {
    return self.npc_base.collides_with_sleigh(sleigh);
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level: &level::Level) {
    self.npc_base.draw(canvas, level);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }
}

impl<'a> Snowman<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Snowman<'a> {
    return Snowman{
      npc_base: NpcBase::new(asset_library.get_image("snowman"), level, tile, 8.0),
    };
  }
}

impl<'a> Npc for Snowman<'a> {
  fn do_logic(&mut self, level_offset_x: f64, sleigh: &sleigh::Sleigh) {
    self.npc_base.do_logic();
  }

  fn collides_with_sleigh(&self, sleigh: &sleigh::Sleigh) -> bool {
    return self.npc_base.collides_with_sleigh(sleigh);
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level: &level::Level) {
    self.npc_base.draw(canvas, level);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }
}
