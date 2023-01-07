
fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}

fn gcd_recursive(a: usize, b: usize) -> usize {
    if b > 0 {
        gcd_recursive(b, a % b)
    } else {
        a
    }
}


fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 { return v; }
    if v == 0 { return u; }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        #[allow(clippy::manual_swap)]
        if u > v {
            // mem::swap(&mut u, &mut v);
            let temp = u;
            u = v;
            v = temp;
        }

        v -= u; // here v >= u

        if v == 0 { break; }
    }

    u << shift
}

fn main() {
    println!("{}", gcd(65, 45));
    println!("{}", gcd_recursive(65, 45));

    println!("{}", lcm(65, 45));
    println!("{}", lcm(4, 6));
}
