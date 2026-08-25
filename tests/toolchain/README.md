# 工具链测试

本目录测试工具链锁、下载摘要、patch 顺序、module graph、生成器确定性和离线重建。测试不得
隐式信任 PATH 中的 `go` 或缓存中的旧 TamaGo；实际版本必须与锁文件匹配。

`manifests/` 与 `schemas/` 中标有 `shape-only.invalid` 的文件是预期失败 fixture：将来接入真实
`cargo elm package-check` 测试时，必须断言它们因缺失 artifact、symbol 或类型图不一致而失败，
不能把“能解析 TOML/JSON”当成 package 有效。
