
# Journey Map Merger

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

[中文](README_zh.md)

## About The Project

Journey Map Merger is a tool for merging Journey Map data files, which can help players share map data between multiple clients. Journey Map is a map mod for Minecraft, which can record the areas that players have explored and provide various features and settings.

## Getting Started

- Download the merger and put it to the folder where `.minecraft` is located.

![install-screenshot]

- Make sure you have installed the Journey Map mod and have played at least once on the server you want to merge.

## Usage

### Automatic export mode

- Double-click to run merger.exe, the merger will automatically scan all Journey Map data files in all clients and merge them into one file.
- The merged file will be named after the server name, saved in the folder where `.minecraft` is located, such as `your server name.bin`.

![exported-screenshot]

- You can share this file with other players or back it up.

### Automatic import mode

1. Disconnect from the Minecraft server.
2. Drag the merged data file you want to import (such as your server name.bin) to merger.exe, or enter `./journeymerger.exe "your server name.bin"` in cmd/ps.
3. The merger will automatically import this file into all clients and overwrite the original data files.
4. You can enter the corresponding server, and view the map data.

## Roadmap

- [ ] marge Journey Map data files
  - [x] export map and waypoints
  - [x] import and merge map
  - [ ] import and merge waypoints
- [ ] marge Visual Prospecting data files
  - [ ] coming soon

## License

Distributed under the WTFPL License. See `LICENSE.txt` for more information.

[contributors-shield]: https://img.shields.io/github/contributors/YaoerWu/journeymerger.svg?style=for-the-badge
[contributors-url]: https://github.com/YaoerWu/journeymerger/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/YaoerWu/journeymerger.svg?style=for-the-badge
[forks-url]: https://github.com/YaoerWu/journeymerger/network/members
[stars-shield]: https://img.shields.io/github/stars/YaoerWu/journeymerger.svg?style=for-the-badge
[stars-url]: https://github.com/YaoerWu/journeymerger/stargazers
[issues-shield]: https://img.shields.io/github/issues/YaoerWu/journeymerger.svg?style=for-the-badge
[issues-url]: https://github.com/YaoerWu/journeymerger/issues
[license-shield]: https://img.shields.io/github/license/YaoerWu/journeymerger.svg?style=for-the-badge
[license-url]: https://github.com/YaoerWu/journeymerger/blob/main/LICENSE.txt
[install-screenshot]: images/install_screenshot.png
[exported-screenshot]: images/exported_screenshot.png
