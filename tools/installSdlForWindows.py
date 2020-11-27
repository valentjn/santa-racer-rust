#!/usr/bin/python3

# Copyright (C) 2020 Julian Valentin
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import os
import shutil
import tempfile
import urllib.request
import zipfile



def processUrl(url):
  repoDirPath = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
  dstDllDirPath = os.path.join(repoDirPath, "msvc", "dll", "x64")
  dstLibDirPath = os.path.join(repoDirPath, "msvc", "lib", "x64")
  os.makedirs(dstDllDirPath, exist_ok=True)
  os.makedirs(dstLibDirPath, exist_ok=True)

  with tempfile.TemporaryDirectory() as tmpDirPath:
    archiveFilePath = os.path.join(tmpDirPath, "archive.zip")
    print(f"Downloading '{url}' to '{archiveFilePath}'...")

    with urllib.request.urlopen(url) as srcFile, open(archiveFilePath, "wb") as dstFile:
      shutil.copyfileobj(srcFile, dstFile)

    print(f"Extracting '{archiveFilePath}' to '{tmpDirPath}'...")
    with zipfile.ZipFile(archiveFilePath, 'r') as f: f.extractall(tmpDirPath)

    srcDirPath = None

    for dirName in sorted(os.listdir(tmpDirPath)):
      if os.path.isdir(os.path.join(tmpDirPath, dirName)):
        srcDirPath = os.path.join(tmpDirPath, dirName, "lib", "x64")
        break

    for fileName in sorted(os.listdir(srcDirPath)):
      fileExtension = os.path.splitext(fileName)[1]
      srcFilePath = os.path.join(srcDirPath, fileName)

      if fileExtension == ".dll":
        dstFilePath = os.path.join(dstDllDirPath, fileName)
      elif fileExtension == ".lib":
        dstFilePath = os.path.join(dstLibDirPath, fileName)
      else:
        continue

      print(f"Moving '{srcFilePath}' to '{dstFilePath}'...")
      if os.path.isfile(dstFilePath): os.remove(dstFilePath)
      shutil.move(srcFilePath, dstFilePath)



def main():
  urls = [
        "https://www.libsdl.org/release/SDL2-devel-2.0.12-VC.zip",
        "https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.0.5-VC.zip",
        "https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-devel-2.0.4-VC.zip",
      ]

  for url in urls:
    processUrl(url)



if __name__ == "__main__":
  main()
