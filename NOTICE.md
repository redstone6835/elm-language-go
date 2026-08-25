# 第三方说明

本仓库自身代码采用 `GPL-3.0-only`。

- Go 项目由 The Go Authors 维护，使用 BSD 风格许可证；
- TamaGo 由 usbarmory 项目维护，使用其上游仓库声明的许可证；
- 本仓库通过 Go Modules 和 `go tool tamago` 引用上游，不复制或重新授权上游源码；
- 下载的工具链和生成物不提交到 Git，其许可证与校验信息由 `third_party/` 和工具链锁记录。

发布二进制或源代码归档时，必须同时携带实际使用版本对应的上游许可证文本和 source offer。
