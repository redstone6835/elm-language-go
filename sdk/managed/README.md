# Managed 控制面

本目录只在显式 `managed` 构建中提供固定 frame 的 submit、poll、cancel 与诊断 API。适用场景是
宿主 fake backend、低频异步管理和实验性受限执行面。

禁止把 managed transport 用作常规 MMIO、DMA、IRQ ack、包收发或文件系统数据路径。它不提供
地址空间隔离；需要运行不可信代码时，应使用进程、虚拟机或硬件隔离。
