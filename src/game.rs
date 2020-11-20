/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::assets::Point;

pub struct Game<'a> {
  options: &'a options::Options,

  canvas: &'a mut sdl2::render::WindowCanvas,
  buffer_texture: sdl2::render::Texture<'a>,
  buffer_size: Point,
  event_pump: &'a mut sdl2::EventPump,

  target_fps: f64,
  quit_flag: bool,
  fps: f64,
  frame_counter: u32,
  last_frame_instant: std::time::Instant,
  last_fps_update_instant: std::time::Instant,

  asset_library: &'a assets::AssetLibrary<'a>,

  mode: Mode,

  font: ui::Font<'a>,
  landscape: level::Landscape<'a>,
  level: level::Level<'a>,
  sleigh: fg_objects::Sleigh<'a>,
}

struct DrawArguments<'a> {
  buffer_size: Point,
  asset_library: &'a assets::AssetLibrary<'a>,
  font: &'a ui::Font<'a>,
  mode: &'a Mode,
  landscape: &'a level::Landscape<'a>,
  level: &'a level::Level<'a>,
  sleigh: &'a fg_objects::Sleigh<'a>,
  fps: f64,
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

const BUFFER_WIDTH: f64 = 640.0;
const BUFFER_HEIGHT: f64 = 480.0;
const TARGET_FPS: f64 = 30.0;

impl<'a> Game<'a> {
  pub fn new(canvas: &'a mut sdl2::render::WindowCanvas,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        event_pump: &'a mut sdl2::EventPump,
        asset_library: &'a assets::AssetLibrary, options: &'a options::Options) -> Game<'a> {
    let buffer_size = Point::from_u32_tuple(
        canvas.output_size().expect("Could not get output size of canvas"));
    let buffer_texture = texture_creator.create_texture_target(
        None, BUFFER_WIDTH as u32, BUFFER_HEIGHT as u32).expect("Could not create buffer texture");

    asset_library.get_song("music").play();

    let font = ui::Font::new(asset_library);
    let landscape = level::Landscape::new(asset_library);
    let level = level::Level::new(asset_library, buffer_size);
    let sleigh = fg_objects::Sleigh::new(asset_library, buffer_size, texture_creator);

    return Game{
      options: options,

      canvas: canvas,
      buffer_texture: buffer_texture,
      buffer_size: buffer_size,
      event_pump: event_pump,

      target_fps: TARGET_FPS,
      quit_flag: false,
      fps: 0.0,
      frame_counter: 0,
      last_frame_instant: std::time::Instant::now(),
      last_fps_update_instant: std::time::Instant::now(),

      asset_library: asset_library,

      // TODO
      mode: Mode::Running,

      font: font,
      landscape: landscape,
      level: level,
      sleigh: sleigh,
    };
  }

  pub fn run_loop(&mut self) {
    while !self.quit_flag {
      self.process_events();
      self.check_keyboard_state();
      self.do_logic();
      self.draw();
      self.finish_frame();
    }
  }

  fn process_events(&mut self) {
    for event in self.event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit{..} => self.quit_flag = true,
        sdl2::event::Event::KeyDown{keycode, keymod, ..} => {
          let keycode = keycode.expect("Could not get keycode");

          if (keymod.contains(sdl2::keyboard::Mod::LCTRLMOD)
                || keymod.contains(sdl2::keyboard::Mod::RCTRLMOD))
                && (keycode == sdl2::keyboard::Keycode::C) {
            self.quit_flag = true;
          } else if (keymod.contains(sdl2::keyboard::Mod::LALTMOD)
                || keymod.contains(sdl2::keyboard::Mod::RALTMOD))
                && (keycode == sdl2::keyboard::Keycode::Return) {
            let fullscreen_state = match self.canvas.window().fullscreen_state() {
              sdl2::video::FullscreenType::True => sdl2::video::FullscreenType::Off,
              _ => sdl2::video::FullscreenType::True,
            };
            self.canvas.window_mut().set_fullscreen(fullscreen_state).expect(
                "Could not change fullscreen state");
          }
        },
        _ => {},
      }
    }
  }

  fn check_keyboard_state(&mut self) {
    let keyboard_state = self.event_pump.keyboard_state();

    match self.mode {
      Mode::Running => {
        self.sleigh.check_keyboard_state(&keyboard_state);
      },
      _ => {},
    }
  }

  fn do_logic(&mut self) {
    self.landscape.do_logic(&self.level);
    self.level.do_logic(&self.sleigh);
    self.sleigh.do_logic();
  }

  fn draw(&mut self) {
    let draw_arguments = DrawArguments{
      buffer_size: self.buffer_size,
      asset_library: &self.asset_library,
      font: &self.font,
      mode: &self.mode,
      landscape: &self.landscape,
      level: &self.level,
      sleigh: &self.sleigh,
      fps: self.fps,
    };

    self.canvas.with_texture_canvas(&mut self.buffer_texture,
        |canvas| Game::draw_to_canvas(canvas, draw_arguments)).expect("Could not draw to buffer");

    let canvas_size = Point::from_u32_tuple(self.canvas.output_size().expect(
        "Could not get output size of window canvas"));

    let dst_rect = if canvas_size.x / canvas_size.y
          >= self.buffer_size.x / self.buffer_size.y {
      let dst_width = (self.buffer_size.x / self.buffer_size.y) * canvas_size.y;
      sdl2::rect::Rect::new(((canvas_size.x - dst_width) / 2.0) as i32, 0,
          dst_width as u32, canvas_size.y as u32)
    } else {
      let dst_height = (self.buffer_size.y / self.buffer_size.x) * canvas_size.x;
      sdl2::rect::Rect::new(0, ((canvas_size.y - dst_height) / 2.0) as i32,
          canvas_size.x as u32, dst_height as u32)
    };

    self.canvas.copy(&self.buffer_texture, None, dst_rect).expect(
        "Could not copy buffer to window");
    self.canvas.present();
  }

  fn draw_to_canvas(canvas: &mut sdl2::render::WindowCanvas, draw_arguments: DrawArguments) {
    let background_image_name = match draw_arguments.mode {
      Mode::HelpPage1 => "help1",
      Mode::HelpPage2 => "help2",
      Mode::Won | Mode::NewHighscore => "won",
      Mode::LostDueToTime => "lostDueToTime",
      Mode::LostDueToDamage => "lostDueToDamage",
      _ => "background",
    };

    draw_arguments.asset_library.get_image(background_image_name).draw(canvas, &Point::zero(), 0.0);

    match draw_arguments.mode {
      Mode::NewHighscore => {
      },
      Mode::Menu | Mode::Highscores | Mode::Running => {
        draw_arguments.landscape.draw(canvas);
        draw_arguments.level.draw_background(canvas);
        draw_arguments.sleigh.draw(canvas);
      },
      _ => {},
    }

    draw_arguments.font.draw(canvas, &Point::zero(), "Hello World", ui::Alignment::TopLeft);
    draw_arguments.font.draw(canvas, &draw_arguments.buffer_size,
        format!("{:.0} FPS", draw_arguments.fps), ui::Alignment::BottomRight);
  }

  fn finish_frame(&mut self) {
    let now = std::time::Instant::now();
    let duration_since_last_fps_update = now.duration_since(self.last_fps_update_instant);

    if duration_since_last_fps_update >= std::time::Duration::from_secs(1) {
      self.fps = (self.frame_counter as f64) / duration_since_last_fps_update.as_secs_f64();
      if self.options.verbose_enabled { println!("FPS: {:.1}", self.fps); }
      self.last_fps_update_instant = now;
      self.frame_counter = 0;
    }

    let frame_duration = now - self.last_frame_instant;
    let target_frame_duration = std::time::Duration::from_secs_f64(1.0 / self.target_fps);

    if frame_duration < target_frame_duration {
      std::thread::sleep(target_frame_duration - frame_duration);
    }

    self.last_frame_instant = std::time::Instant::now();
    self.frame_counter += 1;
  }
}
