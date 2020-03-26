fn main() {
    let mut buf = String::with_capacity(1);
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<u64>().unwrap();

    let mut c: u64 = 0;
    let mut i: u64 = 5;
    while n / i >= 1 {
        c += n / i;
        i *= 5;
    }
    print!("{}", c);
}
