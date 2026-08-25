# 工具链脚本

`verify-toolchain.sh` 与 `check-layout.sh` 只做审计和校验，不安装依赖。
`tamago-target.sh` 是构建入口，首次运行会按 TamaGo 官方行为在用户缓存中下载并编译匹配的
`tamago-go`；它不会把工具链写进仓库。

## `verify-toolchain.sh`

无参数运行时，它检查 Go Module、版本锚点、工具链锁和空补丁集的仓库元数据是否一致：

```sh
./toolchain/scripts/verify-toolchain.sh
```

传入已经存在的官方 Linux amd64 发布包时，还会验证 SHA-256：

```sh
./toolchain/scripts/verify-toolchain.sh \
  /path/to/tamago-go1.27.0.linux-amd64.tar.gz
```

脚本不联网、不下载、不解压，也不修改 Go 缓存或仓库内容。它只验证已记录的 pin；实际目标
入口由 `tamago-target.sh` 再核对 launcher checkout 的 commit。

## `check-layout.sh`

该脚本检查所有已跟踪用途目录都有 `README.md`，并拒绝 Git submodule、
`vendor/`、构建产物和工具链归档进入仓库：

```sh
./toolchain/scripts/check-layout.sh
```

## `tamago-target.sh`

该脚本清空可能改变命令形态的 `GOFLAGS`，从根 `go.mod` 的 `tool` 声明解析单一宿主 launcher，
再把 TamaGo 目标环境传给 launcher，避免把 launcher 本身交叉编译：

```sh
./toolchain/scripts/tamago-target.sh riscv64 env GOOS GOARCH GOOSPKG
./toolchain/scripts/tamago-target.sh riscv64 build ./examples/hello
```

它不固定链接地址或 ELM 打包参数；这些参数由未来的 target manifest 提供。
`loong64` 在对应 TamaGo/Hitoshizuku 端口完成前只能用于明确的移植实验。
