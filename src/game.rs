/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;
use crate::asset::Point;

pub struct Game<'a: 'b, 'b> {
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

  mode: Mode,
  difficulty: Difficulty,

  font: ui::Font<'a>,
  score: ui::Score<'a>,
  highscore_table: ui::HighscoreTable<'a>,
  landscape: level::Landscape<'a>,
  level: level::Level<'a>,
  sleigh: sleigh::Sleigh<'a>,
  chimneys: Vec<gift::Chimney>,
  gifts: Vec<gift::Gift<'b>>,

  counting_down: bool,
  last_gift_instant: std::time::Instant,

  countdown_duration: std::time::Duration,
  new_gift_wait_duration: std::time::Duration,
}

struct DrawArguments<'a: 'b, 'b> {
  options: &'a options::Options,
  buffer_size: Point,
  asset_library: &'a asset::AssetLibrary<'a>,
  mode: &'a Mode,
  font: &'a ui::Font<'a>,
  highscore_table: &'a ui::HighscoreTable<'a>,
  score: &'a ui::Score<'a>,
  landscape: &'a level::Landscape<'a>,
  level: &'a level::Level<'a>,
  sleigh: &'a sleigh::Sleigh<'a>,
  gifts: &'b Vec<gift::Gift<'b>>,
  fps: f64,
}

#[derive(PartialEq)]
pub enum Mode {
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
pub enum Difficulty {
  Easy,
  Hard,
}

impl<'a: 'b, 'b> Game<'a, 'b> {
  pub fn new(canvas: &'a mut sdl2::render::WindowCanvas,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        event_pump: &'a mut sdl2::EventPump,
        asset_library: &'a asset::AssetLibrary, options: &'a options::Options) -> Game<'a, 'b> {
    let buffer_size = Point::new(640.0, 480.0);
    let buffer_texture = texture_creator.create_texture_target(
        None, buffer_size.x as u32, buffer_size.y as u32).expect("Could not create buffer texture");

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

      mode: Mode::Menu,
      difficulty: Difficulty::Easy,

      font: ui::Font::new(asset_library),
      score: ui::Score::new(asset_library, buffer_size),
      highscore_table: ui::HighscoreTable::new(buffer_size, texture_creator),
      landscape: level::Landscape::new(asset_library),
      level: level::Level::new(asset_library, buffer_size),
      sleigh: sleigh::Sleigh::new(asset_library, buffer_size),
      chimneys: Game::load_chimneys(asset_library),
      gifts: Vec::new(),

      counting_down: false,
      last_gift_instant: now,

      countdown_duration: std::time::Duration::from_millis(3000),
      new_gift_wait_duration: std::time::Duration::from_millis(250),
    };
  }

  fn load_chimneys(asset_library: &'a asset::AssetLibrary) ->
        Vec<gift::Chimney> {
    let data = asset_library.get_data("chimneys");
    let mut chimneys = Vec::new();
    assert!(data.len() % 4 == 0, "Length of chimney hit box data not divisible by 4");

    for i in 0 .. data.len() / 4 {
      chimneys.push(gift::Chimney{
        position: Point::new(data[4 * i], data[4 * i + 1]),
        size: Point::new(data[4 * i + 2], 5.0),
        frame: data[4 * i + 3],
      });
    }

    return chimneys;
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
                && ((self.mode == Mode::Menu) || (self.mode == Mode::HelpPage1)
                  || (self.mode == Mode::HelpPage2) || (self.mode == Mode::HighscoreTable)) {
            self.mode = Mode::HelpPage1;
            self.highscore_table.hide();

          } else if (keycode == sdl2::keyboard::Keycode::F2)
                && ((self.mode == Mode::Menu) || (self.mode == Mode::HelpPage1)
                  || (self.mode == Mode::HelpPage2) || (self.mode == Mode::HighscoreTable)) {
            self.mode = Mode::HelpPage2;
            self.highscore_table.hide();

          } else if (keycode == sdl2::keyboard::Keycode::F3) && (self.mode == Mode::Menu) {
            self.mode = Mode::HighscoreTable;
            self.highscore_table.show();

          } else if (keycode == sdl2::keyboard::Keycode::F3)
                && (self.mode == Mode::HighscoreTable) {
            self.mode = Mode::Menu;
            self.highscore_table.hide();

          } else if ((keycode == sdl2::keyboard::Keycode::F5)
                  || (keycode == sdl2::keyboard::Keycode::F6))
                && ((self.mode == Mode::Menu) || (self.mode == Mode::HighscoreTable)) {
            self.mode = Mode::Running;
            self.difficulty = if keycode == sdl2::keyboard::Keycode::F5 { Difficulty::Easy }
                else { Difficulty::Hard };
            let game_start_instant = now + self.countdown_duration;
            self.counting_down = true;
            self.score.start_game(game_start_instant);
            self.highscore_table.hide();
            self.landscape.start_game(game_start_instant);
            self.level.start_game(game_start_instant);
            self.sleigh.start_game(game_start_instant);

          } else if keycode == sdl2::keyboard::Keycode::Escape {
            match self.mode {
              Mode::Menu | Mode::HighscoreTable => {
                self.quit_flag = true;
              },
              Mode::HelpPage1 | Mode::HelpPage2 => {
                self.mode = Mode::Menu;
              },
              Mode::Running => {
                self.mode = Mode::Menu;
                self.score.start_menu();
                self.landscape.start_menu();
                self.level.start_menu();
                self.sleigh.start_menu();
              },
              _ => {},
            }

          } else if ((keycode == sdl2::keyboard::Keycode::Escape)
                  || (keycode == sdl2::keyboard::Keycode::Space))
                && ((self.mode == Mode::HelpPage1) || (self.mode == Mode::HelpPage2)) {
            self.mode = Mode::Menu;

          } else if (keycode == sdl2::keyboard::Keycode::Space)
                && (self.mode == Mode::Running)
                && (now - self.last_gift_instant >= self.new_gift_wait_duration) {
            self.gifts.push(gift::Gift::new(
                self.asset_library, &self.level, &self.sleigh, self.buffer_size, self.difficulty));
            self.last_gift_instant = now;
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
    self.score.do_logic();
    self.landscape.do_logic(&self.level);
    self.level.do_logic(&self.sleigh);
    self.sleigh.do_logic(&mut self.score, &mut self.landscape, &mut self.level);

    let mut i = 0;

    while i < self.gifts.len() {
      self.gifts[i].do_logic(&mut self.score, &self.level, &self.chimneys);

      if self.gifts[i].mode == gift::GiftMode::CanBeDeleted {
        self.gifts.remove(i);
      } else {
        i += 1;
      }
    }
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
      gifts: &self.gifts,
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

    draw_arguments.asset_library.get_image(background_image_name).draw(canvas, Point::zero(), 0.0);

    match draw_arguments.mode {
      Mode::NewHighscore => {
      },
      Mode::Menu | Mode::HighscoreTable | Mode::Running => {
        draw_arguments.landscape.draw(canvas);
        draw_arguments.level.draw(canvas);
        draw_arguments.sleigh.draw(canvas, draw_arguments.font);

        for gift in draw_arguments.gifts.iter() {
          gift.draw(canvas, draw_arguments.level);
        }

        draw_arguments.score.draw(canvas, draw_arguments.font);
        draw_arguments.highscore_table.draw(canvas, draw_arguments.font,
            &draw_arguments.options.highscores);
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
