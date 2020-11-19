/* Copyright (C) 2008-2020 Julian Valentin
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

fn main() {
  let target = std::env::var("TARGET").expect("Could not get TARGET environment variable");

  if target.contains("pc-windows") {
    let manifest_dir_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect(
        "Could not get CARGO_MANIFEST_DIR environment variable"));
    let mut lib_dir_path = manifest_dir_path.clone();
    let mut dll_dir_path = manifest_dir_path.clone();

    if target.contains("msvc") {
      lib_dir_path.push("msvc");
      dll_dir_path.push("msvc");
    } else {
      lib_dir_path.push("gnu-mingw");
      dll_dir_path.push("gnu-mingw");
    }

    lib_dir_path.push("lib");
    dll_dir_path.push("dll");

    if target.contains("x86_64") {
      lib_dir_path.push("x64");
      dll_dir_path.push("x64");
    } else {
      lib_dir_path.push("x86");
      dll_dir_path.push("x86");
    }

    println!("cargo:rustc-link-search=all={}", lib_dir_path.display());

    for src_entry in std::fs::read_dir(dll_dir_path).expect("Could not read dir") {
      let src_entry_path = src_entry.expect("Could not get directory entry").path();
      let src_file_name = src_entry_path.file_name();
      let mut dst_file_path = manifest_dir_path.clone();

      if let Some(src_file_name) = src_file_name {
        let src_file_name = src_file_name.to_str().expect("Could not convert path to string");

        if src_file_name.ends_with(".dll") {
          dst_file_path.push(src_file_name);
          std::fs::copy(&src_entry_path, dst_file_path.as_path()).expect(
              format!("Could not copy {} to {}", src_entry_path.display(),
              dst_file_path.display()).as_str());
        }
      }
    }
  }
}
