use std::io::Read;
use std::io::Write;

fn xor(key: u8, data: u8) -> u8 {
    key ^ data
}

fn xor_buf(key: u8, buf: &mut [u8]) {
    for x in buf.iter_mut() {
        *x = xor(key, *x)
    }
}

fn main() {
    let mut buf = [0u8, 10];

    while std::io::stdin().read(&mut buf)
        .ok()
        .expect("Could not read from stdin.") != 0 {

        xor_buf(7, &mut buf);

        std::io::stdout().write(&buf)
            .ok()
            .expect("Could not write to stdout.");

        std::io::stdout().flush()
            .ok()
            .expect("Could not flush stdout.");
    }
    //println!("Hello, world!");
}
