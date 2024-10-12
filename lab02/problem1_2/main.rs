fn add_chars_n(mut s: String, c: char, n: u32) -> String {
    for _i in 1..=n {
        s.push(c);
    }

    s
}
fn add_chars_n_self_return(s: &mut String, c: char, n: u32) {
    for _i in 1..=n {
        s.push(c);
    }
}
fn main() {
    let mut s = String::from("");
    let mut b = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);
        add_chars_n_self_return(&mut b, c, 26 - i);

        i += 1;
    }

    println!("{}", s);
    print!("{}", s);
}
