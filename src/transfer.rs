fn as_func() {
    let b: i64 = 1i32 as i64;
    println!("b {}", b)
}

fn from_func() {
    println!("Hello, world!");
    let b: Complex = 1.into();
    println!("{:?}", b);
}

#[derive(Debug)]
struct Complex {
    re: i32,
    im: i32,
}

impl From<i32> for Complex {
    fn from(re: i32) -> Self {
        Complex {
            re,
            im: 0,
        }
    }
}