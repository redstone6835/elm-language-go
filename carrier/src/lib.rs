#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

//! Go AOT runtime 与 `go.support` ELM glue 之间的固定生命周期 ABI。
//!
//! 该 ABI 只承载语言 runtime 生命周期。设备热路径通过生成的 trusted-direct kernel symbol
//! wrapper 调用，不经过这里的通用分发器。

/// carrier ABI 主版本。
pub const GO_CARRIER_ABI_VERSION: u32 = 1;

/// 生命周期函数成功完成。
pub const GO_CARRIER_STATUS_OK: i32 = 0;

/// 经过 loader 可执行段、relocation 和 ABI 校验后使用的生命周期调用签名。
///
/// artifact 表中只保存 [`GoEntryPointV1`] 的整数地址，调用门不得在验证完成前构造此函数值。
/// `context` 必须非空且匹配正在执行的 phase；返回零表示成功，非零由 `go.support` 转成 ELM
/// lifecycle hook failure。异常或 Go panic 不得跨越此 C ABI。
pub type GoLifecycleHookV1 = unsafe extern "C" fn(context: *const GoLifecycleContextV1) -> i32;

#[cfg(feature = "managed")]
mod managed;

#[cfg(feature = "managed")]
pub use managed::*;

/// Go runtime 生命周期函数当前执行的阶段。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GoLifecyclePhase {
    /// 创建 runtime、heap、scheduler 和静态元数据。
    Initialize = 1,
    /// 暂停接纳工作并进入可恢复的 scheduler/GC 停机边界。
    Pause = 2,
    /// 从可恢复停机边界恢复 scheduler、GC 和工作接纳。
    Resume = 3,
    /// 进入不可恢复的卸载静默边界。
    Quiesce = 4,
    /// 在卸载静默边界内终止 runtime 并释放其私有状态。
    Finalize = 5,
}

impl GoLifecyclePhase {
    /// 从固定 ABI 数值解析生命周期阶段。
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            1 => Some(Self::Initialize),
            2 => Some(Self::Pause),
            3 => Some(Self::Resume),
            4 => Some(Self::Quiesce),
            5 => Some(Self::Finalize),
            _ => None,
        }
    }

    /// 返回固定 ABI 数值。
    #[must_use]
    pub const fn raw(self) -> u32 {
        self as u32
    }
}

/// 传给 Go runtime 生命周期函数的只读上下文。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct GoLifecycleContextV1 {
    /// ABI 主版本。
    pub abi_version: u32,
    /// 当前结构的完整字节数。
    pub struct_size: u32,
    /// ELM cell ID；常驻集成模块可为零。
    pub cell_id: u64,
    /// ELM generation；常驻集成模块可为零。
    pub generation: u64,
    /// [`GoLifecyclePhase`] 的固定 ABI 数值。
    pub phase: u32,
    /// V1 必须为零。
    pub flags: u32,
    /// V1 必须全部为零，供后续停机边界元数据扩展。
    pub reserved: [u64; 4],
}

impl GoLifecycleContextV1 {
    /// V1 结构的精确尺寸。
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;

    /// 构造没有启用扩展的 V1 上下文。
    #[must_use]
    pub const fn new(cell_id: u64, generation: u64, phase: GoLifecyclePhase) -> Self {
        Self {
            abi_version: GO_CARRIER_ABI_VERSION,
            struct_size: Self::SIZE,
            cell_id,
            generation,
            phase: phase.raw(),
            flags: 0,
            reserved: [0; 4],
        }
    }

    /// 检查固定头、阶段和 V1 保留字段。
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.abi_version == GO_CARRIER_ABI_VERSION
            && self.struct_size == Self::SIZE
            && GoLifecyclePhase::from_raw(self.phase).is_some()
            && self.flags == 0
            && self.reserved[0] == 0
            && self.reserved[1] == 0
            && self.reserved[2] == 0
            && self.reserved[3] == 0
    }

    /// 检查上下文是否属于指定阶段。
    #[must_use]
    pub const fn is_valid_for(&self, expected: GoLifecyclePhase) -> bool {
        self.is_valid() && self.phase == expected.raw()
    }
}

/// 尚未解析的 carrier 入口地址。
///
/// 描述符来自外语 artifact，不能在检查前把任意位模式构造成 Rust 函数指针。loader 必须先验证
/// 地址位于该 artifact 的可执行段并完成 relocation，调用门才可把它转换为具体签名。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct GoEntryPointV1(u64);

impl GoEntryPointV1 {
    /// 构造尚未完成可执行段验证的地址值。
    #[must_use]
    pub const fn from_raw(address: u64) -> Self {
        Self(address)
    }

    /// 返回原始目标地址。
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// 检查入口是否存在；这不证明地址可以安全执行。
    #[must_use]
    pub const fn is_present(self) -> bool {
        self.0 != 0
    }
}

/// Go AOT artifact 导出的生命周期表。
///
/// `pause` 是可恢复停机边界，`resume` 只与它配对。`quiesce` 是卸载的终止性停机边界：成功
/// 后不得恢复，也不得再有 Go 代码、并发 GC 或回调执行；`finalize` 在该边界内完成终止。
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GoRuntimeV1 {
    /// ABI 主版本。
    pub abi_version: u32,
    /// 当前结构的完整字节数。
    pub struct_size: u32,
    /// 初始化 runtime。
    pub initialize: GoEntryPointV1,
    /// 进入可恢复的 scheduler/GC 停机边界。
    pub pause: GoEntryPointV1,
    /// 离开可恢复停机边界。
    pub resume: GoEntryPointV1,
    /// 进入不可恢复的卸载静默边界。
    pub quiesce: GoEntryPointV1,
    /// 在卸载静默边界内终止 runtime。
    pub finalize: GoEntryPointV1,
    /// V1 必须全部为零，供未来可选生命周期能力扩展。
    pub reserved: [u64; 4],
}

impl GoRuntimeV1 {
    /// 当前目标上 V1 结构的精确尺寸。
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;

    /// 检查 ABI 固定头和保留槽。
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.abi_version == GO_CARRIER_ABI_VERSION
            && self.struct_size == Self::SIZE
            && self.initialize.is_present()
            && self.pause.is_present()
            && self.resume.is_present()
            && self.quiesce.is_present()
            && self.finalize.is_present()
            && self.reserved.iter().all(|value| *value == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    fn runtime() -> GoRuntimeV1 {
        GoRuntimeV1 {
            abi_version: GO_CARRIER_ABI_VERSION,
            struct_size: GoRuntimeV1::SIZE,
            initialize: GoEntryPointV1::from_raw(0x1000),
            pause: GoEntryPointV1::from_raw(0x1010),
            resume: GoEntryPointV1::from_raw(0x1020),
            quiesce: GoEntryPointV1::from_raw(0x1030),
            finalize: GoEntryPointV1::from_raw(0x1040),
            reserved: [0; 4],
        }
    }

    #[test]
    fn context_layout_is_stable() {
        assert_eq!(size_of::<GoLifecycleContextV1>(), 64);
        assert_eq!(align_of::<GoLifecycleContextV1>(), 8);
        assert_eq!(offset_of!(GoLifecycleContextV1, abi_version), 0);
        assert_eq!(offset_of!(GoLifecycleContextV1, cell_id), 8);
        assert_eq!(offset_of!(GoLifecycleContextV1, generation), 16);
        assert_eq!(offset_of!(GoLifecycleContextV1, phase), 24);
        assert_eq!(offset_of!(GoLifecycleContextV1, reserved), 32);
    }

    #[test]
    fn context_rejects_unknown_phase_and_reserved_data() {
        let mut context = GoLifecycleContextV1::new(7, 3, GoLifecyclePhase::Quiesce);
        assert!(context.is_valid_for(GoLifecyclePhase::Quiesce));
        assert!(!context.is_valid_for(GoLifecyclePhase::Resume));

        context.phase = 99;
        assert!(!context.is_valid());
        context.phase = GoLifecyclePhase::Quiesce.raw();
        context.reserved[2] = 1;
        assert!(!context.is_valid());
    }

    #[test]
    fn runtime_table_layout_and_reserved_slots_are_stable() {
        assert_eq!(offset_of!(GoRuntimeV1, initialize), 8);
        assert_eq!(offset_of!(GoRuntimeV1, pause), 16);
        assert_eq!(offset_of!(GoRuntimeV1, resume), 24);
        assert_eq!(offset_of!(GoRuntimeV1, quiesce), 32);
        assert_eq!(offset_of!(GoRuntimeV1, finalize), 40);
        assert_eq!(offset_of!(GoRuntimeV1, reserved), 48);
        assert_eq!(GoRuntimeV1::SIZE, 80);

        let mut table = runtime();
        assert!(table.is_valid());
        table.reserved[0] = 1;
        assert!(!table.is_valid());
        table.reserved[0] = 0;
        table.pause = GoEntryPointV1::from_raw(0);
        assert!(!table.is_valid());
    }
}
