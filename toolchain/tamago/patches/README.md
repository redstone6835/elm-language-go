# 补丁队列

当前没有补丁。`toolchain.lock.toml` 因此声明 `patch_set = "none"`，官方
`go tool tamago` 下载并构建未经修改的 `tamago-go1.27.0`。

只有当 ELM 所需能力无法通过上游 `runtime/goos` 覆盖或普通 Go 包完成时，
才可以在这里引入编译器补丁。补丁采用常见的有序邮件格式命名：

```text
0001-short-description.patch
0002-next-change.patch
```

引入第一份补丁前必须同时完成以下工作：

- 将补丁提交上游或记录无法上游化的原因；
- 把锁文件中的 `patch_set` 改成稳定标识；
- 提供显式应用补丁并构建工具链的脚本；
- 禁止继续声称官方 `go tool tamago` 路径可复现该补丁产物；
- 在 CI 中验证补丁按编号顺序干净应用到锁定标签。

不要在本目录提交工具链源码、构建目录或二进制文件。
