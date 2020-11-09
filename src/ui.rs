/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::assets::CloneAsI32Vector;
use crate::assets::Point;

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
        dst_point: &Point, text: S, alignment: Alignment) {
    self.draw_internal(canvas, dst_point, text, alignment, false);
  }

  pub fn draw_monospace<RenderTarget: sdl2::render::RenderTarget, S: Into<String>>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>,
        dst_point: &Point, text: S, alignment: Alignment) {
    self.draw_internal(canvas, dst_point, text, alignment, true);
  }

  fn draw_internal<RenderTarget: sdl2::render::RenderTarget, S: Into<String>>(
        &self, canvas: &mut sdl2::render::Canvas<RenderTarget>,
        dst_point: &Point, text: S, alignment: Alignment, monospace: bool) {
    let text: String = text.into();
    let frames: Vec<i32> = text.chars().map(
        |x| self.characters.chars().position(|y| y == x).unwrap_or_else(
        || self.characters.chars().position(|y| y == x.to_uppercase().next().unwrap_or('-'))
          .unwrap_or(0)) as i32).collect();

    let text_character_widths: Vec<i32> =
        frames.iter().map(|&x| self.character_widths[x as usize]).collect();
    let text_width: i32 = match monospace {
      true => (text.len() as i32) * self.max_character_width,
      false => text_character_widths.iter().sum(),
    } as i32;
    let text_height = self.image.height();

    let mut dst_point = Point::new(
      dst_point.x - (((alignment as i32) % 3) * (text_width / 2)) as f64,
      dst_point.y - (((alignment as i32) / 3) * ((text_height as i32) / 2)) as f64,
    );

    for x in text.chars().zip(text_character_widths.iter()).zip(frames.iter()) {
      let ((character, character_width), frame) = x;

      if monospace {
        dst_point.x += ((self.max_character_width as f64) -
            (*character_width as f64)) / 2.0;
      }

      if character != ' ' {
        self.image.draw(canvas, &dst_point, *frame as f64);
      }

      dst_point.x += match monospace {
        true => self.max_character_width,
        false => *character_width,
      } as f64;
    }
  }
}
