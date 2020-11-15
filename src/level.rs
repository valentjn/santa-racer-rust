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
  sleigh_collided_with_tile_sound1: &'a asset::Sound,
  sleigh_collided_with_tile_sound2: &'a asset::Sound,

  pub game_mode: game::GameMode,

  pub offset_x: f64,
  pub scroll_speed_x: f64,
  game_start_instant: std::time::Instant,
  scrolling_resume_instant: std::time::Instant,
  dog_sound_instant: std::time::Instant,
  bell_sound_instant: std::time::Instant,
  last_update_instant: std::time::Instant,

  npcs: Vec<Box<dyn npc::Npc + 'a>>,

  pub tile_size: Point,
  number_of_tiles: (usize, usize),
  number_of_visible_tiles_x: usize,
  start_offset_x: f64,
  min_scroll_speed_x: f64,
  max_scroll_speed_x: f64,
  menu_scroll_speed_x: f64,
  dog_sound_volume: f64,
  bell_sound_volume: f64,
  min_dog_sound_duration: std::time::Duration,
  max_dog_sound_duration: std::time::Duration,
  min_bell_sound_duration: std::time::Duration,
  max_bell_sound_duration: std::time::Duration,
  sleigh_collided_with_tile_damage_points: f64,
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

    let start_offset_x = -200.0;

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
      sleigh_collided_with_tile_sound1: asset_library.get_sound("sleighCollidedWithLevelTile1"),
      sleigh_collided_with_tile_sound2: asset_library.get_sound("sleighCollidedWithLevelTile2"),

      game_mode: game::GameMode::Menu,

      offset_x: start_offset_x,
      scroll_speed_x: 0.0,
      game_start_instant: now,
      scrolling_resume_instant: now,
      dog_sound_instant: now + rand::thread_rng().gen_range(
          min_dog_sound_duration, max_dog_sound_duration),
      bell_sound_instant: now + rand::thread_rng().gen_range(
          min_bell_sound_duration, max_bell_sound_duration),
      last_update_instant: now,

      npcs: Vec::new(),

      tile_size: tile_size,
      number_of_tiles: (number_of_tiles_x, number_of_tiles_y),
      number_of_visible_tiles_x: (canvas_size.x / tile_size.x + 1.0) as usize,
      start_offset_x: start_offset_x,
      min_scroll_speed_x: 40.0,
      max_scroll_speed_x: 160.0,
      menu_scroll_speed_x: 40.0,
      dog_sound_volume: 0.5,
      bell_sound_volume: 0.5,
      min_dog_sound_duration: min_dog_sound_duration,
      max_dog_sound_duration: max_dog_sound_duration,
      min_bell_sound_duration: min_bell_sound_duration,
      max_bell_sound_duration: max_bell_sound_duration,
      sleigh_collided_with_tile_damage_points: 50.0,
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
    self.offset_x = self.start_offset_x;
    self.game_start_instant = game_start_instant;
    self.scrolling_resume_instant = game_start_instant;
    self.npcs.clear();
  }

  pub fn start_menu(&mut self) {
    self.game_mode = game::GameMode::Menu;
    self.offset_x = self.start_offset_x;
    self.scrolling_resume_instant = std::time::Instant::now();
    self.npcs.clear();
  }

  pub fn pause_scrolling(&mut self, scrolling_resume_instant: std::time::Instant) {
    self.scrolling_resume_instant = scrolling_resume_instant;
  }

  pub fn do_logic(&mut self, asset_library: &'a asset::AssetLibrary<'a>, score: &mut ui::Score,
        landscape: &mut level::Landscape, sleigh: &mut sleigh::Sleigh) {
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

    if (self.game_mode == game::GameMode::Running) && !sleigh.invincible && !sleigh.immobile
          && !sleigh.counting_down && self.sleigh_collides_with_tile(sleigh) {
      let collided_with_level_sound = match rand::thread_rng().gen_range(0, 2) {
        0 => self.sleigh_collided_with_tile_sound1,
        _ => self.sleigh_collided_with_tile_sound2,
      };

      collided_with_level_sound.play_with_position(self, sleigh.position.x);
      sleigh.collide_with_level_tile();
      score.add_damage_points(self.sleigh_collided_with_tile_damage_points);
      landscape.pause_scrolling(now + sleigh.immobile_duration);
      self.pause_scrolling(now + sleigh.immobile_duration);
    }

    let mut delete_npc: Vec<bool> = vec![true; self.npcs.len()];

    for (tile_x, tile_y) in self.visible_tiles_iter() {
      let frame = self.npc_map[tile_y][tile_x];
      if frame < 0.0 { continue; }
      let tile = (tile_x, tile_y);
      let mut npc_found = false;

      for (i, npc) in self.npcs.iter().enumerate() {
        if npc.tile() == tile {
          npc_found = true;
          delete_npc[i] = false;
          break;
        }
      }

      if npc_found { continue; }
      self.npcs.push(npc::new_npc(asset_library, self, tile, frame));
      delete_npc.push(false);
    }

    {
      let mut i = 0;

      while i < self.npcs.len() {
        if delete_npc[i] {
          self.npcs.remove(i);
          delete_npc.remove(i);
        } else {
          i += 1;
        }
      }
    }

    self.npcs.sort_unstable_by(|x, y| x.z_order().partial_cmp(&y.z_order()).expect(
        "Could not compare NPC z-orders"));

    for npc in &mut self.npcs { npc.do_logic(self.offset_x, sleigh); }

    self.last_update_instant = now;
  }

  fn sleigh_collides_with_tile(&self, sleigh: &sleigh::Sleigh) -> bool {
    for (tile_x, tile_y) in self.visible_tiles_iter() {
      let tile_frame = self.tile_map[tile_y][tile_x];
      if tile_frame < 0.0 { continue; }
      let tile_position = Point::new((tile_x as f64) * self.tile_size.x - self.offset_x,
          (tile_y as f64) * self.tile_size.y);
      if sleigh.collides_with_image(self.image, tile_position, tile_frame) { return true; }
    }

    return false;
  }

  pub fn draw(&self, canvas: &mut sdl2::render::WindowCanvas) {
    for npc in &self.npcs {
      if npc.z_order() < 0.0 { npc.draw(canvas, self); }
    }

    for (tile_x, tile_y) in self.visible_tiles_iter() {
      let frame = self.tile_map[tile_y][tile_x];
      if frame < 0.0 { continue; }
      let dst_point = Point::new((tile_x as f64) * self.tile_size.x - self.offset_x,
          (tile_y as f64) * self.tile_size.y);
      self.image.draw(canvas, dst_point, frame);
    }

    for npc in &self.npcs {
      if npc.z_order() >= 0.0 { npc.draw(canvas, self); }
    }
  }

  pub fn visible_tiles_iter(&self) -> TileIterator {
    let min_tile_x = (self.offset_x / self.tile_size.x - 1.0).max(0.0) as usize;
    let max_tile_x = (min_tile_x + self.number_of_visible_tiles_x + 2).min(self.number_of_tiles.0);
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
