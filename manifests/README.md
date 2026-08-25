# 清单模板

这里明确分开两套尚未统一的清单：

- `LanguagePackage.managed.shape-only.invalid.toml` 与
  `LanguageBridge.managed.shape-only.invalid.toml` 只记录当前 cargo-elm schema v2 的 managed
  字段形状；它们缺少真实 EKI/symbol，属于预期失败 fixture；
- `LanguagePackage.trusted-direct.target.toml` 与 `DirectBindings.target.toml` 是目标设计草案，
  描述 resident framework 和直接 symbol，但当前 cargo-elm 不接受这些字段。

invalid fixture 的 digest、target、artifact 和签名均为占位值，必须在 `package-check` 中失败。
不能把 managed operation schema 作为 trusted-direct carrier schema 使用。

正式生成物由 host 工具写入 build 目录并绑定：

- `language.runtime` 与 `go.support` ABI 版本；
- execution plane（默认 `trusted-direct`）；
- EKI Profile 与 carrier ABI digest；
- target、artifact hash、签名和工具链锁；
- direct imports、capabilities、heap/goroutine/DMA/IRQ limits；
- GC、TLS、stack maps、panic 与反射元数据。
