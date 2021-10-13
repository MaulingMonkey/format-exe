extern crate maulingmonkey_format_exe as exe;

use std::fs::File;
use std::io::*;
use std::path::*;
use std::process::exit;

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

    exe.seek(SeekFrom::Start(mz_header.pe_header_start.into())).expect("unable to seek through executable file");

    let pe_header = exe::pe::Header::read_from(&mut exe).unwrap_or_else(|err| {
        eprintln!("error: unable to read pe::Header from `{}`: {}", exe_path.display(), err);
        exit(1);
    });

    dbg!(pe_header);

    for isection in 0 .. pe_header.file_header.nsections {
        let section_header = exe::pe::SectionHeader::read_from(&mut exe).unwrap_or_else(|err| {
            eprintln!(
                "error: unable to read pe::SectionHeader[{} of {}] from `{}`: {}",
                isection, pe_header.file_header.nsections, exe_path.display(), err,
            );
            exit(1);
        });

        dbg!(section_header);
    }
}
