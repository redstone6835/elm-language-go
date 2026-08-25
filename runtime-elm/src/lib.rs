#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

//! `go.support` 的 Rust glue 骨架。
//!
//! 真实 ELM 入口和目标 EKI direct wrapper 后续由 `cargo elm` 生成。本 crate 只暴露并验证
//! Go runtime 生命周期描述符，不实现设备 API dispatcher。

pub use elm_language_go_carrier::*;

/// `go.support` 的稳定 ELM 名称。
pub const GO_SUPPORT_ELM_NAME: &str = "go.support";

/// 返回本 glue 所要求的 carrier ABI 主版本。
#[must_use]
pub const fn carrier_abi_version() -> u32 {
    GO_CARRIER_ABI_VERSION
}

/// 检查 Go AOT artifact 提供的生命周期表是否与本 glue 兼容。
#[must_use]
pub fn validate_runtime(runtime: &GoRuntimeV1) -> bool {
    runtime.is_valid()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_current_runtime_descriptor() {
        let runtime = GoRuntimeV1 {
            abi_version: GO_CARRIER_ABI_VERSION,
            struct_size: GoRuntimeV1::SIZE,
            initialize: GoEntryPointV1::from_raw(0x1000),
            pause: GoEntryPointV1::from_raw(0x1010),
            resume: GoEntryPointV1::from_raw(0x1020),
            quiesce: GoEntryPointV1::from_raw(0x1030),
            finalize: GoEntryPointV1::from_raw(0x1040),
            reserved: [0; 4],
        };

        assert_eq!(GO_SUPPORT_ELM_NAME, "go.support");
        assert_eq!(carrier_abi_version(), 1);
        assert!(validate_runtime(&runtime));
    }
}
