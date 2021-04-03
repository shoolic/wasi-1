use std::env;
use std::path::PathBuf;

fn main() {
    let witx_paths = env::args_os().skip(1).map(|arg| PathBuf::from(arg)).collect::<Vec<_>>();

    print!("{}", witx_bindgen::generate(&witx_paths));
}
