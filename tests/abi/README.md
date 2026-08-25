# ABI 测试

ABI 测试固定 carrier 结构的 size、alignment、offset、endianness、reserved 字段、调用签名与
状态码。当前 Rust 与 Go 单测分别断言同一组 V1 数值；正式生成器接入时还需生成并由两端读取
同一个机器可读 golden vector。生成的 EKI binding 必须同时校验 Profile digest 和 carrier ABI
版本。
