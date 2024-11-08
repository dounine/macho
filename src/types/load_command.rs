use binrw::{binrw, BinRead, BinWrite};
use crate::convert::bytes;

#[binrw]
#[brw(is_big=big_endian,import(is_u64:bool,big_endian:bool))]
#[derive(Debug)]
pub struct Section<T: for<'a> BinRead<Args<'a>=()> + for<'a> BinWrite<Args<'a>=()>> {
    #[br(parse_with = bytes::to_cstring, args(16,))]
    #[bw(write_with = bytes::cstring_to, args(16,))]
    section_name: String,
    #[br(parse_with = bytes::to_cstring, args(16,))]
    #[bw(write_with = bytes::cstring_to, args(16,))]
    segment_name: String,
    addr: T,
    size: T,
    offset: u32,
    align: u32,
    rel_offset: u32,
    nreloc: u32,
    flags: u32,
    reserved1: u32,
    reserved2: u32,
    #[br(if(is_u64))]
    reserved3: Option<u32>,
}
#[binrw]
#[brw(is_big=big_endian,import(big_endian:bool))]
#[derive(Debug)]
pub struct BuildToolVersion {
    tool: u32,
    version: u32,
}
#[binrw]
#[brw(is_big=big_endian,import(big_endian:bool))]
#[derive(Debug)]
pub enum LoadCommand {
    #[brw(magic = 0x00000001_u32)]
    Segment {
        cmd_size: u32,
        #[br(parse_with = bytes::to_cstring, args(16,))]
        #[bw(write_with = bytes::cstring_to, args(16,))]
        segment_name: String,
        vm_addr: u32,
        vm_size: u32,
        file_offset: u32,
        file_size: u32,
        max_prot: i32,
        init_prot: i32,
        #[br(temp)]
        #[bw(calc = sections.len() as u32)]
        nsects: u32,
        flags: u32,
        #[br(count = nsects)]
        #[br(args{inner:(false,big_endian,)})]
        sections: Vec<Section<u32>>,
    },
    #[brw(magic = 0x00000019_u32)]
    Segment64 {
        cmd_size: u32,
        #[br(parse_with = bytes::to_cstring, args(16,))]
        #[bw(write_with = bytes::cstring_to, args(16,))]
        segment_name: String,
        vm_addr: u64,
        vm_size: u64,
        file_offset: u64,
        file_size: u64,
        max_prot: i32,
        init_prot: i32,
        #[br(temp)]
        #[bw(calc = sections.len() as u32)]
        nsects: u32,
        flags: u32,
        #[br(count = nsects)]
        #[br(args{inner:(true,big_endian,)})]
        sections: Vec<Section<u64>>,
    },
    #[brw(magic = 0x00000021_u32)]
    EncryptionInfo {
        cmd_size: u32,
        file_offset: u32,
        file_size: u32,
    },
    #[brw(magic = 0x0000002c_u32)]
    EncryptionInfo64 {
        cmd_size: u32,
        file_offset: u32,
        file_size: u32,
        crypt_id: u32,
        padding: u32,
    },
    #[brw(magic = 0x80000022_u32)]
    DyldInfoOnly {
        cmd_size: u32,
        rebase_info_offset: u32,
        rebase_info_size: u32,
        bind_info_offset: u32,
        bind_info_size: u32,
        weak_binding_info_offset: u32,
        weak_binding_info_size: u32,
        lazy_binding_info_offset: u32,
        lazy_binding_info_size: u32,
        export_info_offset: u32,
        export_info_size: u32,
    },
    #[brw(magic = 0x00000026_u32)]
    FunctionStarts {
        cmd_size: u32,
        file_offset: u32,
        file_size: u32,
    },
    #[brw(magic = 0x00000029_u32)]
    DataInCode {
        cmd_size: u32,
        file_offset: u32,
        file_size: u32,
    },
    #[brw(magic = 0x00000002_u32)]
    SymTab {
        cmd_size: u32,
        symbol_table_offset: u32,
        number_of_symbols: u32,
        string_table_offset: u32,
        string_table_size: u32,
    },
    #[brw(magic = 0x8000001c_u32)]
    RPath {
        cmd_size: u32,
        str_offset: u32,
        #[br(parse_with = bytes::to_cstring, args((cmd_size-str_offset) as usize,))]
        #[bw(write_with = bytes::cstring_to,args((cmd_size-str_offset) as usize,))]
        path: String,
    },
    #[brw(magic = 0x0000000d_u32)]
    IdDylib {
        cmd_size: u32,
        str_offset: u32,
        time_stamp: u32,
        current_version: u32,
        compatibility_version: u32,
        #[br(parse_with = bytes::to_cstring, args((cmd_size-str_offset) as usize,))]
        #[bw(write_with = bytes::cstring_to, args((cmd_size-str_offset) as usize,))]
        path: String,
    },
    #[brw(magic = 0x0000000c_u32)]
    LoadDylib {
        cmd_size: u32,
        str_offset: u32,
        time_stamp: u32,
        current_version: u32,
        compatibility_version: u32,
        #[br(parse_with = bytes::to_cstring, args((cmd_size-str_offset) as usize,))]
        #[bw(write_with = bytes::cstring_to, args((cmd_size-str_offset) as usize,))]
        name: String,
    },
    #[brw(magic = 0x00000025_u32)]
    VersionMinIphoneos {
        cmd_size: u32,
        version: u32,
        reserved: u32,
    },
    #[brw(magic = 0x00000032_u32)]
    BundleVersion {
        cmd_size: u32,
        platform: u32,
        minimum_os_version: u32,
        bundle_sdk_version: u32,
        #[br(temp)]
        #[bw(calc = build_tool_versions.len() as u32)]
        number_of_tools_entries: u32,
        #[br(count = number_of_tools_entries, args{inner:(big_endian,)})]
        build_tool_versions: Vec<BuildToolVersion>,
    },
    #[brw(magic = 0x0000000e_u32)]
    LoadDyLinker {
        cmd_size: u32,
        str_offset: u32,
        #[br(parse_with = bytes::to_cstring, args((cmd_size-str_offset) as usize,))]
        #[bw(write_with = bytes::cstring_to, args((cmd_size-str_offset) as usize,))]
        name: String,
    },
    #[brw(magic = 0x0000001b_u32)]
    Uuid {
        cmd_size: u32,
        #[br(map = |v:[u8;16]| uuid::Uuid::from_bytes(v))]
        #[bw(map = |v:&uuid::Uuid| v.as_bytes().to_vec())]
        uuid: uuid::Uuid,
    },
    #[brw(magic = 0x0000002a_u32)]
    SourceVersion { cmd_size: u32, version: u64 },
    #[brw(magic = 0x0000000b_u32)]
    DySymTab {
        cmd_size: u32,
        loc_symbol_index: u32,
        loc_symbol_number: u32,
        defined_ext_symbol_index: u32,
        defined_ext_symbol_number: u32,
        undef_ext_symbol_index: u32,
        undef_ext_symbol_number: u32,
        toc_offset: u32,
        toc_entries: u32,
        module_table_offset: u32,
        module_table_entries: u32,
        ext_ref_table_offset: u32,
        ext_ref_table_entries: u32,
        ind_sym_table_offset: u32,
        ind_sym_table_entries: u32,
        ext_reloc_table_offset: u32,
        ext_reloc_table_entries: u32,
        loc_reloc_table_offset: u32,
        loc_reloc_table_entries: u32,
    },
    #[brw(magic = 0x80000028_u32)]
    Main {
        cmd_size: u32,
        entry_offset: u64,
        stacks_size: u64,
    },
    #[brw(magic = 0x0000001d_u32)]
    CodeSignature {
        cmd_size: u32,
        file_offset: u32,
        file_size: u32,
    },
    // Unknown {
    //     //todo 应该匹配所有Command，不应该进来这里
    //     cmd: CmdType,
    //     #[bw(calc = (data.len() + 8) as u32)]
    //     cmd_size: u32,
    //     #[br(count = cmd_size - 8)]
    //     data: Vec<u8>,
    // },
}
