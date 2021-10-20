extern crate maulingmonkey_format_exe as exe;

use exe::pe::{self, ImportDescriptor, RvaReader, RVA};
use exe::FromMemory;

use std::path::*;

fn main() {
    let mut args    = std::env::args_os();
    let self_path   = args.next().unwrap_or(Default::default());
    let exe_path    = args.next().unwrap_or(self_path);
    let exe_path    = PathBuf::from(exe_path);

    let exe = exe::pe::Reader::open(exe_path).unwrap();

    let ptr_size : u8 = match &exe.pe_header().optional_header {
        None => 0,
        Some(pe::OptionalHeader::OptionalHeader32(_)) => 4,
        Some(pe::OptionalHeader::OptionalHeader64(_)) => 8,
    };

    dbg!(exe.mz_header());
    dbg!(exe.pe_header());

    for (i, section) in exe.pe_section_headers().iter().enumerate() {
        eprintln!("sections[{}].name                = {:?}", i, section.name);
        eprintln!("sections[{}].characteristics     = {:?}", i, section.characteristics);
        eprintln!("sections[{}].virtual_address     = base + 0x{:08x} .. 0x{:08x}", i, section.virtual_address.to_u32(), section.virtual_address.to_u32() + section.virtual_size);
        match section.pointer_to_raw_data {
            None => eprintln!("sections[{}].data                = None", i),
            Some(_) => {
                let data = exe.read_pe_section_data(section).unwrap();
                //if section.name.to_bytes() == b".data" {
                if false {
                    eprintln!("sections[{}].data                = Some([", i);
                    let per_line = 32;
                    for line in data.windows(per_line) {
                        eprint!("   ");
                        for b in line {
                            //eprint!(" 0x{:02x},", b);
                            eprint!(" {:02x}", b);
                        }
                        for _ in line.len() .. per_line {
                            //eprint!("      ");
                            eprint!("   ");
                        }
                        eprintln!();
                    }
                    eprintln!("])");
                } else {
                    eprintln!("sections[{}].data                = Some([ ... {} byte(s) ... ])", i, data.len());
                }
            },
        }
        eprintln!();
    }

    let mut scratch = Vec::new();
    let dd = *exe.data_directory();
    for (i, (name, dd)) in dd.iter_name_dd().enumerate() {
        if *dd == pe::DataDirectory::default() { continue }

        eprintln!("data_directory.{: <16} = {:?}", name, dd);
        if let Some(section) = exe.pe_section_headers().iter().find(|s| s.virtual_address_range().contains(&dd.virtual_address)) {
            eprintln!("    section.name = {:?}", section.name);
        }
        match i {
            0 => {}, // export
            1 => { // import
                // https://stackoverflow.com/a/62850912
                let mut rva = RvaReader::new(&exe, dd.virtual_address);
                let n = dd.size / (std::mem::size_of::<ImportDescriptor>() as u32);
                for i in 0 .. n {
                    let import = ImportDescriptor::from_io(&mut rva).expect("unable to read import descriptor");

                    //eprintln!("    import[{}] = {:?}", i, import);

                    if import.dll_ascii_name_rva == RVA::NULL {
                        eprintln!("    import[{}] = END", i);
                    } else {
                        //eprintln!("    import[{}] = {{", i);
                        //eprintln!("        import_lookup_table_rva: {:?},", import.import_lookup_table_rva);
                        //eprintln!("        time_date_stamp:         {:?},", import.time_date_stamp);
                        //eprintln!("        forwarder_chain:         {:?},", import.forwarder_chain);
                        //eprintln!("        dll_ascii_name_rva:      {:?},", import.dll_ascii_name_rva);
                        //eprintln!("        dll_ascii_name:          {:?},", exe.read_asciiz_rva(import.dll_ascii_name_rva, &mut scratch).map(|s| String::from_utf8_lossy(s)));
                        //eprintln!("    }}");
                        //eprintln!();

                        eprintln!("    import[{}].dll_ascii_name = {:?},", i, exe.read_asciiz_rva(import.dll_ascii_name_rva, &mut scratch).map(|s| String::from_utf8_lossy(s)));
                    }
                }
            },
            2 => {}, // resource
            3 => {}, // exception
            4 => {}, // security
            5 => {}, // basereloc
            6 => {}, // debug
            7 => {}, // architecture
            8 => {}, // globalptr
            9 => {}, // tls
            10 => {}, // load_config
            11 => { // bound_import
                // "The bound directory consists of a chain of IMAGE_BOUND_IMPORT_DESCRIPTOR and IMAGE_BOUND_FORWARDER_REF entries."
                // https://stackoverflow.com/a/62850912
            },
            12 => { // iat
                // Just an array of function pointers, per https://stackoverflow.com/a/62850912
                let ptr_size = u32::from(ptr_size);
                let n = dd.size / ptr_size;
                for i in 0 .. n {
                    let rva = dd.virtual_address + i * ptr_size;
                    let buf = exe.read_exact_rva(rva .. rva + ptr_size, &mut scratch).unwrap();
                    eprint!("    function[{: >2}] = base? + 0x", i);
                    for b in buf.iter().rev() {
                        eprint!("{:02x}", b);
                    }
                    eprintln!();
                }
            },
            13 => {}, // delay_import
            14 => {}, // com_descriptor
            _ => {},
        }
        eprintln!();
    }
}
