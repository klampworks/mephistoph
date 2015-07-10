use std::io::Read;

fn xor(key: u8, data: u8) -> u8 {
    key ^ data
}

fn main() {
    let mut buf = [0u8, 1];

    while std::io::stdin().read(&mut buf)
        .ok()
        .expect("a") != 0 {
        print!("{}", xor(7, buf[0]))

    }
    println!("Hello, world!");
}
