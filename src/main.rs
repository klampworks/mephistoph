use std::io::Read;

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
        .expect("a") != 0 {

        xor_buf(7, &mut buf);

        for x in buf.iter() {
            print!("{}", x)
        }

    }
    //println!("Hello, world!");
}
