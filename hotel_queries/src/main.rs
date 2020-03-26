use std::io::{BufRead, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let (n, m): (usize, usize) = (scanner.token(), scanner.token());
    let mut k: usize = 1;
    while k < n as usize {
        k *= 2;
    }

    let mut a: Vec<u32> = vec![0 as u32; 2*k];
    for i in 0..n {
        a[k+i] = scanner.token();
    }

    let mut i = k-1;
    while i > 0 {
        a[i] = a[2*i].max(a[2*i+1]);
        i -= 1;
    }

    for _ in 0..m {
        let h = scanner.token();
        let mut r: usize = 0;
        let mut j: usize = 1;

        while j < k {
            j *= 2;
            if a[j] < h {
                j += 1;
            }
        }
        if j < (k + n as usize) && a[j] >= h {
            r = j - k + 1;
            a[j] -= h;
            j /= 2;
            while j > 0 {
                a[j] = a[2*j].max(a[2*j+1]);
                j /= 2;
            }
        }
        write!(stdout, "{} ", r);
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
