/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::asset::Point;

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

  asset_library: &'a asset::AssetLibrary<'a>,

  mode: GameMode,
  difficulty: GameDifficulty,

  font: ui::Font<'a>,
  score: ui::Score<'a>,
  highscore_table: ui::HighscoreTable<'a>,
  landscape: level::Landscape<'a>,
  level: level::Level<'a>,
  sleigh: sleigh::Sleigh<'a>,

  counting_down: bool,

  countdown_duration: std::time::Duration,
}

struct DrawArguments<'a> {
  options: &'a options::Options,
  buffer_size: Point,
  asset_library: &'a asset::AssetLibrary<'a>,
  mode: &'a GameMode,
  font: &'a ui::Font<'a>,
  highscore_table: &'a ui::HighscoreTable<'a>,
  score: &'a ui::Score<'a>,
  landscape: &'a level::Landscape<'a>,
  level: &'a level::Level<'a>,
  sleigh: &'a sleigh::Sleigh<'a>,
  fps: f64,
}

#[derive(PartialEq)]
pub enum GameMode {
  Menu,
  HelpPage1,
  HelpPage2,
  HighscoreTable,
  Running,
  Won,
  LostDueToTime,
  LostDueToDamage,
  NewHighscore,
}

#[derive(Clone, Copy)]
pub enum GameDifficulty {
  Easy,
  Hard,
}

impl<'a> Game<'a> {
  pub fn new(canvas: &'a mut sdl2::render::WindowCanvas,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        event_pump: &'a mut sdl2::EventPump,
        asset_library: &'a asset::AssetLibrary, options: &'a options::Options) -> Game<'a> {
    let buffer_size = Point::new(640.0, 480.0);
    let buffer_texture = texture_creator.create_texture_target(
        None, buffer_size.x() as u32, buffer_size.y() as u32).expect(
        "Could not create buffer texture");

    asset_library.get_song("music").play();

    let now = std::time::Instant::now();

    return Game{
      options: options,

      canvas: canvas,
      buffer_texture: buffer_texture,
      buffer_size: buffer_size,
      event_pump: event_pump,

      target_fps: 60.0,
      quit_flag: false,
      fps: 0.0,
      frame_counter: 0,
      last_frame_instant: now,
      last_fps_update_instant: now,

      asset_library: asset_library,

      mode: GameMode::Menu,
      difficulty: GameDifficulty::Easy,

      font: ui::Font::new(asset_library),
      score: ui::Score::new(asset_library, buffer_size),
      highscore_table: ui::HighscoreTable::new(buffer_size, texture_creator),
      landscape: level::Landscape::new(asset_library),
      level: level::Level::new(asset_library, buffer_size),
      sleigh: sleigh::Sleigh::new(asset_library, buffer_size),

      counting_down: false,

      countdown_duration: std::time::Duration::from_millis(3000),
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
      let now = std::time::Instant::now();

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

          } else if (keycode == sdl2::keyboard::Keycode::F1)
                && ((self.mode == GameMode::Menu) || (self.mode == GameMode::HelpPage1)
                  || (self.mode == GameMode::HelpPage2) || (self.mode == GameMode::HighscoreTable)) {
            self.mode = GameMode::HelpPage1;
            self.highscore_table.hide();

          } else if (keycode == sdl2::keyboard::Keycode::F2)
                && ((self.mode == GameMode::Menu) || (self.mode == GameMode::HelpPage1)
                  || (self.mode == GameMode::HelpPage2) || (self.mode == GameMode::HighscoreTable)) {
            self.mode = GameMode::HelpPage2;
            self.highscore_table.hide();

          } else if (keycode == sdl2::keyboard::Keycode::F3) && (self.mode == GameMode::Menu) {
            self.mode = GameMode::HighscoreTable;
            self.highscore_table.show();

          } else if (keycode == sdl2::keyboard::Keycode::F3)
                && (self.mode == GameMode::HighscoreTable) {
            self.mode = GameMode::Menu;
            self.highscore_table.hide();

          } else if ((keycode == sdl2::keyboard::Keycode::F5)
                  || (keycode == sdl2::keyboard::Keycode::F6))
                && ((self.mode == GameMode::Menu) || (self.mode == GameMode::HighscoreTable)) {
            self.mode = GameMode::Running;
            self.difficulty = if keycode == sdl2::keyboard::Keycode::F5 { GameDifficulty::Easy }
                else { GameDifficulty::Hard };
            let game_start_instant = now + self.countdown_duration;
            self.counting_down = true;
            self.score.start_game(game_start_instant);
            self.highscore_table.hide();
            self.landscape.start_game(game_start_instant);
            self.level.start_game(game_start_instant);
            self.sleigh.start_game(game_start_instant);

          } else if keycode == sdl2::keyboard::Keycode::Escape {
            match self.mode {
              GameMode::Menu | GameMode::HighscoreTable => {
                self.quit_flag = true;
              },
              GameMode::HelpPage1 | GameMode::HelpPage2 => {
                self.mode = GameMode::Menu;
              },
              GameMode::Running => {
                self.mode = GameMode::Menu;
                self.score.start_menu();
                self.landscape.start_menu();
                self.level.start_menu();
                self.sleigh.start_menu();
              },
              _ => {},
            }

          } else if ((keycode == sdl2::keyboard::Keycode::Escape)
                  || (keycode == sdl2::keyboard::Keycode::Space))
                && ((self.mode == GameMode::HelpPage1) || (self.mode == GameMode::HelpPage2)) {
            self.mode = GameMode::Menu;

          } else if (keycode == sdl2::keyboard::Keycode::Space)
                && (self.mode == GameMode::Running) {
            self.sleigh.drop_gift(self.asset_library, &self.level, self.difficulty);
          }
        },
        _ => {},
      }
    }
  }

  fn check_keyboard_state(&mut self) {
    let keyboard_state = self.event_pump.keyboard_state();

    match self.mode {
      GameMode::Running => {
        self.sleigh.check_keyboard_state(&keyboard_state);
      },
      _ => {},
    }
  }

  fn do_logic(&mut self) {
    self.score.do_logic();
    self.landscape.do_logic(&self.level);
    self.level.do_logic(self.asset_library, &mut self.score, &mut self.landscape, &mut self.sleigh);
    self.sleigh.do_logic(&mut self.score, &mut self.level);
  }

  fn draw(&mut self) {
    let draw_arguments = DrawArguments{
      options: &self.options,
      buffer_size: self.buffer_size,
      asset_library: &self.asset_library,
      mode: &self.mode,
      font: &self.font,
      score: &self.score,
      highscore_table: &self.highscore_table,
      landscape: &self.landscape,
      level: &self.level,
      sleigh: &self.sleigh,
      fps: self.fps,
    };

    self.canvas.with_texture_canvas(&mut self.buffer_texture,
        |canvas| Game::draw_to_canvas(canvas, draw_arguments)).expect("Could not draw to buffer");

    let canvas_size = Point::from_u32_tuple(self.canvas.output_size().expect(
        "Could not get output size of window canvas"));

    let dst_rect = if canvas_size.x() / canvas_size.y()
          >= self.buffer_size.x() / self.buffer_size.y() {
      let dst_width = (self.buffer_size.x() / self.buffer_size.y()) * canvas_size.y();
      sdl2::rect::Rect::new(((canvas_size.x() - dst_width) / 2.0) as i32, 0,
          dst_width as u32, canvas_size.y() as u32)
    } else {
      let dst_height = (self.buffer_size.y() / self.buffer_size.x()) * canvas_size.x();
      sdl2::rect::Rect::new(0, ((canvas_size.y() - dst_height) / 2.0) as i32,
          canvas_size.x() as u32, dst_height as u32)
    };

    self.canvas.copy(&self.buffer_texture, None, dst_rect).expect(
        "Could not copy buffer to window");
    self.canvas.present();
  }

  fn draw_to_canvas(canvas: &mut sdl2::render::WindowCanvas, draw_arguments: DrawArguments) {
    let background_image_name = match draw_arguments.mode {
      GameMode::HelpPage1 => "help1",
      GameMode::HelpPage2 => "help2",
      GameMode::Won | GameMode::NewHighscore => "won",
      GameMode::LostDueToTime => "lostDueToTime",
      GameMode::LostDueToDamage => "lostDueToDamage",
      _ => "background",
    };

    draw_arguments.asset_library.get_image(background_image_name).draw(canvas, Point::zero(), 0.0);

    match draw_arguments.mode {
      GameMode::NewHighscore => {
      },
      GameMode::Menu | GameMode::HighscoreTable | GameMode::Running => {
        draw_arguments.landscape.draw(canvas);
        draw_arguments.level.draw(canvas);
        draw_arguments.sleigh.draw(canvas, draw_arguments.font, draw_arguments.level);
        draw_arguments.score.draw(canvas, draw_arguments.font);
        draw_arguments.highscore_table.draw(canvas, draw_arguments.font,
            &draw_arguments.options.highscores());
      },
      _ => {},
    }

    draw_arguments.font.draw(canvas, draw_arguments.buffer_size,
        format!("{:.0} FPS", draw_arguments.fps), ui::Alignment::BottomRight);
  }

  fn finish_frame(&mut self) {
    let now = std::time::Instant::now();
    let duration_since_last_fps_update = now - self.last_fps_update_instant;

    if duration_since_last_fps_update >= std::time::Duration::from_millis(1000) {
      self.fps = (self.frame_counter as f64) / duration_since_last_fps_update.as_secs_f64();
      if self.options.verbose_enabled() { println!("FPS: {:.1}", self.fps); }
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
