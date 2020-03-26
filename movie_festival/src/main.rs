use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: usize = scanner.token();
    let mut m: Vec<(u32, u32)> = vec![(0, 0); n];
    for i in 0..n {
        m[i] = (scanner.token(), scanner.token());
    }
    m.sort_unstable_by_key(|a| a.1);

    let mut t: u32 = 1;
    let mut ne: u32 = m[0].1;
    for i in 1..n {
        if m[i].0 >= ne {
            ne = m[i].1;
            t += 1;
        }
    }
    write!(cout, "{}", t);
}

struct Scanner<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}

impl<B: BufRead> Scanner<B> {
    fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace(),
        }
    }
    fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.reader.read_to_end(&mut self.buf_str).unwrap();
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
