/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use rand::Rng;

use crate::*;
use crate::asset::Point;

pub struct Sleigh<'a> {
  sleigh_image: &'a asset::Image<'a>,
  reindeer_image: &'a asset::Image<'a>,
  electrocuted_sleigh_image: &'a asset::Image<'a>,
  electrocuted_reindeer_image: &'a asset::Image<'a>,
  shield_image: &'a asset::Image<'a>,
  canvas_size: asset::Point,

  game_mode: game::GameMode,

  size: Point,
  position: Point,
  velocity: Point,
  velocity_point1: Point,
  velocity_point2: Point,
  velocity_x_instant1: std::time::Instant,
  velocity_x_instant2: std::time::Instant,
  velocity_y_instant1: std::time::Instant,
  velocity_y_instant2: std::time::Instant,
  sleigh_frame: f64,
  reindeer_frame: f64,
  shield_frame: f64,
  counting_down: bool,
  bonus: bool,
  shield: bool,
  drunk: bool,
  invincible: bool,
  immobile: bool,
  electrocuted: bool,
  countdown_counter: i32,
  invincible_blink: bool,
  game_start_instant: std::time::Instant,
  last_gift_instant: std::time::Instant,
  bonus_reset_instant: std::time::Instant,
  shield_reset_instant: std::time::Instant,
  drunk_reset_instant: std::time::Instant,
  invincible_reset_instant: std::time::Instant,
  immobile_reset_instant: std::time::Instant,
  electrocuted_reset_instant: std::time::Instant,
  menu_start_instant: std::time::Instant,
  last_update_instant: std::time::Instant,

  chimneys: Vec<gift::Chimney>,
  gifts: Vec<gift::Gift<'a>>,
  stars: Vec<Star<'a>>,

  max_velocity: Point,
  max_acceleration: Point,
  reindeer_offset: Point,
  electrocuted_offset: Point,
  shield_offset: Point,
  countdown_counter_offset_x: f64,
  frame_speed: f64,
  shield_frame_speed: f64,
  new_gift_wait_duration: std::time::Duration,
  bonus_duration: std::time::Duration,
  shield_duration: std::time::Duration,
  drunk_duration: std::time::Duration,
  invincible_duration: std::time::Duration,
  immobile_duration: std::time::Duration,
  electrocuted_duration: std::time::Duration,
  invincible_blink_period_duration: std::time::Duration,
  menu_period: Point,
  menu_offset_angle: Point,
  menu_min_position: Point,
  menu_max_position: Point,
  game_start_position: Point,
}

pub struct Star<'a> {
  image: &'a asset::Image<'a>,
  small_image: &'a asset::Image<'a>,
  drunk_image: &'a asset::Image<'a>,
  small_drunk_image: &'a asset::Image<'a>,

  position: Point,
  frame: f64,
  max_frame: f64,
  small: bool,
  drunk: bool,
  small_probability: f64,
  last_update_instant: std::time::Instant,

  min_offset: Point,
  max_offset: Point,
  frame_speed: f64,
  max_max_frame: f64,
}

impl<'a> Sleigh<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, canvas_size: Point) -> Sleigh<'a> {
    let sleigh_image = asset_library.get_image("sleigh");
    let reindeer_image = asset_library.get_image("reindeer");
    let reindeer_offset = Point::new(10.0, 3.0);
    let size = Point::new(sleigh_image.height() + reindeer_image.height() +
        reindeer_offset.x, sleigh_image.height());
    let now = std::time::Instant::now();
    let mut stars: Vec<Star<'a>> = Vec::new();

    for _ in 0 .. 67 { stars.push(Star::new(asset_library)); }

    return Sleigh{
      sleigh_image: sleigh_image,
      reindeer_image: reindeer_image,
      electrocuted_sleigh_image: asset_library.get_image("electrocutedSleigh"),
      electrocuted_reindeer_image: asset_library.get_image("electrocutedReindeer"),
      shield_image: asset_library.get_image("shield"),
      canvas_size: canvas_size,

      game_mode: game::GameMode::Menu,

      size: size,
      position: Point::zero(),
      velocity: Point::zero(),
      velocity_point1: Point::zero(),
      velocity_point2: Point::zero(),
      velocity_x_instant1: std::time::Instant::now(),
      velocity_x_instant2: std::time::Instant::now(),
      velocity_y_instant1: std::time::Instant::now(),
      velocity_y_instant2: std::time::Instant::now(),
      sleigh_frame: 0.0,
      reindeer_frame: 0.0,
      shield_frame: 0.0,
      counting_down: false,
      bonus: false,
      shield: false,
      drunk: false,
      invincible: false,
      immobile: false,
      electrocuted: false,
      countdown_counter: 0,
      invincible_blink: false,
      game_start_instant: now,
      last_gift_instant: now,
      bonus_reset_instant: now,
      shield_reset_instant: now,
      drunk_reset_instant: now,
      invincible_reset_instant: now,
      immobile_reset_instant: now,
      electrocuted_reset_instant: now,
      menu_start_instant: now,
      last_update_instant: now,

      chimneys: Sleigh::load_chimneys(asset_library),
      gifts: Vec::new(),
      stars: stars,

      max_velocity: Point::new(200.0, 200.0),
      max_acceleration: Point::new(1000.0, 1000.0),
      reindeer_offset: reindeer_offset,
      electrocuted_offset: Point::new(-3.0, -2.0),
      shield_offset: Point::new(-12.0, -17.0),
      countdown_counter_offset_x: -10.0,
      frame_speed: 14.0,
      shield_frame_speed: 8.0,
      new_gift_wait_duration: std::time::Duration::from_millis(250),
      bonus_duration: std::time::Duration::from_millis(15000),
      shield_duration: std::time::Duration::from_millis(15000),
      drunk_duration: std::time::Duration::from_millis(15000),
      invincible_duration: std::time::Duration::from_millis(3000),
      immobile_duration: std::time::Duration::from_millis(5000),
      electrocuted_duration: std::time::Duration::from_millis(1000),
      invincible_blink_period_duration: std::time::Duration::from_millis(500),
      menu_period: Point::new(30.0, 20.0),
      menu_offset_angle: Point::new(rand::thread_rng().gen_range(0.0, 2.0 * std::f64::consts::PI),
        rand::thread_rng().gen_range(0.0, 2.0 * std::f64::consts::PI)),
      menu_min_position: Point::new(50.0, 50.0),
      menu_max_position: Point::new(450.0, 200.0),
      game_start_position: Point::new(50.0, 100.0),
    };
  }

  fn load_chimneys(asset_library: &'a asset::AssetLibrary) ->
        Vec<gift::Chimney> {
    let data = asset_library.get_data("chimneys");
    let mut chimneys = Vec::new();
    assert!(data.len() % 4 == 0, "Length of chimney hit box data not divisible by 4");

    for i in 0 .. data.len() / 4 {
      chimneys.push(gift::Chimney::new(Point::new(data[4 * i], data[4 * i + 1]),
          Point::new(data[4 * i + 2], 20.0), data[4 * i + 3]));
    }

    return chimneys;
  }

  pub fn start_game(&mut self, game_start_instant: std::time::Instant) {
    self.game_mode = game::GameMode::Running;
    self.position = self.game_start_position;
    self.counting_down = true;
    self.drunk = false;
    self.invincible = false;
    self.immobile = false;
    self.game_start_instant = game_start_instant;
    for star in &mut self.stars { star.reset_in_between(self.position, self.size, self.drunk); }
  }

  pub fn start_menu(&mut self) {
    self.game_mode = game::GameMode::Menu;
    self.counting_down = false;
    self.drunk = false;
    self.invincible = false;
    self.immobile = false;
    self.menu_start_instant = std::time::Instant::now();
    for star in &mut self.stars { star.frame = -1.0; }
  }

  pub fn check_keyboard_state(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
    if (self.game_mode == game::GameMode::Menu) || self.immobile || self.counting_down { return; }
    let drunk_factor = if self.drunk { -1.0 } else { 1.0 };

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
    let now = std::time::Instant::now();
    let velocity_x = self.get_velocity_x(now);

    self.velocity_x_instant1 = now;
    self.velocity_point1.x = velocity_x;

    let target_velocity_x = sign * self.max_velocity.x;
    self.velocity_x_instant2 = now + std::time::Duration::from_secs_f64(
        (target_velocity_x - velocity_x).abs() / self.max_acceleration.x);
    self.velocity_point2.x = target_velocity_x;
  }

  fn update_velocity_y(&mut self, sign: f64) {
    let now = std::time::Instant::now();
    let velocity_y = self.get_velocity_y(now);

    self.velocity_y_instant1 = now;
    self.velocity_point1.y = velocity_y;

    let target_velocity_y = sign * self.max_velocity.y;
    self.velocity_y_instant2 = now + std::time::Duration::from_secs_f64(
        (target_velocity_y - velocity_y).abs() / self.max_acceleration.y);
    self.velocity_point2.y = target_velocity_y;
  }

  fn get_velocity_x(&self, instant: std::time::Instant) -> f64 {
    if instant <= self.velocity_x_instant1 {
      return self.velocity_point1.x;
    } else if instant >= self.velocity_x_instant2 {
      return self.velocity_point2.x;
    } else {
      return self.velocity_point1.x + (self.velocity_point2.x - self.velocity_point1.x)
          * (instant - self.velocity_x_instant1).as_secs_f64()
          / (self.velocity_x_instant2 - self.velocity_x_instant1).as_secs_f64();
    }
  }

  fn get_velocity_y(&self, instant: std::time::Instant) -> f64 {
    if instant <= self.velocity_y_instant1 {
      return self.velocity_point1.y;
    } else if instant >= self.velocity_y_instant2 {
      return self.velocity_point2.y;
    } else {
      return self.velocity_point1.y + (self.velocity_point2.y - self.velocity_point1.y)
          * (instant - self.velocity_y_instant1).as_secs_f64()
          / (self.velocity_y_instant2 - self.velocity_y_instant1).as_secs_f64();
    }
  }

  pub fn drop_gift(&mut self, asset_library: &'a asset::AssetLibrary, level: &level::Level,
        game_difficulty: game::GameDifficulty) {
    let now = std::time::Instant::now();
    if now - self.last_gift_instant < self.new_gift_wait_duration { return; }
    self.gifts.push(gift::Gift::new(
        asset_library, level, self, self.canvas_size, game_difficulty));
    self.last_gift_instant = now;
  }

  pub fn start_bonus(&mut self) {
    self.bonus = true;
    self.bonus_reset_instant = std::time::Instant::now() + self.bonus_duration;
  }

  pub fn start_shield(&mut self) {
    self.shield = true;
    self.shield_reset_instant = std::time::Instant::now() + self.shield_duration;
    self.shield_frame = 0.0;
  }

  pub fn start_drunk(&mut self) {
    self.drunk = true;
    self.drunk_reset_instant = std::time::Instant::now() + self.drunk_duration;
  }

  pub fn start_invincible(&mut self) {
    self.invincible = true;
    self.invincible_reset_instant = std::time::Instant::now() + self.invincible_duration;
  }

  pub fn start_electrocuted(&mut self) {
    self.electrocuted = true;
    self.electrocuted_reset_instant = std::time::Instant::now() + self.electrocuted_duration;
  }

  pub fn start_invincible_and_immobile(&mut self) {
    self.invincible = true;
    self.immobile = true;
    self.invincible_reset_instant = std::time::Instant::now() + self.immobile_duration
        + self.invincible_duration;
    self.immobile_reset_instant = std::time::Instant::now() + self.immobile_duration;
    self.velocity = Point::new(0.0, -self.max_velocity.y);
    self.velocity_point1 = self.velocity;
    self.velocity_point2 = self.velocity;
  }

  pub fn do_logic(&mut self, score: &mut ui::Score, level: &mut level::Level) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = (now - self.last_update_instant).as_secs_f64();

    if self.counting_down && (now >= self.game_start_instant) { self.counting_down = false; }
    if self.bonus && (now >= self.bonus_reset_instant) { self.bonus = false; }
    if self.shield && (now >= self.shield_reset_instant) { self.shield = false; }
    if self.drunk && (now >= self.drunk_reset_instant) { self.drunk = false; }
    if self.invincible && (now >= self.invincible_reset_instant) { self.invincible = false; }
    if self.immobile && (now >= self.immobile_reset_instant) { self.immobile = false; }
    if self.electrocuted && (now >= self.electrocuted_reset_instant) { self.electrocuted = false; }

    if self.game_mode == game::GameMode::Menu {
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
      self.velocity.x = self.get_velocity_x(now);
      self.velocity.y = self.get_velocity_y(now);

      if ((self.velocity.x < 0.0) && (self.position.x <= 0.0))
            || ((self.velocity.x > 0.0) && (self.position.x >= self.canvas_size.x - self.size.x)) {
        self.velocity.x = 0.0;
      }

      if ((self.velocity.y < 0.0) && (self.position.y <= 0.0))
            || ((self.velocity.y > 0.0) && (self.position.y >= self.canvas_size.y - self.size.y)) {
        self.velocity.y = 0.0;
      }

      self.position.x = (self.position.x + seconds_since_last_update * self.velocity.x)
          .max(0.0).min(self.canvas_size.x - self.size.x);
      self.position.y = (self.position.y + seconds_since_last_update * self.velocity.y)
          .max(0.0).min(self.canvas_size.y - self.size.y);
    }

    if !self.immobile {
      self.sleigh_frame += seconds_since_last_update * self.frame_speed;
      self.reindeer_frame = self.sleigh_frame +
          (self.reindeer_image.total_number_of_frames() as f64) / 2.0;
    }

    if self.shield {
      self.shield_frame += seconds_since_last_update * self.frame_speed;
    }

    if self.counting_down {
      self.countdown_counter = (self.game_start_instant - now).as_secs_f64().ceil() as i32;
    }

    if self.invincible {
      self.invincible_blink = ((self.invincible_reset_instant - now).as_secs_f64()
          / self.invincible_blink_period_duration.as_secs_f64()) % 1.0 >= 0.5;
    }

    {
      let mut i = 0;

      while i < self.gifts.len() {
        self.gifts[i].do_logic(score, level, &self.chimneys);

        if self.gifts[i].mode() == gift::GiftMode::CanBeDeleted {
          self.gifts.remove(i);
        } else {
          i += 1;
        }
      }
    }

    for star in &mut self.stars {
      star.do_logic(self.position, self.size, self.drunk);
    }

    self.last_update_instant = now;
  }

  pub fn collides_with_image(&self, image: &asset::Image, position: Point, frame: f64) -> bool {
    return self.sleigh_image.collides(self.position, self.sleigh_frame, image,
        position, frame) || self.reindeer_image.collides(Point::new(
          self.position.x + self.sleigh_image.width() + self.reindeer_offset.x,
          self.position.y + self.reindeer_offset.y),
        self.sleigh_frame, image, position, frame);
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>, font: &ui::Font,
        level: &level::Level) {
    if self.invincible_blink { return; }

    if self.electrocuted {
      let electrocuted_sleigh_offset = Point::new(
          self.electrocuted_offset.x
          - (self.electrocuted_sleigh_image.width() - self.sleigh_image.width()) / 2.0,
          self.electrocuted_offset.y
          - (self.electrocuted_sleigh_image.height() - self.sleigh_image.height()) / 2.0);
      let electrocuted_reindeer_offset = Point::new(
          self.electrocuted_offset.x
          - (self.electrocuted_reindeer_image.width() - self.reindeer_image.width()) / 2.0,
          self.electrocuted_offset.y
          - (self.electrocuted_reindeer_image.height() - self.reindeer_image.height()) / 2.0);
      self.electrocuted_sleigh_image.draw(canvas, Point::new(
            self.position.x + electrocuted_sleigh_offset.x,
            self.position.y + electrocuted_sleigh_offset.y),
          self.sleigh_frame);
      self.electrocuted_reindeer_image.draw(canvas, Point::new(
            self.position.x + self.sleigh_image.width() + self.reindeer_offset.x
            + electrocuted_reindeer_offset.x,
            self.position.y + self.reindeer_offset.y + electrocuted_reindeer_offset.y),
          self.reindeer_frame);
      self.electrocuted_reindeer_image.draw(canvas, Point::new(
            self.position.x + self.sleigh_image.width() + electrocuted_reindeer_offset.x,
            self.position.y + self.reindeer_offset.y + electrocuted_reindeer_offset.y),
          self.reindeer_frame);
    }

    self.sleigh_image.draw(canvas, self.position, self.sleigh_frame);
    self.reindeer_image.draw(canvas,
        Point::new(self.position.x + self.sleigh_image.width() + self.reindeer_offset.x,
          self.position.y + self.reindeer_offset.y), self.reindeer_frame);
    self.reindeer_image.draw(canvas,
        Point::new(self.position.x + self.sleigh_image.width(),
          self.position.y + self.reindeer_offset.y), self.reindeer_frame);

    for gift in &self.gifts { gift.draw(canvas, level); }
    for star in &self.stars { star.draw(canvas, 0.0); }

    if self.shield {
      self.shield_image.draw(canvas, Point::new(self.position.x + self.shield_offset.x,
          self.position.y + self.shield_offset.y), self.shield_frame);
    }

    if self.counting_down {
      font.draw(canvas, Point::new(self.position.x + self.countdown_counter_offset_x,
          self.position.y + self.size.y / 2.0), format!("{}", self.countdown_counter),
          ui::Alignment::CenterRight);
    }
  }

  pub fn size(&self) -> Point {
    return self.size;
  }

  pub fn position(&self) -> Point {
    return self.position;
  }

  pub fn velocity(&self) -> Point {
    return self.velocity;
  }

  pub fn counting_down(&self) -> bool {
    return self.counting_down;
  }

  pub fn bonus(&self) -> bool {
    return self.bonus;
  }

  pub fn shield(&self) -> bool {
    return self.shield;
  }

  pub fn invincible(&self) -> bool {
    return self.invincible;
  }

  pub fn immobile(&self) -> bool {
    return self.immobile;
  }

  pub fn immobile_duration(&self) -> std::time::Duration {
    return self.immobile_duration;
  }
}

impl<'a> Star<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>) -> Star<'a> {
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
      small_probability: 0.5,
      last_update_instant: std::time::Instant::now(),

      min_offset: Point::new(-150.0, -10.0),
      max_offset: Point::new(-10.0, 0.0),
      frame_speed: 34.0,
      max_max_frame: 30.0,
    };
  }

  pub fn do_logic(&mut self, sleigh_position: Point, sleigh_size: Point, drunk: bool) {
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
    let offset_x = if self.min_offset.x < self.max_offset.x {
          rand::thread_rng().gen_range(self.min_offset.x, self.max_offset.x)
        } else {
          self.min_offset.x
        };
    let offset_y = if self.min_offset.y < self.max_offset.y {
          rand::thread_rng().gen_range(self.min_offset.y, self.max_offset.y)
        } else {
          self.min_offset.y
        };

    self.position = Point::new(sleigh_position.x + sleigh_size.x + offset_x,
        sleigh_position.y + sleigh_size.y + offset_y);
    self.frame = 0.0;
    self.max_frame = rand::thread_rng().gen_range(
        self.image.total_number_of_frames() as f64, self.max_max_frame);
    self.small = rand::thread_rng().gen_range(0.0, 1.0) < self.small_probability;
    self.drunk = drunk;
    self.last_update_instant = std::time::Instant::now();
  }

  fn reset_in_between(&mut self, sleigh_position: Point, sleigh_size: Point, drunk: bool) {
    self.reset_from_beginning(sleigh_position, sleigh_size, drunk);
    self.frame = rand::thread_rng().gen_range(0.0, self.max_max_frame);
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>, level_offset_x: f64) {
    if self.frame >= self.image.total_number_of_frames() as f64 { return; }

    let image = if self.small { if self.drunk { self.small_drunk_image } else { self.small_image } }
        else { if self.drunk { self.drunk_image } else { self.image } };
    let position = Point::new(self.position.x - level_offset_x, self.position.y);

    image.draw(canvas, position, self.frame);
  }

  pub fn small_probability(&self) -> f64 {
    return self.small_probability;
  }

  pub fn set_small_probability(&mut self, small_probability: f64) {
    self.small_probability = small_probability;
  }

  pub fn min_offset(&self) -> Point {
    return self.min_offset;
  }

  pub fn set_min_offset(&mut self, min_offset: Point) {
    self.min_offset = min_offset;
  }

  pub fn max_offset(&self) -> Point {
    return self.max_offset;
  }

  pub fn set_max_offset(&mut self, max_offset: Point) {
    self.max_offset = max_offset;
  }
}
