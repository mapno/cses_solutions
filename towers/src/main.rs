use std::cmp::Ordering::Less;
use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let mut set = Vec::<u32>::new();
    for _ in 0..scanner.token() {
        let x = scanner.token();
        let up = lower_bound(&set, &(x + 1));
        if up < set.len() && set[up] != x {
            set[up] = x;
        } else {
            set.push(x);
        }
    }
    write!(stdout, "{}", set.len());
}

fn lower_bound(s: &Vec<u32>, x: &u32) -> usize {
    let mut size = s.len();
    if size == 0 {
        return 0;
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let cmp = (unsafe { s.get_unchecked(base) }).cmp(x);
        base = if cmp == Less { mid } else { base };
        size -= half;
    }
    let cmp = (unsafe { s.get_unchecked(base) }).cmp(x);
    base + (cmp == Less) as usize
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
                .read_to_end(&mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
