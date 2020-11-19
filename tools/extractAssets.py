#!/usr/bin/python3

# Copyright (C) 2008-2020 Julian Valentin
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import os
import shutil
import subprocess
import tempfile



fileNames = {
      "2_3000_0.bmp" : "sleigh.png",
      "2_3001_0.bmp" : "reindeer.png",
      "2_3002_0.bmp" : "gift1.png",
      "2_3003_0.bmp" : "gift2.png",
      "2_3004_0.bmp" : "shield.png",
      "2_3005_0.bmp" : "shieldBalloon.png",
      "2_3006_0.bmp" : "gift3.png",
      "2_3007_0.bmp" : "star.png",
      "2_3008_0.bmp" : "smallStar.png",
      "2_3009_0.bmp" : "snowman.png",
      "2_3010_0.bmp" : "goblin.png",
      "2_3011_0.bmp" : "goblinSnowball.png",
      "2_3013_0.bmp" : "font.png",
      #"2_3014_0.bmp" : "goblin.png",
      "2_3015_0.bmp" : "electrocutedSleigh.png",
      "2_3016_0.bmp" : "electrocutedReindeer.png",
      "2_3017_0.bmp" : "wineBalloon.png",
      "2_3018_0.bmp" : "cashBalloon.png",
      "2_3019_0.bmp" : "giftBalloon.png",
      "2_3020_0.bmp" : "heartBalloon.png",
      "2_3021_0.bmp" : "bigStar.png",
      "2_3022_0.bmp" : "points10.png",
      "2_3023_0.bmp" : "points15.png",
      "2_3024_0.bmp" : "points20.png",
      #"2_3025_0.bmp" : "points50.png",
      "2_3026_0.bmp" : "angel.png",
      "2_3027_0.bmp" : "cloud.png",
      "2_3028_0.bmp" : "drunkStar.png",
      "2_3029_0.bmp" : "smallDrunkStar.png",
      #"2_3031_0.bmp" : "airshipAd.png",
      "2_3034_0.bmp" : "timeScoreIcon.png",
      "2_3035_0.bmp" : "damageScoreIcon.png",
      "2_3036_0.bmp" : "giftScoreIcon.png",
      "2_3037_0.bmp" : "finish.png",
      #"2_3038_0.bmp" : "eonAd.png",
      #"2_3039_0.bmp" : "youtradeAd.png",
      #"2_3040_0.bmp" : "bahlsenAd.png",
      #"2_3041_0.bmp" : "andersSeimAd.png",
      #"2_3042_0.bmp" : "bildschirmschonerDeAd.png",
      #"2_3043_0.bmp" : "smallBildschirmschonerDeAd.png",
      #"2_3050_0.bmp" : "preview.png",
      "2_3051_0.bmp" : "landscape.png",
      #"2_3052_0.bmp" : "bunny.png",
      "2_3053_0.bmp" : "background.png",
      "CDXMAPPY_5000_0" : "level.png",
      #"JPEG_9002_0" : "lostDueToAlcohol.png",
      "JPEG_9003_0" : "lostDueToDamageSplash.png",
      "JPEG_9004_0" : "wonSplash.png",
      "JPEG_9005_0" : "lostDueToTimeSplash.png",
      "JPEG_9006_0" : "helpSplash1.png",
      "JPEG_9007_0" : "helpSplash2.png",
      #"JPEG_9008_0" : "titleSplash.png",
      "MIDI_10000_0" : "music.ogg",
      "WAV_8000_0" : "giftCollidedWithGround.wav",
      "WAV_8001_0" : "snowmanLaunch.wav",
      "WAV_8002_0" : "wineBalloon.wav",
      "WAV_8003_0" : "sleighCollidedWithCloud.wav",
      "WAV_8004_0" : "cashBalloon.wav",
      "WAV_8006_0" : "sleighCollidedWithLevelTile1.wav",
      "WAV_8007_0" : "sleighCollidedWithLevelTile2.wav",
      "WAV_8008_0" : "dog.wav",
      "WAV_8009_0" : "bell.wav",
      "WAV_8010_0" : "goblinThrowSnowball.wav",
      "WAV_8011_0" : "sleighCollidedWithNpc.wav",
      "WAV_8012_0" : "giftCollidedWithChimney.wav",
      "WAV_8013_0" : "shieldBalloon.wav",
      "WAV_8014_0" : "giftBalloon.wav",
      "WAV_8015_0" : "won.wav",
      "WAV_8016_0" : "lost.wav",
    }



def convertFmapToPpm(fmapFilePath):
  with open(fmapFilePath, "rb") as f: fmap = f.read()
  i = fmap.index(b"BGFX")
  i += 8
  i += 32768
  j = fmap.index(b"LYR1", i)
  fmapPixelStorage = fmap[i:j]
  numberOfPixels = len(fmapPixelStorage) // 2
  ppmPixelStorage = (numberOfPixels * 3) * [0]
  round5Bits = (lambda x: (x if x < 248 else 255))
  round6Bits = (lambda x: (x if x < 252 else 255))

  for k in range(numberOfPixels):
    fmapColor = int.from_bytes(fmapPixelStorage[2*k:2*k+2], "big")
    ppmPixelStorage[3*k] = round5Bits((fmapColor & 0xf800) >> 8)
    ppmPixelStorage[3*k+1] = round6Bits((fmapColor & 0x07e0) >> 3)
    ppmPixelStorage[3*k+2] = round5Bits((fmapColor & 0x001f) << 3)

  ppmPixelStorage = bytes(ppmPixelStorage)
  width = 128
  height = numberOfPixels // width

  ppm = b"P6 " + str(width).encode() + b" " + str(height).encode() + b" 255 " + ppmPixelStorage
  with open(fmapFilePath, "wb") as f: f.write(ppm)



def decodeRunLengthEncoding(bmpFilePath):
  with open(bmpFilePath, "rb") as f: bmp = f.read()
  getInt = (lambda x, y: int.from_bytes(bmp[x:y], "little"))
  getWord = (lambda x: getInt(x, x + 2))
  getDword = (lambda x: getInt(x, x + 4))
  headerSize = getDword(0)
  compressionMethod = getDword(16)
  if (headerSize != 40) or (compressionMethod == 0): return

  width = getDword(4)
  height = getDword(8)
  assert getWord(12) == 1
  assert getWord(14) == 8
  assert compressionMethod == 1
  rlePixelStorageSize = getDword(20)
  assert getWord(32) == 256

  rlePixelStorage = bmp[-rlePixelStorageSize:]
  decodedPixelStorage = (width * height) * [0]
  i, x, y = 0, 0, 0

  while i < rlePixelStorageSize:
    count = rlePixelStorage[i]
    i += 1

    if count > 0:
      byte = rlePixelStorage[i]
      i += 1

      for k in range(count):
        decodedPixelStorage[x + y * width] = byte
        x += 1
    else:
      count = rlePixelStorage[i]
      i += 1

      if count == 0:
        x = 0
        y += 1
      elif count == 1:
        break
      elif count == 2:
        x += rlePixelStorage[i]
        i += 1
        y += rlePixelStorage[i]
        i += 1
      else:
        for k in range(count):
          decodedPixelStorage[x + y * width] = rlePixelStorage[i]
          i += 1
          x += 1

        if count % 2 == 1: i += 1

  decodedPixelStorage = bytes(decodedPixelStorage)
  bmp = (bmp[:16] + bytes(4) + int.to_bytes(len(decodedPixelStorage), 4, "little") +
      bmp[24:-rlePixelStorageSize] + decodedPixelStorage)
  with open(bmpFilePath, "wb") as f: f.write(bmp)




def main():
  baseName = "NikolausExpress2000"
  rootDirPath = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
  exeFilePath = os.path.join(rootDirPath, "original", f"{baseName}.exe")
  print("Extracting compressed SCR file from EXE...")

  with open(exeFilePath, "rb") as f:
    f.seek(64768)
    compressedScr = f.read(3388131)

  with tempfile.TemporaryDirectory() as tmpDirPath:
    print("Extracting SCR file from compressed SCR file...")
    compressedScrFilePath = os.path.join(tmpDirPath, f"{baseName}.scr_")
    with open(compressedScrFilePath, "wb") as f: f.write(compressedScr)
    subprocess.run(["7z", "x", f"-o{tmpDirPath}", compressedScrFilePath],
        stdout=subprocess.DEVNULL)

    print("Extracting resources from SCR file...")
    scrFileName = f"{baseName}.scr"
    scrFilePath = os.path.join(tmpDirPath, scrFileName)
    # install Arch package icoutils
    subprocess.run(["wrestool", "--extract", "--raw", f"--output={tmpDirPath}", scrFilePath])

    assetsDirName = "assets"
    dstDirNames = {
            ".ogg" : "songs",
            ".png" : "images",
            ".wav" : "sounds",
          }
    srcSuffixes = sorted(list(fileNames.keys()))

    for srcSuffix in srcSuffixes:
      dstFileName = fileNames[srcSuffix]
      _, dstExtension = os.path.splitext(dstFileName)
      dstDirName = dstDirNames[dstExtension]
      print(f"Extracting {assetsDirName}/{dstDirName}/{dstFileName}...")

      dstDirPath = os.path.join(rootDirPath, assetsDirName, dstDirName)
      os.makedirs(dstDirPath, exist_ok=True)

      srcFilePath = os.path.join(tmpDirPath, f"{scrFileName}_{srcSuffix}")
      dstFilePath = os.path.join(dstDirPath, dstFileName)

      if dstExtension == ".ogg":
        # install Arch packages timidity and soundfont-fluid,
        # add "soundfont /usr/share/soundfonts/FluidR3_GM.sf2"
        # to /etc/timidity/timidity.cfg
        subprocess.run(["timidity", "--quiet=2", "--segment=0-86", "--reverb=d",
            "-Ov", f"--output-file={dstFilePath}", srcFilePath])
      elif dstExtension == ".png":
        if dstFileName == "level.png":
          convertFmapToPpm(srcFilePath)
        else:
          decodeRunLengthEncoding(srcFilePath)

        subprocess.run(["convert", srcFilePath, "-transparent", "magenta", f"png32:{dstFilePath}"])

        if dstFileName == "level.png":
          subprocess.run(["convert", dstFilePath, "-fill", "#000048",
              "-draw", "rectangle 47,8078 124,8092",
              "-draw", "rectangle 107,8075 124,8077",
              "-family", "Palatino", "-style", "Normal", "-pointsize", "14",
              "-fill", "white", "-gravity", "NorthWest",
              "-annotate", "-1x-1+51+8080", "Frohes Fest", dstFilePath])
        elif dstFileName in ["bigStar.png", "drunkStar.png", "electrocutedReindeer.png",
              "electrocutedSleigh.png", "shield.png", "smallDrunkStar.png", "smallStar.png",
              "star.png"]:
          subprocess.run(["convert", dstFilePath, "-alpha", "copy", dstFilePath])
      elif dstExtension == ".wav":
        shutil.copyfile(srcFilePath, dstFilePath)



if __name__ == "__main__":
  main()
