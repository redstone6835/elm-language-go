//! 显式启用的 managed 控制面 ABI；不用于驱动热路径。

/// 固定内联载荷容量。
pub const GO_ELM_MANAGED_PAYLOAD_LEN: usize = 192;

/// managed ABI 主版本。
pub const GO_ELM_MANAGED_ABI_VERSION: u32 = 1;

/// 有界 managed 请求或回复 frame。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct GoElmManagedFrameV1 {
    /// ABI 主版本。
    pub abi_version: u32,
    /// 当前结构的完整字节数。
    pub struct_size: u32,
    /// 非零 operation 编号。
    pub operation: u32,
    /// V1 必须为零。
    pub flags: u32,
    /// ELM cell ID。
    pub cell_id: u64,
    /// ELM generation。
    pub generation: u64,
    /// 非零请求 ID。
    pub request_id: u64,
    /// `payload` 中的有效字节数。
    pub payload_len: u32,
    /// V1 必须为零。
    pub reserved0: u32,
    /// 固定内联载荷。
    pub payload: [u8; GO_ELM_MANAGED_PAYLOAD_LEN],
    /// V1 必须全部为零。
    pub reserved: [u64; 2],
}

impl GoElmManagedFrameV1 {
    /// V1 结构的精确尺寸。
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;

    /// 检查固定头、身份字段、载荷边界和保留字段。
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.abi_version == GO_ELM_MANAGED_ABI_VERSION
            && self.struct_size == Self::SIZE
            && self.operation != 0
            && self.flags == 0
            && self.cell_id != 0
            && self.generation != 0
            && self.request_id != 0
            && self.payload_len as usize <= GO_ELM_MANAGED_PAYLOAD_LEN
            && self.reserved0 == 0
            && self.reserved.iter().all(|value| *value == 0)
    }
}

/// managed 控制面传输表。
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GoElmManagedTransportV1 {
    /// ABI 主版本。
    pub abi_version: u32,
    /// 当前结构的完整字节数。
    pub struct_size: u32,
    /// 提交入口。
    pub submit: crate::GoEntryPointV1,
    /// 轮询入口。
    pub poll: crate::GoEntryPointV1,
    /// 取消入口。
    pub cancel: crate::GoEntryPointV1,
    /// V1 必须全部为零。
    pub reserved: [u64; 4],
}

impl GoElmManagedTransportV1 {
    /// 当前目标上 V1 结构的精确尺寸。
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;

    /// 检查固定头和保留槽。
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.abi_version == GO_ELM_MANAGED_ABI_VERSION
            && self.struct_size == Self::SIZE
            && self.submit.is_present()
            && self.poll.is_present()
            && self.cancel.is_present()
            && self.reserved.iter().all(|value| *value == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn frame_layout_is_stable() {
        assert_eq!(size_of::<GoElmManagedFrameV1>(), 256);
        assert_eq!(align_of::<GoElmManagedFrameV1>(), 8);
        assert_eq!(offset_of!(GoElmManagedFrameV1, cell_id), 16);
        assert_eq!(offset_of!(GoElmManagedFrameV1, payload), 48);
        assert_eq!(offset_of!(GoElmManagedFrameV1, reserved), 240);
    }

    #[test]
    fn oversized_or_reserved_frame_is_rejected() {
        let mut frame = GoElmManagedFrameV1 {
            abi_version: GO_ELM_MANAGED_ABI_VERSION,
            struct_size: GoElmManagedFrameV1::SIZE,
            operation: 1,
            flags: 0,
            cell_id: 7,
            generation: 2,
            request_id: 3,
            payload_len: GO_ELM_MANAGED_PAYLOAD_LEN as u32,
            reserved0: 0,
            payload: [0; GO_ELM_MANAGED_PAYLOAD_LEN],
            reserved: [0; 2],
        };
        assert!(frame.is_valid());

        frame.payload_len = (GO_ELM_MANAGED_PAYLOAD_LEN + 1) as u32;
        assert!(!frame.is_valid());
        frame.payload_len = 0;
        frame.reserved[1] = 1;
        assert!(!frame.is_valid());
    }

    #[test]
    fn transport_requires_unresolved_entries_and_zero_reserved_slots() {
        let mut transport = GoElmManagedTransportV1 {
            abi_version: GO_ELM_MANAGED_ABI_VERSION,
            struct_size: GoElmManagedTransportV1::SIZE,
            submit: crate::GoEntryPointV1::from_raw(0x1000),
            poll: crate::GoEntryPointV1::from_raw(0x1010),
            cancel: crate::GoEntryPointV1::from_raw(0x1020),
            reserved: [0; 4],
        };
        assert_eq!(GoElmManagedTransportV1::SIZE, 64);
        assert!(transport.is_valid());
        transport.cancel = crate::GoEntryPointV1::from_raw(0);
        assert!(!transport.is_valid());
    }
}
