/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![allow(dead_code)]

mod asset;
mod game;
mod gift;
mod level;
mod npc;
mod options;
mod sdl;
mod sleigh;
mod ui;

fn main() {
  let options = options::Options::load();

  let mut sdl_wrapper = sdl::SdlWrapper::new(&options);

  let asset_library = asset::AssetLibrary::new(&sdl_wrapper.texture_creator, &options);

  let mut game = game::Game::new(&mut sdl_wrapper.canvas, &sdl_wrapper.texture_creator,
      &mut sdl_wrapper.event_pump, &asset_library, &options);

  game.run_loop();

  std::process::exit(0);
}
