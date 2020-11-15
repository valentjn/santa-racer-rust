/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use rand::Rng;

use crate::*;
use crate::asset::Point;

pub struct Landscape<'a> {
  image: &'a asset::Image<'a>,

  offset_x: f64,
  scrolling_resume_instant: std::time::Instant,
  last_update_instant: std::time::Instant,

  size: Point,
  scroll_speed_factor_x: f64,
}

pub struct Level<'a> {
  pub image: &'a asset::Image<'a>,
  pub tile_map: Vec<Vec<f64>>,
  npc_map: Vec<Vec<f64>>,
  pub canvas_size: Point,

  dog_sound: &'a asset::Sound,
  bell_sound: &'a asset::Sound,

  pub game_mode: game::GameMode,

  pub offset_x: f64,
  pub scroll_speed_x: f64,
  game_start_instant: std::time::Instant,
  scrolling_resume_instant: std::time::Instant,
  dog_sound_instant: std::time::Instant,
  bell_sound_instant: std::time::Instant,
  last_update_instant: std::time::Instant,

  pub tile_size: Point,
  number_of_tiles: (usize, usize),
  number_of_visible_tiles_x: usize,
  min_scroll_speed_x: f64,
  max_scroll_speed_x: f64,
  menu_scroll_speed_x: f64,
  dog_sound_volume: f64,
  bell_sound_volume: f64,
  min_dog_sound_duration: std::time::Duration,
  max_dog_sound_duration: std::time::Duration,
  min_bell_sound_duration: std::time::Duration,
  max_bell_sound_duration: std::time::Duration,
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
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>) -> Landscape {
    let image = asset_library.get_image("landscape");

    return Landscape{
      image: image,

      offset_x: 0.0,
      scrolling_resume_instant: std::time::Instant::now(),
      last_update_instant: std::time::Instant::now(),

      size: image.size(),
      scroll_speed_factor_x: 0.1,
    };
  }

  pub fn start_game(&mut self, game_start_instant: std::time::Instant) {
    self.offset_x = 0.0;
    self.scrolling_resume_instant = game_start_instant;
  }

  pub fn start_menu(&mut self) {
    self.offset_x = 0.0;
    self.scrolling_resume_instant = std::time::Instant::now();
  }

  pub fn pause_scrolling(&mut self, scrolling_resume_instant: std::time::Instant) {
    self.scrolling_resume_instant = scrolling_resume_instant;
  }

  pub fn do_logic(&mut self, level: &level::Level) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = (now - self.last_update_instant).as_secs_f64();

    let scroll_speed_x = self.scroll_speed_factor_x * level.scroll_speed_x;

    if now > self.scrolling_resume_instant {
      self.offset_x = (self.offset_x + seconds_since_last_update * scroll_speed_x) %
          self.size.x;
    }

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
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, canvas_size: Point) -> Level {
    let image = asset_library.get_image("level");
    let tile_size = image.size();
    let tile_map = Level::convert_data_to_map(
      asset_library.get_data("levelTileMap").to_vec());
    let npc_map = Level::convert_data_to_map(
      asset_library.get_data("levelNpcMap").to_vec());

    assert!(tile_map.len() == npc_map.len(),
        "Lengths of background and foreground object maps are not equal");
    assert!(tile_map.len() > 0, "Background object map is empty");

    let number_of_tiles_x = tile_map[0].len();
    let number_of_tiles_y = tile_map.len();

    for tile_y in 0 .. number_of_tiles_y {
      assert!(tile_map[tile_y].len() == number_of_tiles_x,
          "Rows of background object map do not have equal length");
      assert!(npc_map[tile_y].len() == number_of_tiles_x,
          "Rows of foreground object map do not have equal length");
    }

    let min_dog_sound_duration = std::time::Duration::from_millis(10000);
    let max_dog_sound_duration = std::time::Duration::from_millis(20000);
    let min_bell_sound_duration = std::time::Duration::from_millis(10000);
    let max_bell_sound_duration = std::time::Duration::from_millis(20000);
    let now = std::time::Instant::now();

    return Level{
      image: image,
      tile_map: tile_map,
      npc_map: npc_map,
      canvas_size: canvas_size,

      dog_sound: asset_library.get_sound("dog"),
      bell_sound: asset_library.get_sound("bell"),

      game_mode: game::GameMode::Menu,

      offset_x: 0.0,
      scroll_speed_x: 0.0,
      game_start_instant: now,
      scrolling_resume_instant: now,
      dog_sound_instant: now + rand::thread_rng().gen_range(
          min_dog_sound_duration, max_dog_sound_duration),
      bell_sound_instant: now + rand::thread_rng().gen_range(
          min_bell_sound_duration, max_bell_sound_duration),
      last_update_instant: now,

      tile_size: tile_size,
      number_of_tiles: (number_of_tiles_x, number_of_tiles_y),
      number_of_visible_tiles_x: (canvas_size.x / tile_size.x + 1.0) as usize,
      min_scroll_speed_x: 40.0,
      max_scroll_speed_x: 160.0,
      menu_scroll_speed_x: 40.0,
      dog_sound_volume: 0.5,
      bell_sound_volume: 0.5,
      min_dog_sound_duration: min_dog_sound_duration,
      max_dog_sound_duration: max_dog_sound_duration,
      min_bell_sound_duration: min_bell_sound_duration,
      max_bell_sound_duration: max_bell_sound_duration,
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

  pub fn start_game(&mut self, game_start_instant: std::time::Instant) {
    self.game_mode = game::GameMode::Running;
    self.offset_x = 0.0;
    self.game_start_instant = game_start_instant;
    self.scrolling_resume_instant = game_start_instant;
  }

  pub fn start_menu(&mut self) {
    self.game_mode = game::GameMode::Menu;
    self.offset_x = 0.0;
    self.scrolling_resume_instant = std::time::Instant::now();
  }

  pub fn pause_scrolling(&mut self, scrolling_resume_instant: std::time::Instant) {
    self.scrolling_resume_instant = scrolling_resume_instant;
  }

  pub fn do_logic(&mut self, sleigh: &sleigh::Sleigh) {
    let now = std::time::Instant::now();
    let seconds_since_last_update = (now - self.last_update_instant).as_secs_f64();

    if self.game_mode == game::GameMode::Menu {
      self.scroll_speed_x = self.menu_scroll_speed_x;
    } else {
      self.scroll_speed_x = self.min_scroll_speed_x + sleigh.position.x
          / (self.canvas_size.x - sleigh.size.x)
          * (self.max_scroll_speed_x - self.min_scroll_speed_x);
    }

    if now >= self.scrolling_resume_instant {
      self.offset_x += seconds_since_last_update * self.scroll_speed_x;
    }

    if now >= self.dog_sound_instant {
      self.dog_sound.play_with_volume(self.dog_sound_volume);
      self.dog_sound_instant = now + rand::thread_rng().gen_range(
          self.min_dog_sound_duration, self.max_dog_sound_duration);
    }

    if now >= self.bell_sound_instant {
      self.bell_sound.play_with_volume(self.bell_sound_volume);
      self.bell_sound_instant = now + rand::thread_rng().gen_range(
          self.min_bell_sound_duration, self.max_bell_sound_duration);
    }

    self.last_update_instant = now;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>) {
    for (tile_x, tile_y) in self.visible_tiles_iter() {
      let frame = self.tile_map[tile_y][tile_x];
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
    let max_tile_y = self.tile_map.len();

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
