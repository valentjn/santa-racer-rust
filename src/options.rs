/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use serde::Serialize;
use serde::Deserialize;

#[derive(Clone)]
pub struct Options {
  fullscreen_enabled: bool,
  sound_enabled: bool,
  verbose_enabled: bool,
  highscores: Vec<Highscore>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Highscore {
  name: String,
  score: i32,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
  highscores: Vec<Highscore>,
}

impl std::default::Default for ConfigFile {
  fn default() -> ConfigFile {
    return ConfigFile{
      highscores: Vec::new(),
    };
  }
}

fn print_description() {
  println!("Santa Racer - an open-source clone of \"Nikolaus Express 2000\".");
  println!("Source code: Copyright (C) 2008-2020 Julian Valentin, licensed under MPL 2.0.");
  println!("Exception: External assets such as music, sounds, and textures may be subject to \
      be intellectual property of third parties.");
}

impl Options {
  pub fn load() -> Options {
    let config_file: ConfigFile = confy::load("santa-racer").expect("Failed to read config");

    let mut options = Options {
      fullscreen_enabled: false,
      sound_enabled: true,
      verbose_enabled: false,
      highscores: config_file.highscores,
    };

    for _ in options.highscores.len() .. 10 {
      options.highscores.push(Highscore::new("Leer", 0));
    }

    for argument in std::env::args().skip(1) {
      if (argument == "-h") || (argument == "--help") {
        print_description();
        println!("");
        println!("-f, --fullscreen     enable fullscreen mode");
        println!("    --no-fullscreen  disable fullscreen mode");
        println!("-s, --sound          enable sound");
        println!("    --no-sound       disable sound");
        println!("-v, --verbose        increase verbosity");
        println!("-h, --help           display help message");
        println!("-V, --version        display version");
        println!("-L, --license        display license info");
        std::process::exit(0);
      } else if (argument == "-V") || (argument == "--version") {
        println!("Santa Racer {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
      } else if (argument == "-L") || (argument == "--license") {
        print_description();
        std::process::exit(0);
      } else if (argument == "-f") || (argument == "--fullscreen") {
        options.fullscreen_enabled = true;
      } else if argument == "--no-fullscreen" {
        options.fullscreen_enabled = false;
      } else if (argument == "-s") || (argument == "--sound") {
        options.sound_enabled = true;
      } else if argument == "--no-sound" {
        options.sound_enabled = false;
      } else if (argument == "-v") || (argument == "--verbose") {
        options.verbose_enabled = true;
      }
    }

    return options;
  }

  pub fn fullscreen_enabled(&self) -> bool {
    return self.fullscreen_enabled;
  }

  pub fn set_fullscreen_enabled(&mut self, fullscreen_enabled: bool) {
    self.fullscreen_enabled = fullscreen_enabled;
  }

  pub fn sound_enabled(&self) -> bool {
    return self.sound_enabled;
  }

  pub fn set_sound_enabled(&mut self, sound_enabled: bool) {
    self.sound_enabled = sound_enabled;
  }

  pub fn verbose_enabled(&self) -> bool {
    return self.verbose_enabled;
  }

  pub fn set_verbose_enabled(&mut self, verbose_enabled: bool) {
    self.verbose_enabled = verbose_enabled;
  }

  pub fn highscores(&self) -> &Vec<Highscore> {
    return &self.highscores;
  }

  pub fn set_highscores(&mut self, highscores: Vec<Highscore>) {
    self.highscores = highscores;
  }
}

impl Highscore {
  pub fn new<S: Into<String>>(name: S, score: i32) -> Highscore {
    return Highscore{
      name: name.into(),
      score: score,
    };
  }

  pub fn name(&self) -> String {
    return self.name.to_string();
  }

  pub fn set_name<S: Into<String>>(&mut self, name: S) {
    self.name = name.into();
  }

  pub fn score(&self) -> i32 {
    return self.score;
  }

  pub fn set_score(&mut self, score: i32) {
    self.score = score;
  }
}
