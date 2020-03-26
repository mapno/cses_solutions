use std::cmp::max;
use std::io::{BufRead, Write};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: u32 = scanner.token();
    let mut total: u64 = 0;
    let mut m: u32 = 0;
    for _ in 0..n {
        let v = scanner.token();
        m = max(m, v);
        total += v as u64;
    }
    write!(stdout, "{}", max(total, 2 * m as u64));

    write!(stdout, "\nelapsed {}ns", now.elapsed().as_micros());

    write!(stdout, "\n{}", scanner.buf_str.capacity());

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
            buf_str: Vec::with_capacity(838856),
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
