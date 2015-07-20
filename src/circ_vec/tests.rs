//use circ_vec::super::*;
use ::xor::*;
use super::*;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::env::vars;

#[test]
fn test_xor_val() {
    assert_eq!(1, xor(6, 7))
}

#[test]
fn test_xor_id() {
    assert_eq!(7, xor(6, xor(6, 7)))
}

#[test]
fn test_circvec_read() {

    let mut data: CircVec = CircVec::new(vec![66u8; 1]);
    let mut buf = [0u8; 10];
    let mut exp = [0u8; 10];
    exp[0] = 66u8;

    assert!(buf != exp);
    assert_eq!(data.read(&mut buf).ok().unwrap(), data.data.len());
    assert_eq!(buf, exp)
}

#[test]
fn test_circvec_read_small_dst() {

    let mut data: CircVec = CircVec::new(vec![1,2,3]);
    let mut buf = [0u8; 2];
    let exp = [1,2];

    assert!(buf != exp);
    assert_eq!(data.read(&mut buf).ok().unwrap(), buf.len());
    assert_eq!(buf, exp)
}

#[test]
fn test_circvec_circread() {

    let mut data: CircVec = CircVec::new(vec![66u8; 1]);
    let mut buf = [0u8; 10];
    let exp = [66u8; 10];

    assert!(buf != exp);
    data.circread(&mut buf);
    assert_eq!(buf, exp)
}

#[test]
fn test_circvec_write() {

    let bin = vec![1,2,3,4,5];
    let mut bout: CircVec = CircVec::new(vec![0u8; 5]);

    assert!(bin != bout.data);

    assert_eq!(bin.len(), bout.data.len());
    assert_eq!(bout.write(&bin).ok().unwrap(), bin.len());

    assert_eq!(bin, bout.data)
}

#[test]
fn test_xor_file_to_file() {
    let mut key: CircVec = CircVec::new(vec![66u8; 1]);
    let mut data: CircVec = CircVec::new(vec![1u8, 2u8, 3u8, 4u8, 5u8]);
    let data_exp = vec![67u8, 64u8, 65u8, 70u8, 71u8];
    let mut out: CircVec = CircVec::new(vec![0u8; 5]);

    xor_file_to_file(&mut key, &mut data, &mut out);

    assert_eq!(out.data, data_exp)
}

#[test]
fn test_file_circread() {
    let pwd = vars().find(|ref x| x.0 == "PWD");
    let mut f = match File::open("1octet.bin") {
        Ok(f) => {f}
        Err(f) => {panic!(f.to_string())}
    };
    let mut buf = [0u8; 10];
    let exp = [97u8; 10];

    assert!(buf != exp);
    f.circread(&mut buf);
    assert_eq!(buf, exp);
}
