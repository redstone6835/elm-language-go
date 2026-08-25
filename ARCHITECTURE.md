# Go ELM 架构

## 原则

1. Go ELM 通过预检后与 Rust ELM 一样执行原生目标代码。
2. loader 只理解通用 ELM/EBI，不增加 Go 分支。
3. `language-runtime` 是依赖与生命周期底座，不代理正常内核调用。
4. 语言 ABI 不泄漏 Go 指针、slice、interface、map、channel 或运行时内部对象。
5. 危险能力在装载和 probe 阶段授权，热路径使用已绑定的直接入口。
6. 所有 Go 代码和 goroutine 必须在 ELM 卸载前停止并到达已知 safepoint。

## 分层

### ELM Core

主仓库负责 EBI 装载、签名、Profile、重定位、能力、依赖拓扑、generation 和资源回收。Go 模块
不改变这些规则。

### language.runtime

常驻语言框架提供 attach、pause/resume、quiesce、drain、finalize 与 owner/generation 失效协议。
它不实现 Go GC，也不在每个 kernel call 上分发 operation ID。

### go.support

`runtime-elm/` 和 `runtime/` 共同构成 Go 支持层：它们为每个 Go EKI 创建独立 TamaGo runtime
instance、heap 和 goroutine 集合，注册 carrier、执行 GC safepoint，并把未处理的 runtime fault
转换为该 ELM 的 fault。`go.support` 共享实现与 ABI，不承载一个跨 EKI 的共享 Go heap。

### Go ELM

具体驱动或内核扩展以普通 ELM 形式部署。它依赖 `go.support` 的 ABI，通过生成的 Go 声明和
架构 stub 调用 Rust carrier；carrier 在装载时已经绑定审核后的 EKI kernel symbols。

## 调用路径

### trusted-direct

初始化期间解析直接导入并取得类型化设备资源。运行期间调用是 Go stub 到 carrier 再到已绑定
kernel symbol 的有限跳转，不进入全局请求队列。针对经过 ABI 审核的窄接口，后续可生成纯汇编
直达 stub，但不能绕过 EKI Profile 与装载期能力校验。

TamaGo 裸机环境不依赖 cgo。Go 侧声明、目标架构汇编 trampoline 和 Rust `extern "C"` carrier
由工具链共同生成，链接器在 EKI 打包前解析它们。

### managed

managed plane 只服务低频异步控制、诊断和宿主 fake backend。它保留固定 frame、取消与超时，
必须显式启用 feature。设备寄存器访问、DMA 提交、IRQ ack 和网络包处理不得默认走该路径。

## 运行时约束

- 每个 Go EKI 拥有一个 runtime instance 和 GC heap，属于单个 ELM owner，大小由 package manifest 固定上限；
- carrier 边界不允许传递 Go heap 指针；长期 DMA buffer 必须 pin，并由 owner 资源表登记；
- 中断顶半部不分配、不触发 GC、不阻塞，只写有界队列或原子状态；
- pause 必须到达可恢复 safepoint；quiesce 必须在截止时间内终止接收新工作并汇合；
- `finalize` 前必须完成 IRQ 注销、DMA 静默、回调解除和 goroutine join；
- Go panic 不得跨 carrier ABI 展开，必须转成状态码或当前 ELM fault；
- 反射只使用 AOT 二进制中保留的静态元数据，不支持 plugin 或运行时生成代码。

## 目标架构

官方 Go 提供 `riscv64` 和 `loong64` 后端，但 TamaGo/Hitoshizuku 的 freestanding 目标、启动代码、
链接布局和运行时 glue 仍需逐目标实现。首个里程碑只选一个架构；达到一致性测试后再启用第二个。

目标端口的验收项见 [`docs/porting/README.md`](docs/porting/README.md)。
