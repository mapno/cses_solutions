use std::io::{BufRead, Write};

const MODULO: u64 = 1000000007;

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: u32 = scanner.token();
    for _ in 0..n {
        let (a, b, c) = (scanner.token(), scanner.token(), scanner.token());
        write!(cout, "{}\n", mod_pow(a, mod_pow(b, c, MODULO-1), MODULO));
    }
}

fn mod_pow(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut r: u64 = 1;
    while b > 0 {
        if b % 2 == 1 {
            r = (r * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    r
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
