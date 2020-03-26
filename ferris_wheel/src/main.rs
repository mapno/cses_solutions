use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut cin = Scanner::new(stdin.lock());

    let n: usize = cin.token();
    let x: u32 = cin.token();
    let mut c: Vec<u32> = vec![0; n];
    for i in 0..n {
        c[i] = cin.token();
    }
    c.sort();

    let mut g: u32 = 0;
    let mut i: usize = 0;
    let mut j: usize = n - 1;
    while i <= j {
        if c[i] + c[j] <= x {
            i += 1;
        }
        g += 1;
        if j == 0 {
            break
        }
        j -= 1;
    }

    write!(cout, "{}", g);
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
            self.buf_str.clear();
            self.reader
                .read_to_end(&mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
