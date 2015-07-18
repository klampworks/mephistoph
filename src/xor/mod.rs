use std::io::Read;
use std::io::Write;

use super::circ_vec::CircRead;

pub fn xor(key: u8, data: u8) -> u8 {
    key ^ data
}

fn xor_buf(key: &[u8], buf: &mut [u8]) {
    assert_eq!(key.len(), buf.len());

    for i in 0..(key.len()) {
        buf[i] = xor(key[i], buf[i])
    }
}

pub fn xor_file_to_file<T: Read, K: CircRead, O: Write>
    (key: &mut K, mut f: T, out: &mut O) {

    let mut kbuf = [0u8; 5];
    let mut dbuf = [0u8; 5];

    while f.read(&mut dbuf)
        .ok()
        .expect("Could not read from stdin.") != 0 {

        key.circread(&mut kbuf);
        xor_buf(&kbuf, &mut dbuf);

        out.write(&dbuf)
            .ok()
            .expect("Could not write to stdout.");

        out.flush()
            .ok()
            .expect("Could not flush stdout.");
    }
}

