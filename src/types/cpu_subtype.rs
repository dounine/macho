use binrw::binrw;

#[binrw]
#[derive(Debug, PartialEq, Eq)]
pub enum CpuSubtype {
    #[brw(magic(6_u32))]
    ArmV6,
    #[brw(magic(9_u32))]
    ArmV7,
    #[brw(magic(11_u32))]
    ArmV7S,
    #[brw(magic(12_u32))]
    ArmV7K,
    #[brw(magic(13_u32))]
    ArmV8,
    #[brw(magic(0_u32))]
    Arm64All,
    #[brw(magic(1_u32))]
    Arm64V8,
    #[brw(magic(2_u32))]
    Arm64E,
    Unknown(u32),
}