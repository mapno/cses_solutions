use std::cmp::{Ordering, Ordering::Greater};

use std::io::{BufRead, Write};

static mut V: [usize; 200_001] = [0; 200_001];

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let (n, m) = (scanner.token::<usize>() + 1, scanner.token::<usize>());
    let mut set: Vec<usize> = Vec::with_capacity(n);
    set.push(0);
    for _ in 1..n {
        set.push(scanner.token());
    }
    set.sort();

    for _ in 0..m {
        let up = upper_bound(&set, &scanner.token()) - set[0] as usize - 1;
        let mut p = up;
        while unsafe { V[p] } != 0 {
            p -= unsafe { V[p] };
        }
        if p > 0 {
            write!(stdout, "{}\n", set[p]);
            unsafe {
                V[up] = up - p;
                V[p] = 1;
            }
        } else {
            write!(stdout, "-1\n");
        }
    }
}

// Shamelessly stolen from https://crates.io/crates/ordslice
// All credits to the original author.
fn upper_bound(s: &Vec<usize>, x: &usize) -> usize {
    upper_bound_by(s, |y| y.cmp(x))
}

fn upper_bound_by<'a, F>(s: &'a Vec<usize>, mut f: F) -> usize
where
    F: FnMut(&'a usize) -> Ordering,
{
    let mut size = s.len();
    if size == 0 {
        return 0;
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let cmp = f(unsafe { s.get_unchecked(mid) });
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    let cmp = f(unsafe { s.get_unchecked(base) });
    base + (cmp != Greater) as usize
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
