/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::asset::Point;

pub struct Score<'a> {
  gift_image: &'a asset::Image<'a>,
  damage_image: &'a asset::Image<'a>,
  time_image: &'a asset::Image<'a>,
  canvas_size: asset::Point,

  game_mode: game::GameMode,

  gift_points: f64,
  damage_points: f64,
  remaining_duration: std::time::Duration,
  game_start_instant: std::time::Instant,
  last_update_instant: std::time::Instant,

  gift_position_x: f64,
  damage_position_x: f64,
  time_position_x: f64,
  margin_x: f64,
  position_y: f64,
}

pub struct HighscoreTable<'a> {
  background_image: asset::Image<'a>,
  canvas_size: asset::Point,

  game_mode: game::GameMode,

  size: Point,
  position: Point,
  inner_margin: Point,
  number_of_rows: f64,
}

pub struct Font<'a> {
  image: &'a asset::Image<'a>,
  characters: String,
  character_rects: Vec<sdl2::rect::Rect>,
  max_character_width: i32,
  height: f64,
}

#[derive(Clone, Copy)]
pub enum Alignment {
  TopLeft = 0,
  TopCenter = 1,
  TopRight = 2,
  CenterLeft = 3,
  Center = 4,
  CenterRight = 5,
  BottomLeft = 6,
  BottomCenter = 7,
  BottomRight = 8,
}

impl<'a> Score<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>, canvas_size: Point) -> Score<'a> {
    let gift_image = asset_library.get_image("giftScoreIcon");
    let now = std::time::Instant::now();

    return Score{
      gift_image: gift_image,
      damage_image: asset_library.get_image("damageScoreIcon"),
      time_image: asset_library.get_image("timeScoreIcon"),
      canvas_size: canvas_size,

      game_mode: game::GameMode::Menu,

      gift_points: 0.0,
      damage_points: 0.0,
      remaining_duration: std::time::Duration::from_millis(0),
      game_start_instant: now,
      last_update_instant: now,

      gift_position_x: 0.0,
      damage_position_x: 150.0,
      time_position_x: 535.0,
      margin_x: 35.0,
      position_y: gift_image.height() / 2.0,
    };
  }

  pub fn start_game(&mut self, game_start_instant: std::time::Instant) {
    self.game_mode = game::GameMode::Running;
    self.gift_points = 0.0;
    self.damage_points = 0.0;
    self.remaining_duration = std::time::Duration::from_millis(450000);
    self.game_start_instant = game_start_instant;
    self.last_update_instant = std::time::Instant::now();
  }

  pub fn start_menu(&mut self) {
    self.game_mode = game::GameMode::Menu;
  }

  pub fn add_gift_points(&mut self, gift_points: f64) {
    self.gift_points = (self.gift_points + gift_points).max(0.0);
  }

  pub fn add_damage_points(&mut self, damage_points: f64) {
    self.damage_points = (self.damage_points + damage_points).max(0.0);
  }

  pub fn do_logic(&mut self) {
    let now = std::time::Instant::now();

    if (self.game_mode == game::GameMode::Running) && (now >= self.game_start_instant) {
      self.remaining_duration -= now - self.last_update_instant;
      let zero_duration = std::time::Duration::from_millis(0);
      if self.remaining_duration < zero_duration { self.remaining_duration = zero_duration; }
    }

    self.last_update_instant = now;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>, font: &'a Font<'a>) {
    if self.game_mode == game::GameMode::Menu {
      font.draw(canvas, Point::zero(), "F1/F2 - Hilfe", Alignment::TopLeft);
      font.draw(canvas, Point::new(self.canvas_size.x / 2.0, 0.0), "F3 - Highscores",
          Alignment::TopCenter);
      font.draw(canvas, Point::new(self.canvas_size.x, 0.0), "F5/F6 - Spielen",
          Alignment::TopRight);
    } else {
      self.gift_image.draw(canvas, Point::new(self.gift_position_x, 0.0), 0.0);
      font.draw_monospace(canvas, Point::new(self.gift_position_x + self.margin_x,
          self.position_y), format!("{}", self.gift_points as i32), Alignment::CenterLeft);

      self.damage_image.draw(canvas, Point::new(self.damage_position_x, 0.0), 0.0);
      font.draw_monospace(canvas, Point::new(self.damage_position_x + self.margin_x,
          self.position_y), format!("{}", -self.damage_points as i32), Alignment::CenterLeft);

      let seconds = self.remaining_duration.as_secs_f64();
      let minutes = (seconds / 60.0).floor() as i32;
      let seconds = (seconds % 60.0) as i32;
      self.time_image.draw(canvas, Point::new(self.time_position_x, 0.0), 0.0);
      font.draw_monospace(canvas, Point::new(self.time_position_x + self.margin_x,
          self.position_y), format!("{}:{:02}", minutes, seconds), Alignment::CenterLeft);
    }
  }
}

impl<'a> HighscoreTable<'a> {
  pub fn new(canvas_size: Point,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) ->
        HighscoreTable<'a> {
    let position = Point::new(50.0, 50.0);
    let size = Point::new(canvas_size.x - 2.0 * position.x, canvas_size.y - 2.0 * position.y);

    let mut background_surface = sdl2::surface::Surface::new(size.x as u32, size.y as u32,
        sdl2::pixels::PixelFormatEnum::RGBA32).expect("Could not create surface");
    background_surface.fill_rect(None, sdl2::pixels::Color::BLACK).expect(
        "Could not fill surface with color");

    let mut background_image = asset::Image::new(
        texture_creator, &background_surface, (1, 1), None);
    background_image.set_alpha(0.5);

    return HighscoreTable{
      background_image: background_image,
      canvas_size: canvas_size,

      game_mode: game::GameMode::Menu,

      size: size,
      position: position,
      inner_margin: Point::new(20.0, 20.0),
      number_of_rows: 10.0,
    };
  }

  pub fn show(&mut self) {
    self.game_mode = game::GameMode::HighscoreTable;
  }

  pub fn hide(&mut self) {
    self.game_mode = game::GameMode::Menu;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>, font: &Font,
        highscores: &Vec<options::Highscore>) {
    if (self.game_mode != game::GameMode::HighscoreTable)
        && (self.game_mode != game::GameMode::NewHighscore) {
      return;
    }

    self.background_image.draw(canvas, self.position, 0.0);

    let inner_height = self.size.y - 2.0 * self.inner_margin.y;
    let offset_y = (inner_height - font.height) / (self.number_of_rows - 1.0);

    for (i, highscore) in highscores.iter().enumerate() {
      let mut dst_point = Point::new(self.position.x + self.inner_margin.x,
          self.position.y + self.inner_margin.y + offset_y * (i as f64));
      font.draw_monospace(canvas, dst_point, highscore.name.to_string(), Alignment::TopLeft);

      dst_point.x = self.position.x + self.size.x - self.inner_margin.y;
      font.draw_monospace(canvas, dst_point, highscore.score.to_string(), Alignment::TopRight);
    }
  }
}

impl<'a> Font<'a> {
  pub fn new(asset_library: &'a asset::AssetLibrary<'a>) -> Font<'a> {
    let image = asset_library.get_image("font");
    let image_width = image.width() as usize;
    let image_height = image.height() as usize;
    let image_surface_width = image_width * (image.total_number_of_frames() as usize);
    let mask = image.mask();
    let characters =
        "-./0123456789:@ABCDEFGHIJKLMNOPQRSTUVWXYZ_\u{00c4}\u{00d6}\u{00dc} ".to_string();
    let mut character_rects: Vec<sdl2::rect::Rect> = Vec::new();

    for (frame, character) in characters.chars().enumerate() {
      if character == ' ' {
        character_rects.push(sdl2::rect::Rect::new(
            0, 0, (image_width / 2) as u32, image_height as u32));
        continue;
      }

      let mut min_point: Point = Point::new(image_width as f64, image_height as f64);
      let mut max_point: Point = Point::new(-1.0, -1.0);

      for y in 0 .. image_height {
        for x in 0 .. image_width {
          let i = (frame as usize) * image_width + x + y * image_surface_width;

          if mask[i] {
            min_point.x = min_point.x.min(x as f64);
            min_point.y = min_point.y.min(y as f64);
            max_point.x = max_point.x.max(x as f64);
            max_point.y = max_point.y.max(y as f64);
          }
        }
      }

      character_rects.push(sdl2::rect::Rect::new(min_point.x as i32, 0,
          (max_point.x - min_point.x + 1.0) as u32, image_height as u32));
    }

    let max_character_width: i32 = character_rects.iter().max_by(|&x, &y| x.width().cmp(&y.width()))
        .expect("No elements in character_widths").width() as i32;

    return Font{
      image: image,
      characters: characters,
      character_rects: character_rects,
      max_character_width: max_character_width,
      height: image.height(),
    };
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget, S: Into<String>>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>,
        dst_point: Point, text: S, alignment: Alignment) {
    self.draw_internal(canvas, dst_point, text, alignment, false);
  }

  pub fn draw_monospace<RenderTarget: sdl2::render::RenderTarget, S: Into<String>>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>,
        dst_point: Point, text: S, alignment: Alignment) {
    self.draw_internal(canvas, dst_point, text, alignment, true);
  }

  fn draw_internal<RenderTarget: sdl2::render::RenderTarget, S: Into<String>>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>,
        dst_point: Point, text: S, alignment: Alignment, monospace: bool) {
    let text: String = text.into();
    let frames: Vec<i32> = text.chars().map(
        |x| self.characters.chars().position(|y| y == x).unwrap_or_else(
        || self.characters.chars().position(|y| y == x.to_uppercase().next().unwrap_or('-'))
          .unwrap_or(0)) as i32).collect();

    let text_character_rects: Vec<sdl2::rect::Rect> =
        frames.iter().map(|x| self.character_rects[*x as usize]).collect();
    let text_width: i32 = if monospace { (text.len() as i32) * self.max_character_width }
        else { text_character_rects.iter().map(|x| x.width() as i32).sum() } as i32;

    let mut dst_point = Point::new(
      dst_point.x - (((alignment as i32) % 3) * (text_width / 2)) as f64,
      dst_point.y - (((alignment as i32) / 3) * ((self.height as i32) / 2)) as f64,
    );

    for ((character, character_rect), frame) in
          text.chars().zip(text_character_rects.iter()).zip(frames.iter()) {
      let monospace_offset_x = ((self.max_character_width as f64)
          - (character_rect.width() as f64)) / 2.0;
      if monospace { dst_point.x += monospace_offset_x; }

      if character != ' ' {
        self.image.draw_blit(canvas, *character_rect, dst_point, *frame as f64);
      }

      dst_point.x += character_rect.width() as f64;
      if monospace { dst_point.x += monospace_offset_x; }
    }
  }
}
