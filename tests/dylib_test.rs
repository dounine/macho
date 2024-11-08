#[cfg(test)]
mod dylib_test{
    use std::fs;
    use std::io::Cursor;
    use binrw::{BinReaderExt, BinWrite, Endian};
    use macho::types::fat::FatHeader;
    use macho::types::mach_type::MachType;

    #[test]
    fn parser_dylib() {
        let data = fs::read("./tests/data/macho.dylib").unwrap();
        println!("before length:{}", data.len());
        let mut reader = Cursor::new(&data);
        let magic: MachType = reader.read_le().unwrap();
        let endian = if magic == MachType::FatCiGam || magic == MachType::MachoCiGam {
            Endian::Big
        } else {
            Endian::Little
        };
        reader.set_position(0);
        let fat_header: FatHeader = reader.read_type(endian).unwrap();
        println!("magic : {:?}", fat_header);

        let mut writer = Cursor::new(vec![]);
        if endian == Endian::Big {
            fat_header.write_be(&mut writer).unwrap();
        } else {
            fat_header.write_le(&mut writer).unwrap();
        }
        // let mut file = File::create("./tests/data/.dylib").unwrap();
        // writer.set_position(0);
        // std::io::copy(&mut writer, &mut file).unwrap();
        // println!("after length: {:#?}", writer.get_ref().len());
    }
}