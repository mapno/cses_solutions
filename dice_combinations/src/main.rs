use std::io::{BufRead, Write};

static mut V: [u64; 1000006] = [0u64; 1000006];

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: usize = scanner.token();
    unsafe {
        V[0] = 0;
        V[1] = 1;
        V[2] = 2;
        V[3] = 4;
        V[4] = 8;
        V[5] = 16;
        V[6] = 32;
        for i in 7..(n + 1) {
            V[i] = (V[i - 1] + V[i - 2] + V[i - 3] + V[i - 4] + V[i - 5] + V[i - 6]) % 1000000007;
        }
        write!(stdout, "{}", V[n]).unwrap();
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
