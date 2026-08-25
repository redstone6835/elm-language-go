# 测试

- [`abi/`](abi/README.md)：carrier 布局、endianness 和 golden vectors；
- [`lifecycle/`](lifecycle/README.md)：启动、取消、quiesce、卸载和 generation；
- [`conformance/`](conformance/README.md)：SDK、EKI、工具链与双架构行为；
- [`toolchain/`](toolchain/README.md)：TamaGo 来源、patch、可复现构建和负面检查。

host 测试、QEMU 测试与真实硬件测试必须分开报告，不能用 host fake 代替目标端验收。
