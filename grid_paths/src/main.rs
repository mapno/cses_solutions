use std::io::{BufRead, Write};

// ASCII values
const QM: u8 = 63;
const R: u8 = 82;
const U: u8 = 85;
const L: u8 = 76;
const D: u8 = 68;
//
const DIR: [u8; 4] = [U, L, D, R];
const DX: [i8; 4] = [0, -1, 0, 1];
const DY: [i8; 4] = [-1, 0, 1, 0];

static mut PATHS: u32 = 0;

static mut VISITED: [bool; 49] = [false; 49];
static mut RES: [u8; 48] = [QM; 48];

fn main() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());

    let mut dir: Vec<u8> = Vec::with_capacity(49);
    let stdin = std::io::stdin();
    stdin.lock().read_until(b'\n', &mut dir).unwrap();


    unsafe {
        RES.copy_from_slice(&dir[0..48]);
        VISITED[0] = true;
        dfs(0, 0, 0, QM);
        write!(cout, "{}", PATHS);
    }
}

unsafe fn dfs(x: usize, y: usize, n: u32 , mov: u8) {
    if n == 48 {
        PATHS += 1;
        return;
    }
    if VISITED[42] {
        return;
    }
    if mov == D && (y == 6 || VISITED[(y+1)*7+x]) && x > 0 && x < 6 && !VISITED[y*7+x-1] && !VISITED[y*7+x+1] { return; }
    if mov == U && (y == 0 || VISITED[(y-1)*7+x]) && x > 0 && x < 6 && !VISITED[y*7+x-1] && !VISITED[y*7+x+1] { return; }
    if mov == R && (x == 6 || VISITED[y*7+x+1]) && y > 0 && y < 6 && !VISITED[(y-1)*7+x] && !VISITED[(y+1)*7+x] { return; }
    if mov == L && (x == 0 || VISITED[y*7+x-1]) && y > 0 && y < 6 && !VISITED[(y-1)*7+x] && !VISITED[(y+1)*7+x] { return; }
    match RES[n as usize] {
        QM => {
            for k in 0..4 {
                visit(x, y, k, n);
            }
        },
        D => {
            visit(x, y, 2, n);
        },
        U => {
            visit(x, y, 0, n);
        },
        R => {
            visit(x, y, 3, n);
        },
        L => {
            visit(x, y, 1, n);
        },
        _ => panic!()
    }
}

unsafe fn visit(x: usize, y: usize, k: usize, n: u32) {
    let i = x as i8 + DX[k];
    let j = y as i8 + DY[k];
    if inside_grid(j, i) && !VISITED[(j*7+i) as usize] {
        VISITED[(j*7+i) as usize] = true;
        dfs(i as usize, j as usize, n+1, DIR[k]);
        VISITED[(j*7+i) as usize] = false;
    }
}

fn inside_grid(y: i8, x: i8) -> bool {
    return y >= 0 && y <= 6 && x >= 0 && x <= 6;
}
