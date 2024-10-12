fn add_space(mut s: String, n: u32) -> String {
    for _i in 1..=n {
        s.push(' ');
    }
    s
}
fn add_str(mut s: String, a: &str) -> String {
    s.push_str(a);
    s
}
fn add_integer(mut s: String, mut num: u32) -> String {
    if num == 0 {
        s.push('0');
        return s;
    }

    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % 10) as u8 + b'0');
        num /= 10;
    }

    let mut count = 0;
    for &digit in digits.iter().rev() {
        if count > 0 && count % 3 == 0 {
            s.push('_');
        }
        s.push(digit as char);
        count += 1;
    }

    s
}

fn add_float(mut s: String, num: f32) -> String {
    let integer_part = num.trunc() as u32;
    let mut decimal_part = num.fract();

    s = add_integer(s, integer_part);
    s.push('.');

    for _ in 0..3 {
        //doar 3 zecimale
        decimal_part *= 10.0;
        let digit = decimal_part.trunc() as u8;
        s.push((digit + b'0') as char);
        decimal_part -= digit as f32;
    }

    s
}
fn main() {
    let mut result = String::new();

    result = add_space(result, 40);
    result = add_str(result, "I 💚");
    result = add_space(result, 1);
    result.push('\n');

    result = add_space(result, 40);
    result = add_str(result, "RUST.");
    result = add_space(result, 1);
    result.push('\n');

    result = add_space(result, 1);
    result.push('\n');
    result = add_space(result, 4);
    result = add_str(result, "Most");
    result = add_space(result, 12);
    result = add_str(result, "crate");
    result = add_space(result, 6);
    result = add_integer(result, 306437968);
    result = add_space(result, 11);
    result = add_str(result, "and");
    result = add_space(result, 5);
    result = add_str(result, "lastest");
    result = add_space(result, 9);
    result = add_str(result, "is");
    result = add_space(result, 1);
    result.push('\n');

    result = add_space(result, 9);
    result = add_str(result, "downloaded");
    result = add_space(result, 8);
    result = add_str(result, "has");
    result = add_space(result, 13);
    result = add_str(result, "downloads");
    result = add_space(result, 5);
    result = add_str(result, "the");
    result = add_space(result, 9);
    result = add_str(result, "version");
    result = add_space(result, 9);
    result = add_float(result, 2.038);
    result = add_str(result, ".");

    println!("{}", result);
}
