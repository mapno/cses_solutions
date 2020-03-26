use std::collections::VecDeque;
use std::io::{BufRead, Write};


fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut cin = Scanner::new(stdin.lock());

    let dx: [i64; 4]= [0, 1, 0, -1];
    let dy: [i64; 4] = [1, 0, -1, 0];

    let n0 = cin.token();
    let n1 = cin.token();

    let mut a: Vec<Vec<bool>> = vec![vec![false; n1 as usize]; n0 as usize];
    let mut rm: i64 = 0;
    let mut bfs: VecDeque<(i64, i64)> = VecDeque::with_capacity(1024);

    for i in 0..n0 {
        let line: String = cin.token();
        for (j, c) in line.chars().enumerate() {
            a[i as usize][j as usize] = c == '.';
        }
    }

    for i in 0..n0 {
        for j in 0..n1 {
            if a[i as usize][j as usize] {
                rm += 1;
                bfs.push_back((i, j));
                a[i as usize][j as usize] = false;
                while !bfs.is_empty() {
                    let (x, y) = bfs.pop_front().unwrap();
                    for d in 0..4 {
                        if x + dx[d as usize] >= 0 && x + dx[d as usize] < n0 && y + dy[d as usize] >= 0 && y + dy[d as usize] < n1 && a[(x + dx[d as usize]) as usize][(y + dy[d as usize]) as usize] {
                            bfs.push_back((x + dx[d as usize], y + dy[d as usize]));
                            a[(x + dx[d as usize]) as usize][(y + dy[d as usize]) as usize] = false;
                        }
                    }
                }
            }
        }
    }
    write!(cout, "{} ", rm);
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
