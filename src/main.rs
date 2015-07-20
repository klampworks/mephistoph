mod circ_vec;
use circ_vec::CircVec;
use circ_vec::CircRead;

mod xor;
use xor::xor_file_to_file;

use std::env;
extern crate getopts;
use getopts::Options;

use std::fs::File;

fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} FILE [options]", program);
            print!("{}", opts.usage(&brief));
}

fn main() {

    let mut kk: Option<Box<CircRead>> = None;

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("k", "key", 
        "A string to be used as a key. If shorter than data will be cycled.", 
        "\"my secret key\"");
    opts.optopt("", "keyfile", 
        "A file to be used as a key. If the contents is shorter than the data it will be cycled.", 
        "\"~/my-key-file\"");

    let matches = match opts.parse(&args[1..]) {
            Ok(m) => {m}
            Err(f) => { panic!(f.to_string()) }
    };

    let key_s = match matches.opt_str("k") {
        Some(k) => {k}
        None => {format!("")} 
    };

    if !key_s.is_empty() {
        kk = Some(Box::new(CircVec::new(key_s.into_bytes())));
    }

    let key_f = match matches.opt_str("keyfile") {
        Some(k) => {k}
        None => {format!("")} 
    };

    if !key_f.is_empty() {
        let mut key_file = match File::open(key_f) {
            Ok(f) => {f}
            Err(f) => {panic!(f.to_string())}
        };
        kk = Some(Box::new(key_file));
    }

    if kk.is_some() {

        let mut fin = std::io::stdin();
        let mut fout = std::io::stdout();

        let k: &mut CircRead = &mut*kk.unwrap();
        xor_file_to_file(k, &mut fin, &mut fout);
        return;
    }

    print_usage(&program, opts);
    return;
}
