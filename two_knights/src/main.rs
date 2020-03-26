fn main() {
    let mut buf = String::with_capacity(1);
    std::io::stdin().read_line(&mut buf);
    let n = buf.trim().parse::<i64>().unwrap() + 1;

    let mut a = 0;
    let mut b = 0;
    for i in 1..n {
        a = i * i * (i * i - 1) / 2;
        b = 4 * (i - 2) * (i - 1);
        print!("{} ", a - b);
    }
}
