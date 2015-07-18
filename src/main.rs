use std::io::Read;
use std::io::Write;
use std::vec::Vec;

fn xor(key: u8, data: u8) -> u8 {
    key ^ data
}

fn xor_buf(key: &[u8], buf: &mut [u8]) {
    assert_eq!(key.len(), buf.len());

    for i in 0..(key.len()) {
        buf[i] = xor(key[i], buf[i])
    }
}

fn xor_file_to_file<T: Read, K: CircRead, O: Write>
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

struct CircVec {
    data: Vec<u8>,
    wi: usize,
    ri: usize
}

impl CircVec {
    fn new(d: Vec<u8>) -> CircVec {
        CircVec{ data: d, wi: 0, ri: 0}
    }
}

trait CircRead {
    fn circread(&mut self , buf: &mut [u8]);
}

impl CircRead for CircVec {
    fn circread(&mut self, buf: &mut [u8]) {
        let mut out_i = 0;

        while out_i < buf.len() {
            buf[out_i] = self.data[self.ri];
            self.ri = (self.ri + 1) % self.data.len();
            out_i += 1;
        }
    }
}

impl Read for CircVec {

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {

        let mut out_i = 0;

        while self.ri < self.data.len() && out_i < buf.len() {
            buf[out_i] = self.data[self.ri];
            out_i += 1;
            self.ri += 1;
        }

        return Ok(out_i);
    }
}

impl Write for CircVec {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {

        let mut out_i = 0;
        while self.wi < self.data.len() && out_i < buf.len() {
            self.data[self.wi] = buf[out_i];
            out_i += 1;
            self.wi += 1;
        }

        return Ok(out_i);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // Err...
        // <(^.^)<
        return Ok(());
    }
}

fn main() {

    let key_s: String = format!("hello");
    let mut key = CircVec::new(key_s.into_bytes());
    let mut fin = std::io::stdin();
    let mut fout = std::io::stdout();
    xor_file_to_file(&mut key, &mut fin, &mut fout);
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
fn test_circvec_read() {

    let mut data: CircVec = CircVec::new(vec![66u8; 1]);
    let mut buf = [0u8; 10];
    let mut exp = [0u8; 10];
    exp[0] = 66u8;

    assert!(buf != exp);
    data.read(&mut buf);
    assert_eq!(buf, exp)
}

#[test]
fn test_circvec_circread() {

    let mut data: CircVec = CircVec::new(vec![66u8; 1]);
    let mut buf = [0u8; 10];
    let mut exp = [66u8; 10];

    assert!(buf != exp);
    data.circread(&mut buf);
    assert_eq!(buf, exp)
}

#[test]
fn test_circvec_write() {

    let bin = vec![1,2,3,4,5];
    let mut bout: CircVec = CircVec::new(vec![0u8; 5]);

    assert!(bin != bout.data);
    bout.write(&bin);
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
