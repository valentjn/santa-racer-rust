/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::assets::CloneAsI32Vector;
use crate::assets::Point;

pub struct Score<'a> {
  gift_image: &'a assets::Image<'a>,
  damage_image: &'a assets::Image<'a>,
  time_image: &'a assets::Image<'a>,
  canvas_size: assets::Point,

  game_mode: game::Mode,

  gift_points: f64,
  damage_points: f64,
  remaining_duration: std::time::Duration,
  last_update_instant: std::time::Instant,

  gift_position_x: f64,
  damage_position_x: f64,
  time_position_x: f64,
  margin_x: f64,
  position_y: f64,
}

pub struct Font<'a> {
  image: &'a assets::Image<'a>,
  characters: String,
  character_widths: Vec<i32>,
  max_character_width: i32,
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
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>, canvas_size: Point) -> Score<'a> {
    let gift_image = asset_library.get_image("giftScoreIcon");

    return Score{
      gift_image: gift_image,
      damage_image: asset_library.get_image("damageScoreIcon"),
      time_image: asset_library.get_image("timeScoreIcon"),
      canvas_size: canvas_size,

      game_mode: game::Mode::Menu,

      gift_points: 0.0,
      damage_points: 0.0,
      remaining_duration: std::time::Duration::from_millis(0),
      last_update_instant: std::time::Instant::now(),

      gift_position_x: 0.0,
      damage_position_x: 150.0,
      time_position_x: 535.0,
      margin_x: 35.0,
      position_y: gift_image.height() / 2.0,
    };
  }

  pub fn reset(&mut self) {
    self.gift_points = 0.0;
    self.damage_points = 0.0;
    self.remaining_duration = std::time::Duration::from_millis(450000);
    self.last_update_instant = std::time::Instant::now();
  }

  pub fn add_gift_points(&mut self, gift_points: f64) {
    self.gift_points += gift_points;
  }

  pub fn add_damage_points(&mut self, damage_points: f64) {
    self.damage_points += damage_points;
  }

  pub fn do_logic(&mut self) {
    let now = std::time::Instant::now();
    self.remaining_duration -= now - self.last_update_instant;
    let zero_duration = std::time::Duration::from_millis(0);
    if self.remaining_duration < zero_duration { self.remaining_duration = zero_duration; }
    self.last_update_instant = now;
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>, font: &'a Font<'a>) {
    if self.game_mode == game::Mode::Menu {
      font.draw(canvas, Point::zero(), "F1/F2 - Hilfe", Alignment::TopLeft);
      font.draw(canvas, Point::new(self.canvas_size.x / 2.0, 0.0), "F3 - Highscores",
          Alignment::TopCenter);
      font.draw(canvas, Point::new(self.canvas_size.x, 0.0), "F5 - Spielen", Alignment::TopRight);
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

impl<'a> Font<'a> {
  pub fn new(asset_library: &'a assets::AssetLibrary<'a>) -> Font<'a> {
    let character_widths = asset_library.get_data("fontCharacterWidths").clone_as_i32();
    let max_character_width: i32 = *character_widths.iter().max().expect(
        "No elements in character_widths");

    return Font{
      image: asset_library.get_image("font"),
      characters: "-./0123456789:@ABCDEFGHIJKLMNOPQRSTUVWXYZ_\u{00c4}\u{00d6}\u{00dc} ".to_string(),
      character_widths: character_widths,
      max_character_width: max_character_width,
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

    let text_character_widths: Vec<i32> =
        frames.iter().map(|&x| self.character_widths[x as usize]).collect();
    let text_width: i32 = if monospace { (text.len() as i32) * self.max_character_width }
        else { text_character_widths.iter().sum() } as i32;
    let text_height = self.image.height();

    let mut dst_point = Point::new(
      dst_point.x - (((alignment as i32) % 3) * (text_width / 2)) as f64,
      dst_point.y - (((alignment as i32) / 3) * ((text_height as i32) / 2)) as f64,
    );

    for x in text.chars().zip(text_character_widths.iter()).zip(frames.iter()) {
      let ((character, &character_width), frame) = x;
      let monospace_offset_x = ((self.max_character_width as f64)
          - (character_width as f64)) / 2.0;
      if monospace { dst_point.x += monospace_offset_x; }

      if character != ' ' {
        self.image.draw(canvas, dst_point, *frame as f64);
      }

      dst_point.x += if monospace { monospace_offset_x } else { character_width as f64 };
    }
  }
}
