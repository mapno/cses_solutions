use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut cin = Scanner::new(stdin.lock());
    let n: u32 = cin.token();

    if n == 1 {
        write!(cout, "1");
    } else if n <= 3 {
        write!(cout, "NO SOLUTION");
    } else {
        let mut f: u32 = 1;
        let mut m: u32 = n/2+1;
        let mut i: u32 = 1;
        while i <= n/2 {
            write!(cout, "{} {} ", m, f);
            m += 1;
            f += 1;
            i += 1;
        }
        if n&1==1 {
            write!(cout, "{}", n);
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
