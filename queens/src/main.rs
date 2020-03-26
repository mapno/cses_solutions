use std::io::{Read, Write};

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());

    let mut buf = Vec::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_end(&mut buf).unwrap();
    let buf_iter = unsafe { std::str::from_utf8_unchecked(&buf) };

    let mut board: Vec<bool> = Vec::with_capacity(64);
    for c in buf_iter.bytes() {
        match c {
            42 => board.push(true),
            10 => continue,
            46 => board.push(false),
            _ => panic!("unknown"),
        }
    }

    let mut y: usize = 0;
    let mut count: u16 = 0;
    let mut col: [bool; 9] = [false; 9];
    let mut diag1: [bool; 15] = [false; 15];
    let mut diag2: [bool; 15] = [false; 15];
    search(
        &mut y, &mut count, &mut col, &mut diag1, &mut diag2, &mut board,
    );

    write!(cout, "{}", count).unwrap();
}

fn search(
    y: &mut usize,
    count: &mut u16,
    col: &mut [bool; 9],
    diag1: &mut [bool; 15],
    diag2: &mut [bool; 15],
    board: &mut Vec<bool>,
) {
    if *y == 8 {
        *count += 1;
        return;
    }
    for i in 0..8 {
        if col[i] || diag1[i + *y] || diag2[8 + i - *y - 1] || board[*y * 8 + i] {
            continue;
        }
        col[i] = true;
        diag1[i + *y] = true;
        diag2[8 + i - *y - 1] = true;
        search(&mut (*y + 1), count, col, diag1, diag2, board);
        col[i] = false;
        diag1[i + *y] = false;
        diag2[8 + i - *y - 1] = false;
    }
}
