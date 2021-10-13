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
}
