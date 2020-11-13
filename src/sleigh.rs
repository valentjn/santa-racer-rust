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

  pub game_mode: game::Mode,

  pub size: Point,
  pub position: Point,
  pub velocity: Point,
  acceleration: Point,
  sleigh_frame: f64,
  reindeer_frame: f64,
  counting_down: bool,
  drunk: bool,
  invincible: bool,
  immobile: bool,
  countdown_counter: i32,
  invincible_blink: bool,
  game_start_instant: std::time::Instant,
  invincible_reset_instant: std::time::Instant,
  immobile_reset_instant: std::time::Instant,
  menu_start_instant: std::time::Instant,
  last_update_instant: std::time::Instant,

  stars: Vec<Star<'a>>,

  max_velocity: Point,
  reindeer_offset: Point,
  countdown_counter_offset_x: f64,
  frame_speed: f64,
  level_collision_damage_points: f64,
  invincible_duration: std::time::Duration,
  immobile_duration: std::time::Duration,
  invincible_blink_periods: i32,
  menu_period: Point,
  menu_offset_angle: Point,
  menu_min_position: Point,
  menu_max_position: Point,
  game_start_position: Point,
}

struct Star<'a> {
  image: &'a assets::Image<'a>,
  small_image: &'a assets::Image<'a>,
  drunk_image: &'a assets::Image<'a>,
  small_drunk_image: &'a assets::Image<'a>,

  position: Point,
  frame: f64,
  max_frame: f64,
  small: bool,
  drunk: bool,
  last_update_instant: std::time::Instant,

  min_offset: Point,
  max_offset: Point,
  frame_speed: f64,
  max_max_frame: f64,
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
    let now = std::time::Instant::now();
    let mut stars: Vec<Star<'a>> = Vec::new();

    for _ in 0 .. 67 {
      stars.push(Star::new(asset_library));
    }

    return Sleigh{
      sleigh_image: sleigh_image,
      reindeer_image: reindeer_image,
      canvas_size: canvas_size,

      collided_with_level_sound1: asset_library.get_sound("sleighCollidedWithLevel1"),
      collided_with_level_sound2: asset_library.get_sound("sleighCollidedWithLevel2"),

      game_mode: game::Mode::Menu,

      size: size,
      position: Point::zero(),
      velocity: Point::zero(),
      acceleration: Point::new(25.0, 25.0),
      sleigh_frame: 0.0,
      reindeer_frame: 0.0,
      counting_down: false,
      drunk: false,
      invincible: false,
      immobile: false,
      countdown_counter: 0,
      invincible_blink: false,
      game_start_instant: now,
      invincible_reset_instant: now,
      immobile_reset_instant: now,
      menu_start_instant: now,
      last_update_instant: now,

      stars: stars,

      max_velocity: Point::new(200.0, 200.0),
      reindeer_offset: reindeer_offset,
      countdown_counter_offset_x: -10.0,
      frame_speed: 13.0,
      level_collision_damage_points: 50.0,
      invincible_duration: std::time::Duration::from_millis(8000),
      immobile_duration: std::time::Duration::from_millis(5000),
      invincible_blink_periods: 16,
      menu_period: Point::new(30.0, 20.0),
      menu_offset_angle: Point::new(rand::thread_rng().gen_range(0.0, 2.0 * std::f64::consts::PI),
        rand::thread_rng().gen_range(0.0, 2.0 * std::f64::consts::PI)),
      menu_min_position: Point::new(50.0, 50.0),
      menu_max_position: Point::new(450.0, 200.0),
      game_start_position: Point::new(50.0, 100.0),
    };
  }

  pub fn start_game(&mut self, game_start_instant: std::time::Instant) {
    self.game_mode = game::Mode::Running;
    self.position = self.game_start_position;
    self.counting_down = true;
    self.drunk = false;
    self.invincible = false;
    self.immobile = false;
    self.game_start_instant = game_start_instant;
    for star in &mut self.stars { star.reset_in_between(self.position, self.size, self.drunk); }
  }

  pub fn start_menu(&mut self) {
    self.game_mode = game::Mode::Menu;
    self.counting_down = false;
    self.drunk = false;
    self.invincible = false;
    self.immobile = false;
    self.menu_start_instant = std::time::Instant::now();
    for star in &mut self.stars { star.frame = -1.0; }
  }

  pub fn check_keyboard_state(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
    if (self.game_mode == game::Mode::Menu) || self.immobile || self.counting_down { return; }
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
    let seconds_since_last_update = (now - self.last_update_instant).as_secs_f64();

    if self.counting_down && (now >= self.game_start_instant) { self.counting_down = false; }

    if self.game_mode == game::Mode::Menu {
      let seconds_since_menu_start = now.duration_since(self.menu_start_instant).as_secs_f64();
      self.position.x = (f64::sin(seconds_since_menu_start / self.menu_period.x
              * 2.0 * std::f64::consts::PI + self.menu_offset_angle.x) + 1.0)
            * ((self.menu_max_position.x - self.menu_min_position.x) / 2.0)
            + self.menu_min_position.x;
      self.position.y = (f64::sin(seconds_since_menu_start / self.menu_period.y
              * 2.0 * std::f64::consts::PI + self.menu_offset_angle.y) + 1.0)
            * ((self.menu_max_position.y - self.menu_min_position.y) / 2.0)
            + self.menu_min_position.y;
    } else if self.counting_down {
    } else {
      self.position.x = (self.position.x
          + (seconds_since_last_update * (self.velocity.x as f64)) as f64)
          .max(0.0).min(self.canvas_size.x - self.size.x);
      self.position.y = (self.position.y
          + (seconds_since_last_update * (self.velocity.y as f64)) as f64)
          .max(0.0).min(self.canvas_size.y - self.size.y);
    }

    if !self.immobile {
      self.sleigh_frame += seconds_since_last_update * self.frame_speed;
      self.reindeer_frame = self.sleigh_frame +
          (self.reindeer_image.total_number_of_frames() as f64) / 2.0;
    }

    if (self.game_mode == game::Mode::Menu) || self.invincible || self.immobile
          || self.counting_down {
      if self.invincible && (now >= self.invincible_reset_instant) { self.invincible = false; }
      if self.immobile && (now >= self.immobile_reset_instant) { self.immobile = false; }
    } else if self.collides_with_level(level) {
      let collided_with_level_sound = match rand::thread_rng().gen_range(0, 2) {
        0 => self.collided_with_level_sound1,
        _ => self.collided_with_level_sound2,
      };

      collided_with_level_sound.play_with_position(level, self.position.x);
      self.invincible = true;
      self.immobile = true;
      self.invincible_reset_instant = now + self.invincible_duration;
      self.immobile_reset_instant = now + self.immobile_duration;
      self.velocity = Point::new(0.0, -self.max_velocity.y);
      score.add_damage_points(self.level_collision_damage_points);
      landscape.pause_scrolling(now + self.immobile_duration);
      level.pause_scrolling(now + self.immobile_duration);
    }

    if self.counting_down {
      self.countdown_counter = (self.game_start_instant - now).as_secs_f64().ceil() as i32;
    }

    if self.invincible {
      self.invincible_blink = (((self.invincible_reset_instant - now).as_secs_f64()
            / self.invincible_duration.as_secs_f64())
          * (self.invincible_blink_periods as f64)) % 1.0 >= 0.5;
    }

    for star in &mut self.stars {
      star.do_logic(self.position, self.size, self.drunk);
    }

    self.last_update_instant = now;
  }

  fn collides_with_level(&self, level: &level::Level) -> bool {
    for (tile_x, tile_y) in level.visible_tiles_iter() {
      let tile_frame = level.tile_map[tile_y][tile_x];
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
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>, font: &ui::Font) {
    if self.invincible_blink { return; }

    let mut position = self.position;
    self.sleigh_image.draw(canvas, position, self.sleigh_frame);
    position.x += self.sleigh_image.width() + self.reindeer_offset.x;
    position.y += self.reindeer_offset.y;
    self.reindeer_image.draw(canvas, position, self.reindeer_frame);
    position.x -= self.reindeer_offset.x;
    self.reindeer_image.draw(canvas, position, self.reindeer_frame);

    for star in &self.stars { star.draw(canvas); }

    if self.counting_down {
      font.draw(canvas, Point::new(self.position.x + self.countdown_counter_offset_x,
          self.position.y + self.size.y / 2.0), format!("{}", self.countdown_counter),
          ui::Alignment::CenterRight);
    }
  }
}

impl<'a> Star<'a> {
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>) -> Star<'a> {
    return Star{
      image: asset_library.get_image("star"),
      small_image: asset_library.get_image("smallStar"),
      drunk_image: asset_library.get_image("drunkStar"),
      small_drunk_image: asset_library.get_image("smallDrunkStar"),

      position: Point::zero(),
      frame: -1.0,
      max_frame: 0.0,
      small: false,
      drunk: false,
      last_update_instant: std::time::Instant::now(),

      min_offset: Point::new(0.0, 0.0),
      max_offset: Point::new(140.0, 10.0),
      frame_speed: 34.0,
      max_max_frame: 30.0,
    };
  }

  fn do_logic(&mut self, sleigh_position: Point, sleigh_size: Point, drunk: bool) {
    if self.frame == -1.0 { self.reset_in_between(sleigh_position, sleigh_size, drunk); }

    let now = std::time::Instant::now();
    let seconds_since_last_update = (now - self.last_update_instant).as_secs_f64();

    self.frame += seconds_since_last_update * self.frame_speed;

    if self.frame >= self.max_frame {
      self.reset_from_beginning(sleigh_position, sleigh_size, drunk);
    }

    self.last_update_instant = now;
  }

  fn reset_from_beginning(&mut self, sleigh_position: Point, sleigh_size: Point, drunk: bool) {
    self.position = Point::new(sleigh_position.x + sleigh_size.x
        - rand::thread_rng().gen_range(self.min_offset.x, self.max_offset.x),
        sleigh_position.y + sleigh_size.y
        - rand::thread_rng().gen_range(self.min_offset.y, self.max_offset.y));
    self.frame = 0.0;
    self.max_frame = rand::thread_rng().gen_range(
        self.image.total_number_of_frames() as f64, self.max_max_frame);
    self.small = rand::thread_rng().gen_range(0.0, 1.0) >= 0.5;
    self.drunk = drunk;
    self.last_update_instant = std::time::Instant::now();
  }

  fn reset_in_between(&mut self, sleigh_position: Point, sleigh_size: Point, drunk: bool) {
    self.reset_from_beginning(sleigh_position, sleigh_size, drunk);
    self.frame = rand::thread_rng().gen_range(0.0, self.max_max_frame);
  }

  fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    if self.frame >= self.image.total_number_of_frames() as f64 { return; }

    let image = if self.small { if self.drunk { self.small_drunk_image } else { self.small_image } }
        else { if self.drunk { self.drunk_image } else { self.image } };
    image.draw(canvas, self.position, self.frame);
  }
}
