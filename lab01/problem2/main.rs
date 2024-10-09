fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn are_coprime(a: u32, b: u32) -> bool {
    let gcd = gcd(a, b);
    gcd == 1
}

fn main() {
    for i in 1..=100 {
        for j in (i + 1)..=100 {
            if are_coprime(i, j) {
                println!("{} si {} sunt coprime", i, j);
            }
        }
    }
}
