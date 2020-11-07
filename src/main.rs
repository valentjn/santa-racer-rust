/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![allow(dead_code)]

mod assets;
mod game;
mod options;
mod sdl;
mod ui;

use std::process;

use crate::assets::AssetLibrary;
use crate::game::Game;
use crate::options::Options;
use crate::sdl::SdlWrapper;

fn main() {
  let options = Options::load();
  let mut sdl_wrapper = SdlWrapper::new(&options);
  let asset_library = AssetLibrary::new(&sdl_wrapper.texture_creator, &options);
  let mut game = Game::new(&mut sdl_wrapper.canvas, &mut sdl_wrapper.event_pump, &asset_library,
      &options);
  game.run_loop();
  process::exit(0);
}
