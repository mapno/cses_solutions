use std::io::{BufRead, Write};
use std::collections::{VecDeque};

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut cin = Scanner::new(stdin.lock());

    let r = cin.token();
    let c = cin.token();

    let mut graph: Vec<Vec<u32>> = vec![Vec::new(); r as usize];
    let mut path: VecDeque<u32> = VecDeque::with_capacity(r as usize);
    let mut visited: Vec<u8> = vec![0; r as usize];

    for _i in 0..c {
        let m: usize = cin.token();
        let n: u32 = cin.token();
        graph[m - 1].push(n - 1);
    }

    for i in 0..r {
        if visited[i as usize] == 0 {
            if !dfs(i, &graph, &mut visited, &mut path) {
                print!("IMPOSSIBLE ");
                return
            }
        }
    }

//    path.reverse();
    for n in path.iter() {
        write!(cout, "{} ", n + 1);
    }
}

fn dfs(u: u32, g: &Vec<Vec<u32>>, v: &mut Vec<u8>, p: &mut VecDeque<u32>) -> bool {
    v[u as usize] = 1;
    for &c in &g[u as usize] {
        if v[c as usize] == 1 || v[c as usize] == 0 && !dfs(c, g, v, p) {
            return false;
        }
    }
    v[u as usize] = 2;
    p.push_front(u);
    true
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
