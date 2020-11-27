/* Copyright (C) 2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::asset::Point;

pub trait Npc {
  fn do_logic(&mut self, level_offset_x: f64, level_scroll_speed_x: f64, sleigh: &sleigh::Sleigh);
  fn check_collision_with_sleigh(&mut self, score: &mut ui::Score,
      level_offset_x: f64, sleigh: &mut sleigh::Sleigh);
  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level_offset_x: f64);

  fn tile(&self) -> (usize, usize);
  fn z_order(&self) -> f64;
  fn check_collision_with_sleigh_in_menu_mode(&self) -> bool;
}

struct NpcBase<'a> {
  image: &'a asset::Image<'a>,
  canvas_size: Point,
  level_tile_size: Point,

  tile: (usize, usize),
  size: Point,
  position: Point,
  velocity: Point,
  acceleration: Point,
  frame: f64,
  last_update_instant: std::time::Instant,

  frame_speed: f64,
}

struct Angel<'a> {
  npc_base: NpcBase<'a>,
  sound: &'a asset::Sound,

  damage_points: f64,
}

struct Balloon<'a> {
  npc_base: NpcBase<'a>,
  sound: &'a asset::Sound,

  balloon_type: BalloonType,
  frame_increasing: bool,
  visible: bool,

  launch_velocity: Point,
  cash_balloon_damage_points: f64,
  heart_balloon_gift_points: f64,
}

struct Cloud<'a> {
  npc_base: NpcBase<'a>,
  sound: &'a asset::Sound,

  damage_points: f64,
}

struct Finish<'a> {
  npc_base: NpcBase<'a>,
}

struct Goblin<'a> {
  npc_base: NpcBase<'a>,
  snowball_image: &'a asset::Image<'a>,
  throw_snowball_sound: &'a asset::Sound,
  collision_sound: &'a asset::Sound,

  snowballs: Vec<NpcBase<'a>>,
  next_throw_snowball_instant: std::time::Instant,

  snowball_velocity: Point,
  snowball_acceleration: Point,
  damage_points: f64,
  throw_snowball_period_duration: std::time::Duration,
}

struct Snowman<'a> {
  npc_base: NpcBase<'a>,
  launch_sound: &'a asset::Sound,
  collision_sound: &'a asset::Sound,

  launched: bool,
  stars: Vec<sleigh::Star<'a>>,

  launch_velocity: Point,
  launch_frame_speed: f64,
  damage_points: f64,
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
  if frame == 69.0 {
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
  pub fn new(image: &'a asset::Image<'a>, canvas_size: Point, level_tile_size: Point,
        tile: (usize, usize), frame_speed: f64) -> NpcBase<'a> {
    return NpcBase{
      image: image,
      canvas_size: canvas_size,
      level_tile_size: level_tile_size,

      tile: tile,
      size: image.size(),
      position: Point::new(((tile.0 as f64) + 0.5) * level_tile_size.x() - image.width() / 2.0,
        ((tile.1 as f64) + 0.5) * level_tile_size.y() - image.height() / 2.0),
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

    self.velocity = self.velocity + seconds_since_last_update * self.acceleration;
    self.position = self.position + seconds_since_last_update * self.velocity;
    self.frame += seconds_since_last_update * self.frame_speed;

    self.last_update_instant = now;
  }

  fn collides_with_sleigh(&self, level_offset_x: f64, sleigh: &mut sleigh::Sleigh) -> bool {
    return sleigh.collides_with_image(self.image,
        Point::new(self.position.x() - level_offset_x, self.position.y()), self.frame);
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level_offset_x: f64) {
    self.image.draw(canvas, Point::new(self.position.x() - level_offset_x, self.position.y()),
        self.frame);
  }
}

impl<'a> Angel<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Angel<'a> {
    return Angel{
      npc_base: NpcBase::new(asset_library.get_image("angel"),
          level.canvas_size(), level.tile_size(), tile, 13.0),
      sound: asset_library.get_sound("sleighCollidedWithNpc"),

      damage_points: 20.0,
    };
  }
}

impl<'a> Npc for Angel<'a> {
  fn do_logic(&mut self, _level_offset_x: f64, _level_scroll_speed_x: f64,
        _sleigh: &sleigh::Sleigh) {
    self.npc_base.do_logic();
  }

  fn check_collision_with_sleigh(&mut self, score: &mut ui::Score,
        level_offset_x: f64, sleigh: &mut sleigh::Sleigh) {
    if !sleigh.invincible() && !sleigh.shield()
          && self.npc_base.collides_with_sleigh(level_offset_x, sleigh) {
      self.sound.play_with_position(self.npc_base.canvas_size, sleigh.position());
      score.add_damage_points(self.damage_points);
      sleigh.start_invincible();
    }
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level_offset_x: f64) {
    self.npc_base.draw(canvas, level_offset_x);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }

  fn check_collision_with_sleigh_in_menu_mode(&self) -> bool {
    return false;
  }
}

impl<'a> Balloon<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize), balloon_type: BalloonType) -> Balloon<'a> {
    let image_name = match balloon_type {
      BalloonType::Cash => "cashBalloon",
      BalloonType::Gift => "giftBalloon",
      BalloonType::Heart => "heartBalloon",
      BalloonType::Shield => "shieldBalloon",
      BalloonType::Wine => "wineBalloon",
    };
    let sound_name = if image_name == "heartBalloon" { "giftCollidedWithChimney" }
        else { image_name };

    return Balloon{
      npc_base: NpcBase::new(asset_library.get_image(image_name),
          level.canvas_size(), level.tile_size(), tile, 10.0),
      sound: asset_library.get_sound(sound_name),

      balloon_type: balloon_type,
      frame_increasing: true,
      visible: true,

      launch_velocity: Point::new(0.0, -50.0),
      cash_balloon_damage_points: -50.0,
      heart_balloon_gift_points: 20.0,
    };
  }
}

impl<'a> Npc for Balloon<'a> {
  fn do_logic(&mut self, level_offset_x: f64, _level_scroll_speed_x: f64,
        _sleigh: &sleigh::Sleigh) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = (now - self.npc_base.last_update_instant).as_secs_f64();

    if (level_offset_x + self.npc_base.canvas_size.x()) / self.npc_base.level_tile_size.x()
          >= self.npc_base.tile.0 as f64 {
      self.npc_base.velocity = self.launch_velocity;
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

  fn check_collision_with_sleigh(&mut self, score: &mut ui::Score,
        level_offset_x: f64, sleigh: &mut sleigh::Sleigh) {
    if self.visible && self.npc_base.collides_with_sleigh(level_offset_x, sleigh) {
      self.sound.play_with_position(self.npc_base.canvas_size, sleigh.position());
      self.visible = false;

      match self.balloon_type {
        BalloonType::Cash => { score.add_damage_points(self.cash_balloon_damage_points); },
        BalloonType::Gift => { sleigh.start_bonus(); },
        BalloonType::Heart => { score.add_gift_points(self.heart_balloon_gift_points); },
        BalloonType::Shield => { sleigh.start_shield(); },
        BalloonType::Wine => { sleigh.start_drunk(); },
      }
    }
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level_offset_x: f64) {
    if self.visible { self.npc_base.draw(canvas, level_offset_x); }
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return -1.0;
  }

  fn check_collision_with_sleigh_in_menu_mode(&self) -> bool {
    return false;
  }
}

impl<'a> Cloud<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Cloud<'a> {
    return Cloud{
      npc_base: NpcBase::new(asset_library.get_image("cloud"),
          level.canvas_size(), level.tile_size(), tile, 0.0),
      sound: asset_library.get_sound("sleighCollidedWithCloud"),

      damage_points: 20.0,
    };
  }
}

impl<'a> Npc for Cloud<'a> {
  fn do_logic(&mut self, _level_offset_x: f64, _level_scroll_speed_x: f64,
        _sleigh: &sleigh::Sleigh) {
    self.npc_base.do_logic();
  }

  fn check_collision_with_sleigh(&mut self, score: &mut ui::Score,
        level_offset_x: f64, sleigh: &mut sleigh::Sleigh) {
    if !sleigh.invincible() && !sleigh.shield()
          && self.npc_base.collides_with_sleigh(level_offset_x, sleigh) {
      self.sound.play_with_position(self.npc_base.canvas_size, sleigh.position());
      score.add_damage_points(self.damage_points);
      sleigh.start_invincible();
      sleigh.start_electrocuted();
    }
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level_offset_x: f64) {
    self.npc_base.draw(canvas, level_offset_x);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 1.0;
  }

  fn check_collision_with_sleigh_in_menu_mode(&self) -> bool {
    return false;
  }
}

impl<'a> Finish<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Finish<'a> {
    return Finish{
      npc_base: NpcBase::new(asset_library.get_image("finish"),
          level.canvas_size(), level.tile_size(), tile, 0.0),
    };
  }
}

impl<'a> Npc for Finish<'a> {
  fn do_logic(&mut self, _level_offset_x: f64, _level_scroll_speed_x: f64,
        _sleigh: &sleigh::Sleigh) {
    self.npc_base.do_logic();
  }

  fn check_collision_with_sleigh(&mut self, score: &mut ui::Score,
        level_offset_x: f64, sleigh: &mut sleigh::Sleigh) {
    if level_offset_x + sleigh.position().x() + sleigh.size().x() / 2.0
          >= self.npc_base.position.x() + self.npc_base.size.x() / 2.0 {
      score.set_won(true);
    }
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level_offset_x: f64) {
    self.npc_base.draw(canvas, level_offset_x);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }

  fn check_collision_with_sleigh_in_menu_mode(&self) -> bool {
    return true;
  }
}

impl<'a> Goblin<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Goblin<'a> {
    let image = asset_library.get_image("goblin");
    let frame_speed = 12.0;
    let throw_snowball_frame = 13.0;

    return Goblin{
      npc_base: NpcBase::new(image, level.canvas_size(), level.tile_size(), tile, frame_speed),
      snowball_image: asset_library.get_image("goblinSnowball"),
      throw_snowball_sound: asset_library.get_sound("goblinThrowSnowball"),
      collision_sound: asset_library.get_sound("sleighCollidedWithNpc"),

      snowballs: Vec::new(),
      next_throw_snowball_instant: std::time::Instant::now() + std::time::Duration::from_secs_f64(
        throw_snowball_frame / frame_speed),

      snowball_velocity: Point::new(-200.0, -250.0),
      snowball_acceleration: Point::new(0.0, 80.0),
      damage_points: 20.0,
      throw_snowball_period_duration: std::time::Duration::from_secs_f64(
          (image.total_number_of_frames() as f64) / frame_speed),
    };
  }
}

impl<'a> Npc for Goblin<'a> {
  fn do_logic(&mut self, level_offset_x: f64, _level_scroll_speed_x: f64,
        _sleigh: &sleigh::Sleigh) {
    let now = std::time::Instant::now();

    if now >= self.next_throw_snowball_instant {
      self.throw_snowball_sound.play_with_level_position(self.npc_base.canvas_size,
          level_offset_x, self.npc_base.position);

      let mut snowball = NpcBase::new(self.snowball_image, self.npc_base.canvas_size,
          self.npc_base.level_tile_size, self.npc_base.tile, 0.0);
      snowball.position = self.npc_base.position;
      snowball.velocity = self.snowball_velocity;
      snowball.acceleration = self.snowball_acceleration;
      self.snowballs.push(snowball);

      while self.next_throw_snowball_instant <= now {
        self.next_throw_snowball_instant += self.throw_snowball_period_duration;
      }
    }

    for snowball in &mut self.snowballs { snowball.do_logic(); }
    self.npc_base.do_logic();
  }

  fn check_collision_with_sleigh(&mut self, score: &mut ui::Score,
        level_offset_x: f64, sleigh: &mut sleigh::Sleigh) {
    if sleigh.invincible() || sleigh.shield() { return; }
    let mut collides = self.npc_base.collides_with_sleigh(level_offset_x, sleigh);

    if !collides {
      for snowball in &self.snowballs {
        if snowball.collides_with_sleigh(level_offset_x, sleigh) {
          collides = true;
          break;
        }
      }
    }

    if collides {
      self.collision_sound.play_with_position(self.npc_base.canvas_size, sleigh.position());
      score.add_damage_points(self.damage_points);
      sleigh.start_invincible();
    }
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level_offset_x: f64) {
    self.npc_base.draw(canvas, level_offset_x);

    for snowball in &self.snowballs {
      snowball.draw(canvas, level_offset_x);
    }
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }

  fn check_collision_with_sleigh_in_menu_mode(&self) -> bool {
    return false;
  }
}

impl<'a> Snowman<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, level: &level::Level,
        tile: (usize, usize)) -> Snowman<'a> {
    let mut stars = Vec::new();

    for _ in 0 .. 20 {
      let mut star = sleigh::Star::new(asset_library);
      star.set_small_probability(0.5);
      star.set_min_offset(Point::new(-12.0, -32.0));
      star.set_max_offset(Point::new(-7.0, -27.0));
      stars.push(star);
    }

    return Snowman{
      npc_base: NpcBase::new(asset_library.get_image("snowman"),
          level.canvas_size(), level.tile_size(), tile, 0.0),
      launch_sound: asset_library.get_sound("snowmanLaunch"),
      collision_sound: asset_library.get_sound("sleighCollidedWithNpc"),

      launched: false,
      stars: stars,

      launch_velocity: Point::new(-100.0, -150.0),
      launch_frame_speed: 8.0,
      damage_points: 20.0,
    };
  }
}

impl<'a> Npc for Snowman<'a> {
  fn do_logic(&mut self, level_offset_x: f64, level_scroll_speed_x: f64, sleigh: &sleigh::Sleigh) {
    if self.launched {
      for star in &mut self.stars {
        star.do_logic(self.npc_base.position, self.npc_base.size, false);
      }
    } else {
      let sleigh_same_y_as_sleigh_seconds = (sleigh.position().y() - self.npc_base.position.y())
          / self.launch_velocity.y();
      let future_sleigh_position_x = level_offset_x + sleigh.position().x() + sleigh.size().x() / 2.0
          + sleigh_same_y_as_sleigh_seconds * level_scroll_speed_x;
      let future_snowman_position_x = self.npc_base.position.x() + self.npc_base.size.x() / 2.0
          + sleigh_same_y_as_sleigh_seconds * self.launch_velocity.x();

      if future_sleigh_position_x >= future_snowman_position_x {
        self.launch_sound.play_with_level_position(self.npc_base.canvas_size, level_offset_x,
            self.npc_base.position);
        self.launched = true;
        self.npc_base.velocity = self.launch_velocity;
        self.npc_base.frame_speed = self.launch_frame_speed;
      }
    }

    self.npc_base.do_logic();

    let last_frame = (self.npc_base.image.total_number_of_frames() as f64) - 1.0;
    if self.npc_base.frame > last_frame { self.npc_base.frame = last_frame; }
  }

  fn check_collision_with_sleigh(&mut self, score: &mut ui::Score,
        level_offset_x: f64, sleigh: &mut sleigh::Sleigh) {
    if !sleigh.invincible() && !sleigh.shield()
          && self.npc_base.collides_with_sleigh(level_offset_x, sleigh) {
      self.collision_sound.play_with_position(self.npc_base.canvas_size, sleigh.position());
      score.add_damage_points(self.damage_points);
      sleigh.start_invincible();
    }
  }

  fn draw(&self, canvas: &mut sdl2::render::WindowCanvas, level_offset_x: f64) {
    if self.launched {
      for star in &self.stars { star.draw(canvas, level_offset_x); }
    }

    self.npc_base.draw(canvas, level_offset_x);
  }

  fn tile(&self) -> (usize, usize) {
    return self.npc_base.tile;
  }

  fn z_order(&self) -> f64 {
    return 0.0;
  }

  fn check_collision_with_sleigh_in_menu_mode(&self) -> bool {
    return false;
  }
}
