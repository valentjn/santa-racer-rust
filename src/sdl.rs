/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::*;

pub struct SdlWrapper {
  sdl: sdl2::Sdl,

  pub canvas: sdl2::render::WindowCanvas,
  pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,

  mixer: Option<sdl2::mixer::Sdl2MixerContext>,
  pub event_pump: sdl2::EventPump,

  pub text_input_util: sdl2::keyboard::TextInputUtil,
}

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const AUDIO_FREQUENCY: i32 = 44100;
const AUDIO_NUMBER_OF_CHANNELS: i32 = 2;
const AUDIO_CHUNK_SIZE: i32 = 256;

impl SdlWrapper {
  pub fn new(options: &options::Options) -> SdlWrapper {
    let sdl = sdl2::init().expect("Could not initialize SDL");

    let video_subsystem = sdl.video().expect("Could not initialize video subsystem");
    let mut window_builder = video_subsystem.window("Santa Racer", WINDOW_WIDTH, WINDOW_HEIGHT);
    window_builder.resizable().position_centered();
    if options.fullscreen_enabled() { window_builder.fullscreen(); }
    let window = window_builder.build().expect("Could not create window");

    // TODO: add options (e.g., vsync)
    let canvas = window.into_canvas().build().expect("Could not create canvas");
    let texture_creator = canvas.texture_creator();

    let mut mixer: Option<sdl2::mixer::Sdl2MixerContext> = None;

    if options.sound_enabled() {
      mixer.replace(sdl2::mixer::init(sdl2::mixer::InitFlag::OGG).expect(
          "Could not initialize mixer"));
      sdl2::mixer::open_audio(AUDIO_FREQUENCY, sdl2::mixer::DEFAULT_FORMAT,
          AUDIO_NUMBER_OF_CHANNELS, AUDIO_CHUNK_SIZE).expect("Could not open audio");
    }

    let event_pump = sdl.event_pump().expect("Could not create event pump");
    let text_input_util = video_subsystem.text_input();

    return SdlWrapper{
      sdl: sdl,

      canvas: canvas,
      texture_creator: texture_creator,

      mixer: mixer,
      event_pump: event_pump,

      text_input_util: text_input_util,
    };
  }
}
