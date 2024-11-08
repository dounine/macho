use std::ffi::{c_char, CStr};
use binrw::{BinResult, BinWrite};

/*
    use
    struct Example {
        #[br(map = |v:[u8;16]| map_to_cstring(&v))]
        name:String,
    }
 */
// fn map_to_cstring(data: &[u8]) -> String {
//     let c_char = data.as_ptr() as *const c_char;
//     let c_str = unsafe { CStr::from_ptr(c_char) };
//     c_str.to_string_lossy().to_string()
// }
/*
    use
    struct Example {
        #[br(map = |v:[u8;16]| map_cstring_to(&v))]
        name:String,
    }
 */
// fn map_cstring_to(str: &String) -> Vec<u8> {
//     let mut data = str.as_bytes().to_vec();
//     data.resize(16, 0u8);
//     data
// }

/*
    use
    struct Example {
        #[br(parse_with = bytes::to_cstring, args(16,))]
        name:String,
    }
 */
#[binrw::parser(reader)]
pub fn to_cstring(size: usize) -> BinResult<String> {
    let mut buffer = vec![0u8; size];
    reader.read_exact(&mut buffer)?;
    let c_char = buffer.as_ptr() as *const c_char;
    let c_str = unsafe { CStr::from_ptr(c_char) };
    Ok(c_str.to_string_lossy().to_string())
}
/*
    use
    struct Example {
        #[bw(write_with = bytes::cstring_to, args(16,))]
        name:String,
    }
 */
#[binrw::writer(writer)]
pub fn cstring_to(data: &String, size: usize) -> BinResult<()> {
    let mut data = data.as_bytes().to_vec();
    data.resize(size, 0u8);
    data.write(writer)?;
    Ok(())
}