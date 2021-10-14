extern crate maulingmonkey_format_exe as exe;

use exe::pe;

use std::path::*;

fn main() {
    let mut args    = std::env::args_os();
    let self_path   = args.next().unwrap_or(Default::default());
    let exe_path    = args.next().unwrap_or(self_path);
    let exe_path    = PathBuf::from(exe_path);

    let mut exe = exe::pe::Reader::open(exe_path).unwrap();

    dbg!(exe.mz_header());
    dbg!(exe.pe_header());
    let sections = exe.read_pe_section_headers().unwrap();

    for (i, section) in sections.iter().enumerate() {
        eprintln!("sections[{}].name                = {:?}", i, section.name);
        eprintln!("sections[{}].characteristics     = {:?}", i, section.characteristics);
        eprintln!("sections[{}].virtual_address     = 0x{:08x} .. 0x{:08x}", i, section.virtual_address, section.virtual_address + section.virtual_size);
        match section.pointer_to_raw_data {
            None => eprintln!("sections[{}].data                = None", i),
            Some(_) => {
                let data = exe.read_pe_section_data(i).unwrap();
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

    for (name, dd) in exe.data_directory().iter_name_dd() {
        if *dd == pe::DataDirectory::default() { continue }

        eprintln!("data_directory.{: <16} = {:?}", name, dd);
        if let Some(section) = sections.iter().find(|s| s.virtual_address_range().contains(&dd.virtual_address)) {
            eprintln!("    section.name = {:?}", section.name);
            eprintln!();
        }
    }
}
