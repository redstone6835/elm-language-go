# MMIO 驱动示例

该示例将演示内核模块式的数据路径：probe 阶段取得已授权 MMIO region、IRQ 和 DMA buffer，
运行阶段通过生成的 direct wrapper 访问寄存器和 ring，不逐次进入 LR dispatcher。

验收必须覆盖地址范围、对齐、memory ordering、DMA ownership、IRQ teardown、设备静默和卸载后
回调失效。示例不会直接硬编码物理地址。
