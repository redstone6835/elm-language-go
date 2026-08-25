# 生命周期测试

生命周期矩阵至少覆盖 initialize、pause/resume、quiesce、cancel、drain、finalize、失败回滚、重复调用、
超时、panic、仍有 goroutine、IRQ/DMA 未静默和 stale generation。任何路径都不能在 image 卸载后
继续执行 Go runtime 或 carrier callback。
