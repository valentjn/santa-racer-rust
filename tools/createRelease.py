#!/usr/bin/python3

# Copyright (C) 2020 Julian Valentin
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import os
import re
import shutil
import subprocess
import tempfile


def copyFile(srcFilePath, dstPath):
  print(f"Copying '{srcFilePath}' to '{dstPath}'...")
  shutil.copy(srcFilePath, dstPath)



def main():
  repoDirPath = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
  version = os.environ["SANTA_RACER_VERSION"]
  platform = ("linux" if os.name == "posix" else "windows")
  releaseName = f"santa-racer-{version}-{platform}"

  with tempfile.TemporaryDirectory() as tmpDirPath:
    dstDirPath = os.path.join(tmpDirPath, releaseName)
    os.mkdir(dstDirPath)

    copyFile(os.path.join(repoDirPath, "LICENSE.md"),
        os.path.join(dstDirPath, "LICENSE.santa-racer.md"))
    copyFile(os.path.join(repoDirPath, "README.md"), dstDirPath)

    exeFileExtension = ("" if os.name == "posix" else ".exe")
    exeFilePath = os.path.join(repoDirPath, "target", "release", f"santa-racer{exeFileExtension}")
    copyFile(exeFilePath, dstDirPath)

    srcLicenseDirPath = os.path.join(repoDirPath, "licenses")

    for srcLicenseFileName in sorted(os.listdir(srcLicenseDirPath)):
      copyFile(os.path.join(srcLicenseDirPath, srcLicenseFileName), dstDirPath)

    if os.name == "posix":
      lddOutput = subprocess.run(["ldd", exeFilePath], stdout=subprocess.PIPE).stdout.decode()
      srcDllFilePaths = re.findall(r"^.*? => (.*) \(.*?\)$", lddOutput, flags=re.MULTILINE)
      srcDllFilePaths = [x for x in srcDllFilePaths
          if os.path.basename(x).split(".so")[0] in
          ["libSDL2-2.0", "libSDL2_mixer-2.0", "libSDL2_image-2.0", "libfluidsynth",
            "libinstpatch-1.0", "libjbig", "libjpeg", "liblzma", "libpng16", "libtiff",
            "libwebp", "libz", "libzstd"]]
    else:
      srcDllDirPath = os.path.join(repoDirPath, "msvc", "dll", "x64")
      srcDllFilePaths = [os.path.join(srcDllDirPath, x) for x in sorted(os.listdir(srcDllDirPath))]

    for srcDllFilePath in sorted(srcDllFilePaths):
      copyFile(srcDllFilePath, dstDirPath)

    archiveFileExtension = (".tar.gz" if os.name == "posix" else ".zip")
    archiveFilePath = os.path.join(repoDirPath, f"{releaseName}{archiveFileExtension}")
    print(f"Creating '{archiveFilePath}'...")
    archiveType = ("gztar" if os.name == "posix" else "zip")
    shutil.make_archive(releaseName, archiveType, tmpDirPath)



if __name__ == "__main__":
  main()
