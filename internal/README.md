# internal

这里保存仓库私有的 Go 实现细节。Go 的 `internal` 导入规则保证仓库外部的
模块不能把这些包误当作稳定 SDK。

当前子目录：

- `tamagoanchor/`：集中记录 TamaGo 框架与编译器版本之间的对应关系。

对 ELM 作者公开的 API 应放入 `sdk/`，不应从这里导出。
