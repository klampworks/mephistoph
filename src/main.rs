use std::io::Read;
use std::io::Write;

fn xor(key: u8, data: u8) -> u8 {
    key ^ data
}

fn xor_buf(key: &[u8], mut key_i: usize, buf: &mut [u8]) -> usize{

    let len = buf.len();

    for x in buf.iter_mut() {
        *x = xor(key[key_i], *x);
        key_i = ((key_i + 1) % len) - 1;
    }

    return key_i;
}

fn main() {
    let mut buf = [0u8; 1];
    let key : [u8; 4] = [1, 2, 3, 4];
    let mut key_i = 0;

    while std::io::stdin().read(&mut buf)
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
    //println!("Hello, world!");
}
