/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::io::BufRead;

use crate::options::Options;

pub struct AssetLibrary<'a> {
  data_library: SingleTypeAssetLibrary<Vec<f64>>,
  image_library: SingleTypeAssetLibrary<Image<'a>>,
  song_library: SingleTypeAssetLibrary<Song<'a>>,
  sound_library: SingleTypeAssetLibrary<Sound>,
}

pub trait CloneAsI32Vector {
  fn clone_as_i32(&self) -> Vec<i32>;
}

impl CloneAsI32Vector for Vec<f64> {
  fn clone_as_i32(&self) -> Vec<i32> {
    return self.iter().map(|x| *x as i32).collect();
  }
}

struct SingleTypeAssetLibrary<AssetType> {
  map: std::collections::HashMap<String, AssetType>,
}

pub struct Image<'a> {
  file_path: std::path::PathBuf,
  surface: sdl2::surface::Surface<'a>,
  texture: sdl2::render::Texture<'a>,
  number_of_frames: (i32, i32),
  mask: Vec<bool>,
}

pub struct Song<'a> {
  music: sdl2::mixer::Music<'a>,
}

pub struct Sound {
  chunk: sdl2::mixer::Chunk,
}

#[derive(Clone)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl<'a> AssetLibrary<'a> {
  pub fn new(texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        options: &'a Options) -> AssetLibrary<'a> {
    let mut data_library: SingleTypeAssetLibrary<Vec<f64>> = SingleTypeAssetLibrary::new();
    let mut image_library: SingleTypeAssetLibrary<Image<'a>> = SingleTypeAssetLibrary::new();
    let mut song_library: SingleTypeAssetLibrary<Song<'a>> = SingleTypeAssetLibrary::new();
    let mut sound_library: SingleTypeAssetLibrary<Sound> = SingleTypeAssetLibrary::new();

    data_library.load_assets(options.verbose_enabled);
    image_library.load_assets(texture_creator, options.verbose_enabled);

    if options.sound_enabled {
      song_library.load_assets(options.verbose_enabled);
      sound_library.load_assets(options.verbose_enabled);
    }

    return AssetLibrary {
      data_library: data_library,
      image_library: image_library,
      song_library: song_library,
      sound_library: sound_library,
    };
  }

  pub fn get_data<S: Into<String>>(&'a self, name: S) -> &'a Vec<f64> {
    return self.data_library.get_asset(name);
  }

  pub fn get_image<S: Into<String>>(&'a self, name: S) -> &'a Image<'a> {
    return self.image_library.get_asset(name);
  }

  pub fn get_song<S: Into<String>>(&'a self, name: S) -> &'a Song<'a> {
    return self.song_library.get_asset(name);
  }

  pub fn get_sound<S: Into<String>>(&'a self, name: S) -> &'a Sound {
    return self.sound_library.get_asset(name);
  }
}

impl<'a, AssetType> SingleTypeAssetLibrary<AssetType> {
  pub fn new() -> SingleTypeAssetLibrary<AssetType> {
    return SingleTypeAssetLibrary {
      map: std::collections::HashMap::new(),
    };
  }

  fn load_assets_from_path<S: Into<String>, F>(&mut self, dir_path: &std::path::Path,
        extension: S, load_fn: F, verbose: bool) where F: Fn(&std::path::Path) -> AssetType {
    let extension: String = extension.into();
    let mut entry_paths: Vec<std::path::PathBuf> = dir_path.read_dir()
        .expect("Could not read directory").filter_map(|x| x.ok()).map(|x| x.path()).collect();
    entry_paths.sort();

    for entry_path in entry_paths {
      if entry_path.is_file() && (entry_path.extension().expect("Could not get extension").to_str()
            .expect("Could not convert extension to string") == extension) {
        let asset_name = entry_path.file_stem().expect("Could not get file stem").to_str()
            .expect("Could not convert file stem to string");
        if verbose { println!("Loading asset '{}'...", asset_name); }
        let asset = load_fn(&entry_path);
        self.map.insert(asset_name.to_string(), asset);
      }
    }
  }

  pub fn get_asset<S: Into<String>>(&'a self, name: S) -> &'a AssetType {
    let name: String = name.into();
    return self.map.get(&name).expect(
        format!("Could not find asset with name '{}'", name).as_str());
  }
}

impl SingleTypeAssetLibrary<Vec<f64>> {
  pub fn load_assets(&mut self, verbose: bool) {
    self.load_assets_from_path(std::path::Path::new("./assets/data"), "txt",
        |file_path| SingleTypeAssetLibrary::load_data(file_path), verbose);
  }

  fn load_data(file_path: &std::path::Path) -> Vec<f64> {
    let file_path_str = file_path.to_str().expect("Could not convert path to string");
    let file = std::fs::File::open(file_path).expect(
        format!("Could not open file '{}'", file_path_str).as_str());
    let reader = std::io::BufReader::new(file);
    let mut data: Vec<f64> = Vec::new();

    for line in reader.lines() {
      let line = line.expect(format!("Could not read line from '{}'", file_path_str).as_str());

      for entry in line.split(char::is_whitespace) {
        if entry.is_empty() { continue; }
        data.push(entry.parse().expect(
            format!("Could not parse '{}' as number", entry).as_str()));
      }
    }

    return data;
  }
}

impl<'a> SingleTypeAssetLibrary<Image<'a>> {
  pub fn load_assets<T>(&mut self, texture_creator: &'a sdl2::render::TextureCreator<T>,
        verbose: bool) {
    let mut numbers_of_frames = std::collections::HashMap::new();
    numbers_of_frames.insert("angel", (13, 1));
    numbers_of_frames.insert("ballon1", (8, 1));
    numbers_of_frames.insert("ballon2", (7, 1));
    numbers_of_frames.insert("ballon3", (8, 1));
    numbers_of_frames.insert("ballon4", (8, 1));
    numbers_of_frames.insert("ballon5", (8, 1));
    numbers_of_frames.insert("bigStar", (10, 1));
    numbers_of_frames.insert("drunkStar", (17, 1));
    numbers_of_frames.insert("font", (45, 1));
    numbers_of_frames.insert("gift1", (15, 1));
    numbers_of_frames.insert("gift2", (15, 1));
    numbers_of_frames.insert("gift3", (15, 1));
    numbers_of_frames.insert("goblin", (19, 1));
    numbers_of_frames.insert("level", (1, 81));
    numbers_of_frames.insert("reindeer", (14, 1));
    numbers_of_frames.insert("shield", (8, 1));
    numbers_of_frames.insert("sleigh", (14, 1));
    numbers_of_frames.insert("snowman", (8, 1));
    numbers_of_frames.insert("smallStar", (17, 1));
    numbers_of_frames.insert("star", (17, 1));

    self.load_assets_from_path(std::path::Path::new("./assets/images"), "png",
        |file_path| {
          let asset_name = file_path.file_stem().expect("Could not get file stem").to_str()
              .expect("Could not convert file stem to string");
          let number_of_frames: (i32, i32) = match numbers_of_frames.get(asset_name) {
            Some(number_of_frames) => *number_of_frames,
            None => (1, 1),
          };
          return Image::new(texture_creator, file_path, number_of_frames, None);
        }, verbose);
  }
}

impl<'a> SingleTypeAssetLibrary<Song<'a>> {
  pub fn load_assets(&mut self, verbose: bool) {
    self.load_assets_from_path(std::path::Path::new("./assets/songs"), "ogg",
        |file_path| Song::new(file_path), verbose);
  }
}

impl SingleTypeAssetLibrary<Sound> {
  pub fn load_assets(&mut self, verbose: bool) {
    self.load_assets_from_path(std::path::Path::new("./assets/sounds"), "wav",
        |file_path| Sound::new(file_path), verbose);
  }
}

impl<'a> Image<'a> {
  fn new<RenderTarget>(texture_creator: &'a sdl2::render::TextureCreator<RenderTarget>,
        file_path: &std::path::Path, number_of_frames: (i32, i32),
        mask: Option<Vec<bool>>) -> Image<'a> {
    let file_path_str = file_path.to_str().expect("Could not convert path to string");
    let surface = sdl2::image::LoadSurface::from_file(file_path).expect(
        format!("Could not load surface from '{}'", file_path_str).as_str());
    let texture = texture_creator.create_texture_from_surface(&surface).expect(
        format!("Could not load texture from '{}'", file_path_str).as_str());

    let mask = match mask {
      Some(mask) => mask,
      None => Image::mask_from_surface(&surface),
    };

    return Image {
      file_path: file_path.to_path_buf(),
      surface: surface,
      texture: texture,
      number_of_frames: number_of_frames,
      mask: mask,
    };
  }

  fn mask_from_surface(surface: &sdl2::surface::Surface<'a>) -> Vec<bool> {
    let (width, height) = surface.size();
    let pitch = surface.pitch();
    let pixel_format = &surface.pixel_format();
    let mut mask: Vec<bool> = Vec::new();

    surface.with_lock(|pixels| {
      for y in 0 .. height {
        for x in 0 .. width {
          let offset = (4 * x + y * pitch) as usize;
          let pixel = u32::from_ne_bytes([pixels[offset], pixels[offset + 1],
              pixels[offset + 2], pixels[offset + 3]]);
          let color = sdl2::pixels::Color::from_u32(pixel_format, pixel);
          mask.push(color.a > 0);
        }
      }
    });

    return mask;
  }

  pub fn clone<RenderTarget>(&self,
        texture_creator: &'a sdl2::render::TextureCreator<RenderTarget>) -> Image<'a> {
    return Image::new(texture_creator, &self.file_path, self.number_of_frames,
        Some(self.mask.to_vec()));
  }

  pub fn draw<RenderTarget: sdl2::render::RenderTarget>(&self,
        canvas: &mut sdl2::render::Canvas<RenderTarget>, dst_point: &Point, frame: i32) {
    self.draw_blit(canvas, &sdl2::rect::Rect::new(0, 0, self.width() as u32, self.height() as u32),
        dst_point, frame);
  }

  pub fn draw_blit<RenderTarget: sdl2::render::RenderTarget>(&self,
        canvas: &mut sdl2::render::Canvas<RenderTarget>,
        src_rect: &sdl2::rect::Rect, dst_point: &Point, frame: i32) {
    let src_rect = sdl2::rect::Rect::new(
        src_rect.x() + (frame % self.number_of_frames.0) * self.width(),
        src_rect.y() + ((frame / self.number_of_frames.0) % self.number_of_frames.1)
          * self.height(), src_rect.width(), src_rect.height());

    let dst_rect = sdl2::rect::Rect::new(dst_point.x, dst_point.y,
        src_rect.width(), src_rect.height());

    canvas.copy(&self.texture, src_rect, dst_rect).expect("Could not copy texture");
  }

  pub fn collides(&self, point: &Point, frame: i32, other: &Image, other_point: &Point,
        other_frame: i32) -> bool {
    let (width, height) = (self.width(), self.height());
    let (other_width, other_height) = (other.width(), other.height());

    if ((point.x < other_point.x) && (point.x + width < other_point.x))
          || ((other_point.x < point.x) && (other_point.x + other_width < point.x))
          || ((point.y < other_point.y) && (point.y + height < other_point.y))
          || ((other_point.y < point.y) && (other_point.y + other_height < point.y)) {
      return false;
    }

    let clip_rect_width = if point.x < other_point.x {
          other_width.min(point.x + width - other_point.x) as u32
        } else {
          width.min(other_point.x + other_width - point.x) as u32
        };

    let clip_rect_height = if point.y < other_point.y {
          other_height.min(point.y + height - other_point.y) as u32
        } else {
          height.min(other_point.y + other_height - point.y) as u32
        };

    let clip_rect = sdl2::rect::Rect::new(
        point.x.max(other_point.x), point.y.max(other_point.y), clip_rect_width, clip_rect_height);

    let surface_width = self.surface.width() as i32;
    let other_surface_width = other.surface.width() as i32;
    let number_of_frames = self.number_of_frames;
    let other_number_of_frames = other.number_of_frames;
    let mask = self.mask();
    let other_mask = other.mask();

    for clip_y in clip_rect.top() .. clip_rect.bottom() {
      for clip_x in clip_rect.left() .. clip_rect.right() {
        let index = (
            (clip_x - point.x + (frame % number_of_frames.0) * width)
            + (clip_y - point.y
              + ((frame / number_of_frames.0) % number_of_frames.1)
              * height) * surface_width) as usize;

        let other_index = (
            (clip_x - other_point.x + (other_frame % other_number_of_frames.0) * other_width)
            + (clip_y - other_point.y
              + ((other_frame / other_number_of_frames.0) % other_number_of_frames.1)
              * other_height) * other_surface_width) as usize;

        if mask[index] && other_mask[other_index] { return true; }
      }
    }

    return false;
  }

  pub fn width(&self) -> i32 {
    return (self.surface.width() as i32) / self.number_of_frames.0;
  }

  pub fn height(&self) -> i32 {
    return (self.surface.height() as i32) / self.number_of_frames.1;
  }

  pub fn size(&self) -> (i32, i32) {
    return (self.width(), self.height());
  }

  fn mask(&self) -> &Vec<bool> {
    return &self.mask;
  }

  pub fn set_alpha(&mut self, alpha: f64) {
    self.texture.set_alpha_mod(((alpha as f64) * 255.0).min(0.0).max(255.0) as u8);
  }
}

impl<'a> Song<'a> {
  fn new(file_path: &std::path::Path) -> Song<'a> {
    let file_path_str = file_path.to_str().expect("Could not convert path to string");

    return Song {
      music: sdl2::mixer::Music::from_file(file_path).expect(
          format!("Could not load song from {}", file_path_str).as_str()),
    };
  }

  pub fn play(&self) {
    self.music.play(-1).expect("Could not play song");
  }
}

impl Sound {
  fn new(file_path: &std::path::Path) -> Sound {
    let file_path_str = file_path.to_str().expect("Could not convert path to string");

    return Sound {
      chunk: sdl2::mixer::Chunk::from_file(file_path).expect(
          format!("Could not load sound from {}", file_path_str).as_str()),
    };
  }

  pub fn play(&self) {
    self.play_with_volume_and_pan(1.0, 0.5);
  }

  pub fn play_with_pan(&self, pan: f64) {
    self.play_with_volume_and_pan(1.0, pan);
  }

  pub fn play_with_volume(&self, volume: f64) {
    self.play_with_volume_and_pan(volume, 0.5);
  }

  pub fn play_with_volume_and_pan(&self, volume: f64, pan: f64) {
    let left: u8 = (2.0 * (1.0 - pan) * 255.0).max(0.0).min(255.0) as u8;
    let right: u8 = (2.0 * pan * 255.0).max(0.0).min(255.0) as u8;

    let channel = sdl2::mixer::Channel::all();
    channel.set_volume((128.0 * volume) as i32);
    channel.set_panning(left, right).expect(
        format!("Could not set panning with left = {} and right = {}", left, right).as_str());
    channel.play(&self.chunk, 0).expect("Could not play sound");
  }
}

impl Point {
  pub const fn new(x: i32, y: i32) -> Point {
    return Point{x: x, y: y};
  }

  pub fn zero() -> Point {
    return Point{x: 0, y: 0};
  }

  pub fn from_u32_tuple(point: (u32, u32)) -> Point {
    return Point{x: point.0 as i32, y: point.1 as i32};
  }
}
