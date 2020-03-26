use std::cmp::max;
use std::io::{BufRead, Write};


fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut cin = Scanner::new(stdin.lock());
    let n: u64
        = cin.token();

    for _ in 0..n {
        let y: u64 = cin.token();
        let x: u64 = cin.token();
        if x >= y {
            if x&1==0 {
                write!(cout, "{}\n", (x-1)*(x-1)+y);
            } else {
                write!(cout, "{}\n", x*x-y+1);
            }
        } else {
            if y&1==0 {
                write!(cout, "{}\n", y*y-x+1);
            } else {
                write!(cout, "{}\n", (y-1)*(y-1)+x);
            }
        }
    }
}

pub struct Scanner<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}

impl<B: BufRead> Scanner<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace(),
        }
    }
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
