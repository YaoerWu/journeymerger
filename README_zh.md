# Journey Map 合并器

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

## 关于本项目

Journey Map Merger是一个用于合并Journey Map的数据文件的工具，它可以帮助玩家在多个客户端之间共享地图数据。Journey Map是一个Minecraft的地图模组，它可以记录玩家探索过的区域，并提供各种功能和设置。

## 开始使用

- 下载合并器的压缩包，解压到`.minecraft`所在的文件夹中。

![install-screenshot]

- 确保你已经安装了Journey Map模组，并且在你想要合并的服务器上至少玩过一次。
- 断开与Minecraft服务器的连接。

## 使用方法

### 自动导出模式

- 双击运行合并器.exe，合并器将自动扫描所有客户端中的Journey Map数据文件，并将它们合并为一个文件。
- 合并后的文件将以服务器名字命名，保存在`.minecraft`所在的文件夹中，例如`your server name.bin`。

![exported-screenshot]

- 你可以将这个文件分享给其他玩家，或者备份起来。

### 自动导入模式

- 将你想要导入的合并后的数据文件（例如`your server name.bin`）拖到合并器.exe上，或者在cmd/ps中输入`./journeymerger.exe "your server name.bin"`。
- 合并器将自动将这个文件导入到所有客户端中，并覆盖原有的数据文件。
- 你可以打开Minecraft客户端，进入相应的服务器，查看地图数据。

## 开发计划

- [ ] 合并 Journey Map 数据文件
  - [x] 导出地图和路径点
  - [x] 导入和合并地图
  - [ ] 导入和合并路径点
- [ ] 合并 Visual Prospecting 数据文件
  - [ ] 敬请期待

## 许可证

本项目使用 WTFPL 许可证分发。更多信息请查看 LICENSE.txt 文件。

[contributors-shield]: https://img.shields.io/github/contributors/YaoerWu/journeymerger.svg?style=for-the-badge
[contributors-url]: https://github.com/YaoerWu/journeymerger/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/YaoerWu/journeymerger.svg?style=for-the-badge
[forks-url]: https://github.com/YaoerWu/journeymerger/network/members
[stars-shield]: https://img.shields.io/github/stars/YaoerWu/journeymerger.svg?style=for-the-badge
[stars-url]: https://github.com/YaoerWu/journeymerger/stargazers
[issues-shield]: https://img.shields.io/github/issues/YaoerWu/journeymerger.svg?style=for-the-badge
[issues-url]: https://github.com/YaoerWu/journeymerger/issues
[license-shield]: https://img.shields.io/github/license/YaoerWu/journeymerger.svg?style=for-the-badge
[license-url]: https://github.com/YaoerWu/journeymerger/blob/master/LICENSE.txt
[install-screenshot]: images/install_screenshot.png
[exported-screenshot]: images/exported_screenshot.png