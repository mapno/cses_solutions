use std::io::{Read, Write};


fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());

    let mut buf = Vec::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_end(&mut buf).unwrap();
    let mut num = unsafe {
        std::str::from_utf8_unchecked(&buf)
            .trim()
            .parse::<u64>()
            .unwrap()
    };

    write!(cout, "{} ", num).unwrap();
    while num > 1 {
        if num & 1 != 0 {
            num *= 3;
            num += 1;
        } else {
            num /= 2;
        }
        write!(cout, "{} ", num).unwrap();
    }
}
