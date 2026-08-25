# Managed 控制面示例

该示例只用于证明可选 managed frame 的 submit、poll、cancel 与超时。它不会访问设备热路径，
也不会把 managed transport 当成安全隔离。默认构建和默认 Go SDK不包含此能力。
