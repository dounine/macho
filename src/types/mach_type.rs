use binrw::binrw;

#[binrw]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachType {
    #[brw(magic(0xcafebabe_u32))]
    FatMaGic,
    #[brw(magic(0xbebafeca_u32))]
    FatCiGam,
    #[brw(magic(0xfeedface_u32))]
    MachoMagic,
    #[brw(magic(0xfeedfacf_u32))]
    MachoMagic64,
    #[brw(magic(0xcefaedfe_u32))]
    MachoCiGam,
    #[brw(magic(0xcffaedfe_u32))]
    MachoCiGam64,
    Unknown(u32),
}