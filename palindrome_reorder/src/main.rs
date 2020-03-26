use std::io::{Read, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());

    let mut buf = Vec::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_end(&mut buf).unwrap();
    let buf_iter = unsafe { std::str::from_utf8_unchecked(&buf) };

    let mut v: Vec<u64> = vec![0; 26];

    for c in buf_iter.bytes() {
        if c >= 65 {
            v[c as usize - 65] += 1;
        }
    }

    let mut odd_char: u8 = 0;
    let mut e: u8 = 0;
    for (c, n) in v.iter().enumerate() {
        if *n & 1 == 1 {
            e += 1;
        }
        if *n%2==1 {
            odd_char = c as u8 + 65;
        }
    }

    if e > 1 {
        write!(cout, "NO SOLUTION");
    } else {
        let mut i = 0;
        loop {
            for _ in 0..&v[i]/2 {
                write!(cout, "{}", (i as u8 + 65) as char);
            }
            if i == 25 {
                break
            }
            i += 1;
        }
        if odd_char > 0 {
            write!(cout, "{}", odd_char as char);
        }
        i = 25;
        loop {
            for _ in 0..&v[i]/2 {
                write!(cout, "{}", (i as u8 + 65) as char);
            }
            if i == 0 {
                break
            }
            i -= 1;
        }
    }
}