use std::io::{BufRead, Write};

const NUM: usize = 1000001;

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    let stdin = std::io::stdin();
    let mut stdin = Scanner::new(stdin.lock());

    let mut sieve: [usize; NUM] = [0; NUM];
    let mut primes: Vec<usize> = Vec::with_capacity(14);
    sieve[1] = 1;
    for i in 2..NUM {
        if sieve[i] == 0 {
            sieve[i] = i;
            primes.push(i);
        }
        let mut j: usize = 0;
        let l: usize = primes.len();
        while j < l {
            let m = i * primes[j];
            if m >= NUM {
                break;
            }
            sieve[m] = primes[j];
            j += 1;
        }
    }

    let n: usize = stdin.token();

    for _ in 0..n {
        let mut n: usize = stdin.token();
        let mut div: u32 = 1;
        while n > 1 {
            let prime = sieve[n as usize];
            let mut count: u32 = 0;
            while n % prime == 0 {
                count += 1;
                n /= prime;
            }
            div *= count + 1;
        }
        write!(stdout, "{} ", div);
    }
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
            self.reader
                .read_to_end(&mut self.buf_str)
                .unwrap();
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
