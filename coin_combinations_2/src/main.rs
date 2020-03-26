use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let (m, n): (u8, usize) = (scanner.token(), scanner.token());

    let mut count = vec![0; n+1];
    let modulo = 1000000007;
    let mut c;
    count[0] = 1;
    for _ in 0..m {
        c = scanner.token();
        for j in c..(n + 1) {
            count[j] += count[j - c];
            if count[j] >= modulo {
                count[j] -= modulo;
            }
        }
    }

    write!(stdout, "{}", count[n]).unwrap();
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
