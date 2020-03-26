use std::io::{BufRead, Write};
use std::time::Instant;


fn main() {
    let now = Instant::now();
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut cin = Scanner::new(stdin.lock());

    let n: usize = cin.token();
    let m: usize = cin.token();
    let k: i32 = cin.token();

    let mut d: Vec<i32> = vec![0; n];
    for i in 0..n {
        d[i] = cin.token();
    }
    d.sort();
    println!("Building stuff: {} ms", now.elapsed().as_secs_f32());

    let mut s: Vec<i32> = vec![0; m];
    for i in 0..m {
        s[i] = cin.token();
    }
    s.sort();
    println!("Building moar stuff: {} ms", now.elapsed().as_secs_f32());

    let mut a: i32 = 0;
    let mut b: usize = 0;
    let mut c: usize = 0;
    while b < n && c < m {
        if d[b]+k < s[c] {
            b += 1;
        } else if d[b]-k > s[c] {
            c += 1;
        } else {
            a += 1;
            b += 1;
            c += 1;
        }
    }

    write!(cout, "{} ", a);
    println!("Calculating: {} ms", now.elapsed().as_secs_f32());
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