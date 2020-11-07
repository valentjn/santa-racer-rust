/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::options::Options;

pub struct SdlWrapper {
  sdl: sdl2::Sdl,
  pub canvas: sdl2::render::WindowCanvas,
  mixer: Option<sdl2::mixer::Sdl2MixerContext>,
  pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
  pub event_pump: sdl2::EventPump,
  screen_width: u32,
  screen_height: u32,
}

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const AUDIO_FREQUENCY: i32 = 44100;
const AUDIO_NUMBER_OF_CHANNELS: i32 = 2;
const AUDIO_CHUNK_SIZE: i32 = 256;

impl SdlWrapper {
  pub fn new(options: &Options) -> SdlWrapper {
    let sdl = sdl2::init().expect("Could not initialize SDL");

    let video_subsystem = sdl.video().expect("Could not initialize video subsystem");
    let mut window_builder = video_subsystem.window("Santa Racer", SCREEN_WIDTH, SCREEN_HEIGHT);
    window_builder.position_centered();
    if options.fullscreen_enabled { window_builder.fullscreen_desktop(); }
    let window = window_builder.build().expect("Could not create window");

    // TODO: add options (e.g., vsync)
    let canvas = window.into_canvas().build().expect("Could not create canvas");

    let mut mixer: Option<sdl2::mixer::Sdl2MixerContext> = None;

    if options.sound_enabled {
      mixer.replace(sdl2::mixer::init(sdl2::mixer::InitFlag::OGG).expect(
          "Could not initialize mixer"));
      sdl2::mixer::open_audio(AUDIO_FREQUENCY, sdl2::mixer::DEFAULT_FORMAT,
          AUDIO_NUMBER_OF_CHANNELS, AUDIO_CHUNK_SIZE).expect("Could not open audio");
    }

    let texture_creator = canvas.texture_creator();
    let event_pump = sdl.event_pump().expect("Could not create event pump");

    return SdlWrapper {
      sdl: sdl,
      canvas: canvas,
      mixer: mixer,
      texture_creator: texture_creator,
      event_pump: event_pump,
      screen_width: SCREEN_WIDTH,
      screen_height: SCREEN_HEIGHT,
    };
  }
}
