use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut cin = Scanner::new(stdin.lock());

    let n: usize = cin.token();
    let mut v: Vec<i64> = vec![0; n];
    let mut sum: i64 = 0;
    for i in 0..n {
        v[i] = cin.token();
        sum += v[i];
    }
    let mut min_diff = sum;
    let m = 1 << n;

    for i in 0..m {
        let mut p_sum: i64 = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                p_sum += v[j];
            }
        }
        let diff = (sum - 2 * p_sum).abs();
        if diff < min_diff {
            min_diff = diff;
        }
    }
    write!(cout, "{}", min_diff.abs());
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
