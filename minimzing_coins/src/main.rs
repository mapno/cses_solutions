use std::io::{BufRead, Write};
use std::collections::HashSet;

static MAX: u32 = u32::max_value();

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let (n, x): (usize, u32) = (scanner.token(), scanner.token());
    let mut coins: HashSet<u32> = HashSet::with_capacity(n);
    for _ in 0..n {
        coins.insert(scanner.token());
    }
    let mut v = vec![MAX; x as usize+1];
    v[0] = 0;
    for c in coins.into_iter() {
        for i in c..(x+1) {
            v[i as usize] = v[i as usize].min(v[i as usize-c as usize].checked_add(1).unwrap_or(MAX));

        }
    }
    let n = v[x as usize];
    if n == MAX {
        write!(stdout, "-1").unwrap();
    } else {
        write!(stdout, "{}", n).unwrap();
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
