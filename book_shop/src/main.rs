use std::cmp::max;
use std::io::{BufRead, Write};

static mut DP: [u32; 100001] = [0; 100001];
static mut WT: [u32; 10001] = [0; 10001];
static mut VAL: [u32; 10001] = [0; 10001];

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());
    unsafe {
        let (n, w): (usize, usize) = (scanner.token(), scanner.token());
        for i in 0..n {
            WT[i] = scanner.token();
        }
        for i in 0..n {
            VAL[i] = scanner.token();
        }
        for i in 0..n {
            let mut j = w;
            let mut x = WT[i] as usize;
            while j >= x {
                DP[j] = max(DP[j], VAL[i] + DP[j - WT[i] as usize]);
                j -= 1;
            }
        }
        write!(stdout, "{}", DP[w]).unwrap();
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
