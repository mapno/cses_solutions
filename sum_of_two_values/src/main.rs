use std::io::{BufRead, Write};
use std::process::exit;

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: usize = scanner.token();
    let o: u32 = scanner.token();
    let mut m: Vec<(u32,usize)> = vec![(0, 0); 200_000];
    for i in 0..n {
        m[i] = (scanner.token(), i+1);
    }
    m[0..n].sort_unstable_by_key(|a| a.0);

    let (mut l, mut r) = (0, n - 1);
    while l < r {
        let (lv, rv) = (m[l].0, m[r].0);
        if lv + rv == o {
            write!(cout, "{} {}",  m[l].1,  m[r].1);
            cout.flush();
            exit(0);
        } else {
            if lv + rv < o {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    write!(cout, "IMPOSSIBLE");
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
            self.reader.read_to_end(&mut self.buf_str).unwrap();
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
