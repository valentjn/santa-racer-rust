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

  number_of_highscores: usize,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
  highscores: Vec<Highscore>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Highscore {
  name: String,
  points: i32,
}

impl Options {
  pub fn load() -> Options {
    let config_file: ConfigFile = confy::load("santa-racer").expect("Failed to read config");

    let mut options = Options {
      fullscreen_enabled: false,
      sound_enabled: true,
      verbose_enabled: false,
      highscores: config_file.highscores,

      number_of_highscores: 10,
    };

    for _ in options.highscores.len() .. options.number_of_highscores {
      options.highscores.push(Highscore::new("Leer", 0));
    }

    for argument in std::env::args().skip(1) {
      if (argument == "-h") || (argument == "--help") {
        Options::print_description();
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
        Options::print_description();
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

  pub fn save(&self) {
    confy::store("santa-racer", ConfigFile::new(self.highscores.to_vec())).expect(
        "Could not save options");
  }

  fn print_description() {
    println!("Santa Racer - an open-source clone of \"Nikolaus Express 2000\".");
    println!("Source code: Copyright (C) 2008-2020 Julian Valentin, licensed under MPL 2.0.");
    println!("Exception: External assets such as music, sounds, and textures may be subject to \
        be intellectual property of third parties.");
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

  pub fn verbose_enabled(&self) -> bool {
    return self.verbose_enabled;
  }

  pub fn number_of_highscores(&self) -> usize {
    return self.number_of_highscores;
  }

  pub fn highscores(&self) -> &Vec<Highscore> {
    return &self.highscores;
  }

  pub fn highscores_mut(&mut self) -> &mut Vec<Highscore> {
    return &mut self.highscores;
  }
}

impl ConfigFile {
  fn new(highscores: Vec<Highscore>) -> ConfigFile {
    return ConfigFile {
      highscores: highscores,
    };
  }
}

impl std::default::Default for ConfigFile {
  fn default() -> ConfigFile {
    return ConfigFile{
      highscores: Vec::new(),
    };
  }
}

impl Highscore {
  pub fn new<S: Into<String>>(name: S, points: i32) -> Highscore {
    return Highscore{
      name: name.into(),
      points: points,
    };
  }

  pub fn name(&self) -> String {
    return self.name.to_string();
  }

  pub fn set_name<S: Into<String>>(&mut self, name: S) {
    self.name = name.into();
  }

  pub fn points(&self) -> i32 {
    return self.points;
  }

  pub fn set_points(&mut self, points: i32) {
    self.points = points;
  }
}
