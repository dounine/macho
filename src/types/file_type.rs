use binrw::binrw;

#[binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    #[brw(magic(0x1_u32))]
    MachObject,
    #[brw(magic(0x2_u32))]
    MachExecute,
    #[brw(magic(0x3_u32))]
    MachFVMLib,
    #[brw(magic(0x4_u32))]
    MachCore,
    #[brw(magic(0x5_u32))]
    MachPreload,
    #[brw(magic(0x6_u32))]
    MachDyLib,
    #[brw(magic(0x7_u32))]
    MachDyLinker,
    #[brw(magic(0x8_u32))]
    MachBundle,
    #[brw(magic(0x9_u32))]
    MachDyLibStub,
    #[brw(magic(0xa_u32))]
    MachDsym,
    #[brw(magic(0xb_u32))]
    MachKextBundle,
    Unknown(u32),
}