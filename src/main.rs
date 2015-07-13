use std::io::Read;
use std::io::Write;

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

fn xor_from_stdin(key : &[u8]) {
    let mut buf = [0u8; 1];
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
}

fn main() {
    let key_s: String = format!("hello");
    let key = key_s.into_bytes();
    xor_from_stdin(&key);
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
