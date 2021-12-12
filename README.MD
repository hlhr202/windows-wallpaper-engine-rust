# Simple rust version of Windows animated wallpaper engine

- Dependencies:
  - [rust win api](https://github.com/microsoft/windows-rs)
  - ffplay (windows binary)
  - Windows C++ Build tools (as win rust dependencies)

- Pre install:
  - download ffplay from [FFBinaries](https://ffbinaries.com/downloads)
  - extract ffplay.exe under the root folder of this project (same folder as Cargo.toml)

- Run:
  - cargo run -- "path_of_your_video.mp4"


## Credits
[The In-depth Analysis of Windows Wallpaper Engine](https://www.bilibili.com/video/BV1HZ4y1978a)