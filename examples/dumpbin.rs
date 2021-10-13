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
}
