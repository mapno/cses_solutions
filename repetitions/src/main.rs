use std::cmp::max;

fn main() {
    let mut buf = String::with_capacity(1);
    std::io::stdin().read_line(&mut buf);
    let s: Vec<char> = buf.trim().chars().collect();

    let mut c: char = s[0];
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    for i in 1..s.len() {
        if s[i] == c {
            a += 1;
        } else {
            b = max(a, b);
            c = s[i];
            a = 1;
        }
    }
    print!("{}", max(a, b));
}
