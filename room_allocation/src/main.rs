use std::cmp::Ordering::{Less, Greater};
use std::io::{BufRead};
use std::cmp::Reverse;

fn main() {
    let stdout = std::io::stdout();
    let _stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: usize = scanner.token();
    let mut rooms: Vec<(u32, u32, usize)> = Vec::with_capacity(n);
    for i in 0..n {
        rooms.push((scanner.token(), scanner.token(), i));
    }
    rooms.sort_by_key(|k| Reverse(k.0));
    println!("{:?}", rooms);
    let mut order: Vec<usize> = vec![0; n];
    while rooms.len() > 0 {
        let r = rooms.pop().unwrap().clone();
        loop {
            let up = upper_bound(&rooms, &r.1);
            for i in up..n {
                println!("{:?}", rooms[i])
            }
            break
        }
    }
//    write!(stdout, "{}\n", rooms.len());
}

fn upper_bound(s: &Vec<(u32, u32, usize)>, x: &u32) -> usize {
    let mut size = s.len();
    if size == 0 {
        return 0;
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let cmp = (unsafe { s.get_unchecked(mid) }).0.cmp(x);
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    let cmp = (unsafe { s.get_unchecked(base) }).0.cmp(x);
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
