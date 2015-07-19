mod circ_vec;
use circ_vec::CircVec;

mod xor;
use xor::xor_file_to_file;

use std::env;
extern crate getopts;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} FILE [options]", program);
            print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("k", "key", 
        "A string to be used as a key. If shorter than data will be cycled.", 
        "\"my secret key\"");

    let matches = match opts.parse(&args[1..]) {
            Ok(m) => {m}
            Err(f) => { panic!(f.to_string()) }
    };


    let key_s = match matches.opt_str("k") {
        Some(k) => {k}
        None => {format!("")} };

    if !key_s.is_empty() {

        let mut key = CircVec::new(key_s.into_bytes());
        let mut fin = std::io::stdin();
        let mut fout = std::io::stdout();

        xor_file_to_file(&mut key, &mut fin, &mut fout);
        return;
    }

    print_usage(&program, opts);
    return;
}
