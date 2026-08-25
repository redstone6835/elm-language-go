# elm-language-go

`elm-language-go` 是 Hitoshizuku ELM 的 Go 语言支持仓库。它以
[TamaGo](https://github.com/usbarmory/tamago) 为 freestanding Go 与静态 AOT 基线，提供
`go.support` ELM、固定 carrier ABI、Go SDK、工具链锁定和一致性测试。

仓库当前处于接口与移植骨架阶段，还不能生成可装载的 Go ELM。首个可运行版本需要完成
RISC-V 64 或 LoongArch 64 的 TamaGo 目标移植、ELM carrier 链接和运行时退出协议。

## 设计定位

Go ELM 是受信任的原生内核模块，不是由 `language-runtime` 解释或代理的脚本：

```text
Go ELM source
    -> TamaGo static AOT
    -> generated Go ABI stubs
    -> Rust carrier
    -> audited EKI kernel symbols
    -> kernel/general/hal
```

模块进入内核前，主机工具和 ELM loader 校验目标架构、EBI/EKI、Kernel API Profile、签名、
重定位、直接导入、能力和运行时元数据。校验和绑定完成后，普通函数、MMIO、DMA、IRQ 与网络
数据面直接执行，不经过 `language-runtime` 的 request queue、operation dispatcher 或逐次 wire
编解码。

目标依赖关系仍然保留：

```text
language.runtime (resident framework)
    -> go.support (TamaGo runtime + carrier)
        -> concrete Go ELM
```

`language.runtime` 提供语言无关的 attach、generation、pause/resume、quiesce、drain、finalize 和资源归属撤销；
`go.support` 为每个 Go EKI 建立并管理独立的 TamaGo runtime instance、heap、goroutine 集合、
TLS 和静态反射元数据。支持 ELM 共享代码和 ABI，不共享不同 EKI 的 Go heap 或 runtime 状态。
`managed` 传输只用于
显式选择的低频控制面和宿主测试，不是驱动热路径，也不是安全沙箱。

当前 ELM 清单还不能表达 `m` 支持模块对 `y` resident framework 的依赖，因此这一依赖暂时只
作为目标契约记录；仓库不会用 managed catalog provider 冒充已实现的父依赖。

## TamaGo 接入

仓库采用 TamaGo 官方推荐的 Go Modules 工具依赖方式：

- `go.mod` 固定 `github.com/usbarmory/tamago` 版本；
- `tool github.com/usbarmory/tamago/cmd/tamago` 声明工具；
- 所有命令通过 `go tool tamago` 调用与模块版本匹配的修改版 Go 工具链；
- 不提交 TamaGo 源码副本，不使用 Git submodule，也不把预编译编译器放进 Git；
- 必须修改上游时，补丁按顺序保存在 `toolchain/tamago/patches/`，并优先向上游提交。

版本、来源和校验信息见
[`toolchain/tamago/toolchain.lock.toml`](toolchain/tamago/toolchain.lock.toml)。

## 目录

| 路径 | 职责 |
| --- | --- |
| `carrier/` | Rust `no_std` carrier ABI、生命周期表和可选 managed 控制面 |
| `runtime-elm/` | `go.support` ELM 的 Rust glue 与 ELM 清单 |
| `runtime/` | TamaGo 到 Hitoshizuku 的 PAL、GC、调度、反射适配边界 |
| `sdk/` | Go 风格的直接内核 API、危险操作 API 和可选 managed API |
| `internal/` | 仅供本仓库工具使用的 Go 内部包与 TamaGo module anchor |
| `toolchain/` | TamaGo 获取、验证、patch 和 EKI/Go AOT 编排 |
| `manifests/` | 语言包和 bridge 清单模板 |
| `schemas/` | 生成的接口 schema 约束与示例 |
| `examples/` | 最小模块、MMIO 驱动和 managed 控制面示例规划 |
| `tests/` | ABI、生命周期、工具链与双架构一致性测试 |
| `third_party/` | 上游来源、许可证和版本说明，不存放 vendored 源码 |
| `docs/` | 架构决策和新目标移植指南 |

每一级已跟踪目录都包含自己的 `README.md`，目录职责不依赖这张总表才能理解。

## 快速开始

宿主机需要 Rust 工具链、支持 `tool` directive 的 Go 工具链、Git、`curl` 和 `sha256sum`。

```sh
git clone https://github.com/redstone6835/elm-language-go.git
cd elm-language-go

go mod download
go tool tamago version
./toolchain/scripts/verify-toolchain.sh
./toolchain/scripts/tamago-target.sh riscv64 env GOOS GOARCH GOOSPKG

cargo test --workspace --all-features --locked
./toolchain/scripts/check-layout.sh
```

当前快速开始只验证依赖、carrier ABI 和仓库结构。真实 ELM 构建将在目标端口完成后使用：

```text
EKI Profile -> interface schema -> Go SDK/stubs
            -> go tool tamago build -> external link with carrier
            -> EKI pack/sign -> package-check
```

## 支持范围

首期目标范围只包含 freestanding、静态 AOT 的 Go：不使用 JIT、plugin、动态装载或宿主操作系统 syscall。
Go GC 与有限反射可以保留，但必须满足有界 heap、可达 safepoint、卸载前停止所有 goroutine、
禁止跨 ELM 保存 Go 指针等约束。详见 [ARCHITECTURE.md](ARCHITECTURE.md)。

## 许可证

本仓库以 `GPL-3.0-only` 发布。TamaGo 与 Go 保持各自上游许可证，详见
[NOTICE.md](NOTICE.md) 和 [`third_party/`](third_party/README.md)。
