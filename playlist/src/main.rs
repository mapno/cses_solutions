use std::io::{BufRead, Write};
use std::collections::{BTreeSet, VecDeque};

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n = scanner.token::<usize>();

    let mut set = BTreeSet::new();
    let mut queue = VecDeque::new();

    let mut m = 0;
    for _ in 0..n {
        let x = scanner.token();
        queue.push_back(x);
        if !set.insert(x) {
            let mut cs = 0;
            loop {
                cs = queue.pop_front().unwrap();
                if cs == x {
                    break
                }
                set.remove(&cs);
            }
        }
        m = m.max(queue.len())
    }
    write!(stdout, "{}", m);
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
