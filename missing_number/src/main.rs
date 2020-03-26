mod input {
    extern {
        fn getchar() -> i32;
    }

    pub trait FromI32 {
        fn from(n: i32) -> Self;
    }

    macro_rules! impl_FromI32 {
        ($($ty:ty)*) => {
            $(
                impl FromI32 for $ty {
                    fn from(n: i32) -> $ty {
                        n as $ty
                    }
                }
            )*
        };
    }

    impl_FromI32!{ usize u32 }

    #[allow(dead_code)]
    pub fn get<T: FromI32>() -> T {
        let mut n = 0;
        let mut c = unsafe { getchar() };
        while c >= b'0' as i32 && c <= b'9' as i32 {
            let d = c - b'0' as i32;
            n = n * 10 + d;
            c = unsafe { getchar() };
        }
        T::from(n)
    }
}

fn main() {
    let n: usize = input::get();

    let mut v: Vec<u32> = vec![0;n];
    for i in 0..n {
        v[i] = input::get();
    }
    v.sort();

    let mut p = v[0];
    for n in v.into_iter().skip(1) {
        if n > p + 1 {
            print!("{}", p + 1);
            break
        }
        p = n;
    }
    if p == n as u32 - 1 {
        print!("{}", n);
    }
}
