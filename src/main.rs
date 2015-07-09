use std::io::Read;

fn main() {
    let mut buf = [0u8, 1];

    while std::io::stdin().read(&mut buf)
        .ok()
        .expect("a") != 0 {

        print!("{}", buf[0])

    }
    println!("Hello, world!");
}
