mod circ_vec;
use circ_vec::CircVec;

mod xor;
use xor::xor_file_to_file;

fn main() {

    let key_s: String = format!("hello");
    let mut key = CircVec::new(key_s.into_bytes());
    let mut fin = std::io::stdin();
    let mut fout = std::io::stdout();
    xor_file_to_file(&mut key, &mut fin, &mut fout);
}
