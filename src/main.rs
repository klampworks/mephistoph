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

fn parse_key(matches: getopts::Matches) -> Option<Box<CircRead>> {

    let mut ret: Option<Box<CircRead>> = None;

    ret = match matches.opt_str("k") {
        Some(k) => {
            Some(Box::new(CircVec::new(k.into_bytes()))) }
        None => {None} 
    };

    if ret.is_some() {
        return ret;
    }

    ret = match matches.opt_str("keyfile") {
        Some(k) => {
            match File::open(k) {
                Ok(f) => {Some(Box::new(f))}
                Err(f) => {panic!(f.to_string())}
            }
        }
        None => {None} 
    };

    return ret;
}

fn main() {

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

    let mut kk = match parse_key(matches) {
        Some(key) => { key }
        None => {
            print_usage(&program, opts);
            return; 
        }
    };

    let mut fin = std::io::stdin();
    let mut fout = std::io::stdout();

    xor_file_to_file(&mut*kk, &mut fin, &mut fout);
    return;
}
