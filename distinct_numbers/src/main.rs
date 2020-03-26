use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();
    let mut cin = Scanner::new(stdin.lock());
    let s: u32 = cin.token();

    let mut v: Vec<u32> = vec![0; s as usize];
    for i in 0..s {
        v[i as usize] = cin.token();
    }

    v.sort();
    v.dedup();
    print!("{}", v.len());
}

pub struct Scanner<B> {
    reader: B,
    buf_str: String,
    buf_iter: std::str::SplitWhitespace<'static>,
}
impl<B: BufRead> Scanner<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: String::new(),
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
                .read_line(&mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe { std::mem::transmute(self.buf_str.split_whitespace()) };
        }
    }
}