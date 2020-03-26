use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: usize = scanner.token();
    let mut count: Vec<Vec<u32>> = vec![vec![0; n]; n];
    for i in 0..n {
        let l: String = scanner.token();
        for (j, char) in l.chars().enumerate() {
            if char == '.' {
                if i == 0 && j == 0 { count[0][0] = 1; }
                if i > 0 {
                    count[i][j] = (count[i][j] + count[i - 1][j]) % 1000000007;
                }
                if j > 0 {
                    count[i][j] = (count[i][j] + count[i][j - 1]) % 1000000007;
                }
            }
        }
    }
    write!(stdout, "{}", count[n - 1][n - 1]);
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
