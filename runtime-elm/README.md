# runtime-elm

这里承载 `go.support` ELM 的 Rust glue。它把 ELM 的生命周期映射到 carrier 固定 ABI，并在
后续实现中绑定 Go AOT artifact、目标 EKI 生成的 direct wrapper，以及 heap、goroutine、
timer、回调和设备资源的 owner 记录。

`go.support` 是 `m` 模式语言支持 ELM。目标设计要求它通过尚待实现的
`resident-framework` 依赖常驻 `language.runtime`，该依赖只用于 runtime ABI、generation 和
卸载编排，不转发设备调用。当前 ELM 工具只支持动态 provider 依赖，不能正确表达 `m` 对 `y`
常驻框架的依赖，因此 `Elm.toml` 刻意不填写伪造的 `language.runtime.catalog@1` 依赖。默认数据
路径仍是 `trusted-direct`；`managed` 只可显式启用。

普通暂停使用可逆的 `pause`/`resume`。卸载时调用终止性的 `quiesce`，确认 scheduler/GC 已
进入不可恢复边界，再 drain ELM owner 持有的回调与设备资源，最后调用 `finalize` 并退役镜像。
当前 crate 只提供可测试的 ABI 骨架，尚未包含真实 Go runtime 或可装载 ELM 入口。源文件说明见
[src/README.md](src/README.md)。
