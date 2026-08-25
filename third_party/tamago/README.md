# TamaGo 第三方说明

本仓库依赖以下上游项目，但不在此目录内分发它们的源码或二进制：

| 项目 | 固定版本 | 来源 | 许可证 |
| --- | --- | --- | --- |
| TamaGo framework | `v1.27.0` | <https://github.com/usbarmory/tamago> | BSD 3-Clause |
| TamaGo Go distribution | `tamago-go1.27.0` | <https://github.com/usbarmory/tamago-go> | BSD 3-Clause |

`LICENSE.tamago` 和 `LICENSE.tamago-go` 是对应固定版本的上游许可证副本。
TamaGo framework 的 Go Module 内容由标准 Go Module 缓存管理；修改版 Go
编译器由官方 `go tool tamago` 启动器管理。两者都不会作为 submodule、
`vendor/` 内容或二进制归档提交到本仓库。

可选 Linux amd64 发布包的文件名和 SHA-256 记录在
`toolchain/tamago/toolchain.lock.toml`，仅用于离线供应链核验。
