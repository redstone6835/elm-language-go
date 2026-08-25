# 示例

- [`hello/`](hello/README.md)：最小 trusted-direct 生命周期；
- [`mmio-driver/`](mmio-driver/README.md)：危险资源在 probe 时绑定、热路径直达；
- [`managed-control/`](managed-control/README.md)：显式可选的低频控制面。

当前示例是接口规划，不是可装载 artifact。只有通过目标端口、EKI pack/sign 和 QEMU/硬件测试
后才会加入可执行源码。
