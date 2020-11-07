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
  pub fullscreen_enabled: bool,
  pub sound_enabled: bool,
  pub verbose_enabled: bool,
  pub highscores: Vec<Highscore>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Highscore {
  pub name: String,
  pub score: i32,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
  highscores: Vec<Highscore>,
}

impl std::default::Default for ConfigFile {
  fn default() -> ConfigFile {
    return ConfigFile {
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
}
