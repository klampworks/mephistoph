use std::io::Read;
use std::io::Write;
use std::io::Error;
use std::fs::File;
use std::io::BufRead;
use std::vec::Vec;

//extern crate libc;
//use libc::funcs::c95::stdio::*;

fn xor(key: u8, data: u8) -> u8 {
    key ^ data
}

fn xor_buf(key: &[u8], mut key_i: usize, buf: &mut [u8]) -> usize{

    let len = key.len();

    for x in buf.iter_mut() {
        *x = xor(key[key_i], *x);
        key_i = (key_i + 1) % len;
    }

    return key_i;
}

fn xor_from_stdin<T: Read>(key : &[u8], mut f: T) {
    let mut buf = [0u8; 1];
    let mut key_i = 0;

    while f.read(&mut buf)
        .ok()
        .expect("Could not read from stdin.") != 0 {

        key_i = xor_buf(&key, key_i, &mut buf);

        std::io::stdout().write(&buf)
            .ok()
            .expect("Could not write to stdout.");

        std::io::stdout().flush()
            .ok()
            .expect("Could not flush stdout.");
    }
}

struct MyVec {
    data: Vec<u8>,
    i: usize
}

impl MyVec {
    fn new(d: Vec<u8>) -> MyVec {
        MyVec{ data: d, i: 0}
    }
}

impl Read for MyVec {

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {

        let mut out_i = 0;
        let mut in_i = 0;

        while out_i < buf.len() {
            buf[out_i] = 1;
            out_i += 1;
        }

        return Ok(out_i);
    }
}

fn main() {
    let key_s: String = format!("hello");
    let key = key_s.into_bytes();

    //let f = std::io::stdin();
    let f = File::open("Cargo.toml").ok().expect("");

//    let data = Array{ data:[66; 1024]};
      let data: MyVec = MyVec::new(vec![66u8; 10]);
    let br = std::io::BufReader::new(data);
    xor_from_stdin(&key, br);
}

#[test]
fn test_xor_val() {
    assert_eq!(1, xor(6, 7))
}

#[test]
fn test_xor_id() {
    assert_eq!(7, xor(6, xor(6, 7)))
}

#[test]
fn test_xor_buf_1k_1b() {
    let mut buf = [7u8];

    // New index for a length 1 key should be 0.
    assert_eq!(0, xor_buf(&[6], 0, &mut buf));
    assert_eq!(1, buf[0])
}

#[test]
fn test_xor_buf_5k_1b() {
    let mut buf = [7u8];
    let mut ki = 0;
    let key = [1,2,3,4,5];
    let b_exp = [6, 4, 7, 3, 6];
    let k_exp = [1,2,3,4,0];

    for i in (0..b_exp.len()) {
        ki = xor_buf(&key, ki, &mut buf);
        assert_eq!(k_exp[i], ki);
        assert_eq!(b_exp[i], buf[0])
    }
}

#[test]
fn test_xor_buf_1k_5b() {
    let mut buf = [1,2,3,4,5];
    let mut ki = 0;
    let key = [6];
    let b_exp = [7, 4, 5, 2, 3];
    let k_exp = 0;

    ki = xor_buf(&key, ki, &mut buf);
    assert_eq!(k_exp, ki);

    for i in (0..b_exp.len()) {
        assert_eq!(b_exp[i], buf[i])
    }
}

#[test]
fn test_xor_buf_5k_5b() {
    let mut buf = [1,2,3,4,5];
    let mut ki = 0;
    let key = [5,4,3,2,1];
    let b_exp = [[4,2,0,4,4],[1,6,3,6,5],[4,2,0,4,4],[1,6,3,6,5],[4,2,0,4,4]];
    let k_exp = [[0; 5]; 5];

    for i in (0..b_exp.len()) {
        for j in (0..b_exp[i].len()) {

            ki = xor_buf(&key, ki, &mut buf);
            assert_eq!(k_exp[i][j], ki);
            assert_eq!(b_exp[i][j], buf[j])
        }
    }
}
