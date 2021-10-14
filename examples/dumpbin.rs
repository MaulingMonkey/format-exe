extern crate maulingmonkey_format_exe as exe;

use std::fs::File;
use std::io::*;
use std::path::*;
use std::process::exit;

use exe::pe::DataDirectory;

fn main() {
    let mut args    = std::env::args_os();
    let self_path   = args.next().unwrap_or(Default::default());
    let exe_path    = args.next().unwrap_or(self_path);
    let exe_path    = PathBuf::from(exe_path);

    if !exe_path.exists() {
        eprintln!("error: `{}` does not exist", exe_path.display());
        exit(1);
    }

    let mut exe = BufReader::new(File::open(&exe_path).unwrap_or_else(|err| {
        eprintln!("error: unable to open `{}`: {}", exe_path.display(), err);
        exit(1);
    }));

    let mz_header = exe::mz::Header::read_from(&mut exe).unwrap_or_else(|err| {
        eprintln!("error: unable to read mz::Header from `{}`: {}", exe_path.display(), err);
        exit(1);
    });

    dbg!(mz_header);

    exe.seek(SeekFrom::Start(mz_header.pe_header_start.into())).unwrap_or_else(|err| {
        eprintln!("error: unable to seek to pe::Header in `{}`: {}", exe_path.display(), err);
        exit(1);
    });

    let pe_header = exe::pe::Header::read_from(&mut exe).unwrap_or_else(|err| {
        eprintln!("error: unable to read pe::Header from `{}`: {}", exe_path.display(), err);
        exit(1);
    });

    dbg!(pe_header);

    let mut sections = [exe::pe::SectionHeader::default(); 32];
    let sections = &mut sections[..32usize.min(pe_header.file_header.nsections.into())];

    for (i, section) in sections.iter_mut().enumerate() {
        *section = exe::pe::SectionHeader::read_from(&mut exe).unwrap_or_else(|err| {
            eprintln!(
                "error: unable to read pe::SectionHeader {} of {} from `{}`: {}",
                i+1, pe_header.file_header.nsections, exe_path.display(), err,
            );
            exit(1);
        });
        dbg!(*section);
    }

    for (i, section) in sections.iter().enumerate() {
        eprintln!();
        eprintln!("sections[{}].name                = {:?}", i, section.name);
        eprintln!("sections[{}].characteristics     = {:?}", i, section.characteristics);
        eprintln!("sections[{}].virtual_address     = 0x{:08x} .. 0x{:08x}", i, section.virtual_address, section.virtual_address + section.virtual_size);
        match section.pointer_to_raw_data {
            None => eprintln!("sections[{}].data                = None", i),
            Some(offset) => {
                exe.seek(SeekFrom::Start(offset.get().into())).unwrap_or_else(|err| {
                    eprintln!(
                        "error: unable to unable to seek to PE section {} of {} in `{}`: {}",
                        i+1, pe_header.file_header.nsections, exe_path.display(), err,
                    );
                    exit(1);
                });
                let mut data = vec![0u8; section.size_of_raw_data as usize];
                exe.read_exact(&mut data).unwrap_or_else(|err| {
                    eprintln!(
                        "error: unable to read PE section {} of {} in `{}`: {}",
                        i+1, pe_header.file_header.nsections, exe_path.display(), err,
                    );
                    exit(1);
                });
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
    }

    if let Some(optional_header) = pe_header.optional_header.as_ref() {
        let data_directory = optional_header.data_directory();
        eprintln!();
        for (name, dd) in data_directory.iter_name_dd() {
            if *dd == DataDirectory::default() { continue }

            eprintln!("data_directory.{: <16} = {:?}", name, dd);
            if let Some(section) = sections.iter().find(|s| s.virtual_address_range().contains(&dd.virtual_address)) {
                eprintln!("    section.name = {:?}", section.name);
                eprintln!();
            }
        }
    }
}
