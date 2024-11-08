use binrw::binrw;

#[binrw]
#[derive(Debug, PartialEq, Eq)]
pub enum CpuType {
    #[brw(magic = 7_u32)]
    X86,
    #[brw(magic = 16777223_u32)]
    X86_64,
    #[brw(magic = 12_u32)]
    Arm,
    #[brw(magic = 16777228_u32)]
    Arm64,
    #[brw(magic = 33554444_u32)]
    Arm64_32,
    Unknown(i32),
}