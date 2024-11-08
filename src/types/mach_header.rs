use std::io::SeekFrom;
use binrw::binrw;
use crate::types::cpu_subtype::CpuSubtype;
use crate::types::cpu_type::CpuType;
use crate::types::file_type::FileType;
use crate::types::load_command::LoadCommand;
use crate::types::mach_type::MachType;

#[binrw]
#[brw(is_big=big_endian,import(big_endian:bool))]
#[derive(Debug)]
pub struct MachHeader {
    magic: MachType, // mach magic 标识符，用来确定其属于 64位 还是 32位
    #[br(seek_before = SeekFrom::Current(-4), map = |magic:MachType| matches!(magic,MachType::MachoMagic64|MachType::MachoCiGam64)
    )]
    #[bw(ignore)]
    bit_64: bool, //是否是64位的二进制文件
    // #[brw( args { big_endian: big_endian.clone() } )]
    cpu_type: CpuType,       //CPU 类型标识符，同通用二进制格式中的定义
    cpu_subtype: CpuSubtype, //CPU 子类型标识符，同通用二级制格式中的定义
    file_type: FileType,     //文件类型，主要用来区分可执行文件、动态库、静态库等
    count_of_cmds: u32,      //加载器中加载命令(Load commands)的数量
    size_of_cmds: u32,       //加载器中加载命令的总字节大小
    flags: u32,              // 标志位，主要用来表示是否是64位的二进制文件，是否是可执行文件等
    #[brw(if(bit_64.clone()))]
    reserved: Option<u32>, // 64 位的保留字段

    #[br(count = count_of_cmds)]
    #[br(args{inner:(big_endian,)})]
    commands: Vec<LoadCommand>,
}