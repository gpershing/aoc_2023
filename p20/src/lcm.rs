pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while a != b {
        if a == 0 || b == 0 {
            a = a.max(b);
            break;
        }
        let t = a.min(b);
        a = a.max(b) % t;
        b = t;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    let g = gcd(a, b);
    a * (b / g)
}