use std::collections::{BTreeSet, BTreeMap};
use std::io::{BufRead, Write};
use std::cmp::Ordering;
use std::cmp::Ordering::Less;

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let (x, n): (u32, u32) = (scanner.token(), scanner.token());
    let mut pos: BTreeMap<u32, u32> = BTreeMap::new();
    let mut len: BTreeSet<(u32, u32)> = BTreeSet::new();
    pos.insert((x, x));
    len.insert(x, x);



    for _ in 0..n {
        let l = scanner.token();
        pos.insert(l);
        let a = pos.range(..l).next_back().unwrap();
        pos.remove(a);

        len.remove(&c);
        len.insert(l - *a);
        len.insert(*b - l);
        write!(stdout, "{} ", len.iter().last().unwrap_or(&0).clone());
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
