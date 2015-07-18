use std::io::Read;
use std::io::Write;
use std;

#[cfg(test)]
mod tests;

pub struct CircVec {
    data: Vec<u8>,
    wi: usize,
    ri: usize
}

impl CircVec {
    pub fn new(d: Vec<u8>) -> CircVec {
        CircVec{ data: d, wi: 0, ri: 0}
    }
}

pub trait CircRead {
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
