use std::io::SeekFrom;
use binrw::binrw;
use crate::types::cpu_subtype::CpuSubtype;
use crate::types::cpu_type::CpuType;
use crate::types::mach_header::MachHeader;
use crate::types::mach_type::MachType;

#[binrw]
#[derive(Debug)]
pub struct FatHeader {
    magic: MachType, // mach magic 标识符，用来确定其属于 64位 还是 32位
    #[br(seek_before = SeekFrom::Current(-4),map = |magic:MachType| matches!(magic,MachType::FatCiGam|MachType::MachoCiGam|MachType::MachoCiGam64))]
    #[bw(ignore)]
    big_endian: bool,
    #[br(temp)]
    #[bw(calc = archs.len() as u32)]
    number_of_architecture: u32,
    #[br(count = number_of_architecture)]
    #[br(args{inner:(big_endian,)})]
    archs: Vec<FatArch>,
}
#[binrw]
#[brw(import(big_endian:bool))]
#[derive(Debug)]
pub struct FatArch {
    cpu_type: CpuType,
    cpu_subtype: CpuSubtype,
    offset: u32,
    size: u32,
    align: u32,
    #[br(align_before = offset)]
    #[bw(align_before = *offset)]
    #[br(is_big=big_endian)]
    mach: MachHeader,
}