/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::assets::AssetLibrary;
use crate::assets::CloneAsI32Vector;
use crate::assets::Point;
use crate::options::Options;
use crate::ui::Alignment;
use crate::ui::Font;

pub struct Game<'a> {
  options: &'a Options,

  canvas: &'a mut sdl2::render::WindowCanvas,
  canvas_size: Point,
  event_pump: &'a mut sdl2::EventPump,
  asset_library: &'a AssetLibrary<'a>,

  font: Font<'a>,

  mode: Mode,

  target_fps: f64,
  quit_flag: bool,
  fps: f64,
  frame_counter: u32,
  last_frame_instant: Option<std::time::Instant>,
  last_fps_update_instant: Option<std::time::Instant>,
}

enum Mode {
  Menu,
  HelpPage1,
  HelpPage2,
  Highscores,
  Running,
  Won,
  LostDueToTime,
  LostDueToDamage,
  NewHighscore,
}

const TARGET_FPS: f64 = 30.0;

impl<'a> Game<'a> {
  pub fn new(canvas: &'a mut sdl2::render::WindowCanvas, event_pump: &'a mut sdl2::EventPump,
        asset_library: &'a AssetLibrary, options: &'a Options) -> Game<'a> {
    let canvas_size = Point::from_u32_tuple(
        canvas.output_size().expect("Could not get output size of canvas"));

    asset_library.get_song("music").play();

    let font = Font::new(asset_library.get_image("font"),
        "-./0123456789:@ABCDEFGHIJKLMNOPQRSTUVWXYZ_\u{00c4}\u{00d6}\u{00dc} ",
        asset_library.get_data("fontCharacterWidths").clone_as_i32());

    return Game {
      options: options,

      canvas: canvas,
      canvas_size: canvas_size,
      event_pump: event_pump,
      asset_library: asset_library,

      font: font,

      mode: Mode::Menu,

      target_fps: TARGET_FPS,
      quit_flag: false,
      fps: 0.0,
      frame_counter: 0,
      last_frame_instant: Option::None,
      last_fps_update_instant: Option::None,
    };
  }

  pub fn run_loop(&mut self) {
    while !self.quit_flag {
      self.process_events();
      self.check_keys();
      self.do_logic();
      self.draw();
      self.finish_frame();
    }
  }

  fn process_events(&mut self) {
    for event in self.event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. } => self.quit_flag = true,
        _ => {},
      }
    }
  }

  fn check_keys(&self) {
  }

  fn do_logic(&self) {

  }

  fn draw(&mut self) {
    let background_image_name = match self.mode {
      Mode::HelpPage1 => "help1",
      Mode::HelpPage2 => "help2",
      Mode::Won | Mode::NewHighscore => "won",
      Mode::LostDueToTime => "lostDueToTime",
      Mode::LostDueToDamage => "lostDueToDamage",
      _ => "background",
    };

    self.asset_library.get_image(background_image_name).draw(self.canvas, &Point::zero(), 0);

    match self.mode {
      Mode::NewHighscore => {
      },
      Mode::Menu | Mode::Highscores | Mode::Running => {
      },
      _ => {},
    }

    self.font.draw(self.canvas, &Point::zero(), "Hello World", Alignment::TopLeft);
    self.font.draw(self.canvas, &self.canvas_size, format!("{:.0} FPS", self.fps),
        Alignment::BottomRight);

    self.canvas.present();
  }

  fn finish_frame(&mut self) {
    let now = std::time::Instant::now();

    match self.last_fps_update_instant {
      Some(last_fps_update_instant)
            if now >= last_fps_update_instant + std::time::Duration::from_secs(1) => {
        self.fps = (self.frame_counter as f64) /
            now.duration_since(last_fps_update_instant).as_secs_f64();
        if self.options.verbose_enabled { println!("FPS: {:.1}", self.fps); }
        self.last_fps_update_instant.replace(now);
        self.frame_counter = 0;
      },
      None => {
        self.last_fps_update_instant.replace(now);
      },
      _ => {},
    };

    if let Some(last_frame_instant) = self.last_frame_instant {
      let frame_duration = now - last_frame_instant;
      let target_frame_duration = std::time::Duration::from_secs_f64(1.0 / self.target_fps);

      if frame_duration < target_frame_duration {
        std::thread::sleep(target_frame_duration - frame_duration);
      }
    }

    self.last_frame_instant.replace(std::time::Instant::now());
    self.frame_counter += 1;
  }
}
