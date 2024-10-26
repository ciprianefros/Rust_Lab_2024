use std::fs;

fn read_lines(file: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(file).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}
fn p1() {
    let lines = read_lines("./../resources/p1.txt");
    let mut max_line_bytes_size: usize = 0;
    let mut max_line_char_size: usize = 0;
    for line in lines {
        let char_count = line.chars().count();
        let byte_count = line.len();

        max_line_bytes_size = max_line_bytes_size.max(byte_count);
        max_line_char_size = max_line_char_size.max(char_count);
    }
    println!("Max line size in bytes: {}", max_line_bytes_size);
    println!("Max line size in chars: {}", max_line_char_size);
}

fn rotate_char(ch: char) -> Result<char, &'static str> {
    if !ch.is_ascii() {
        return Err("Error: Non-ASCII charachter found");
    }
    if ch.is_ascii_alphabetic() {
        let base = if ch.is_ascii_lowercase() { b'a' } else { b'A' };
        Ok(((ch as u8 - base + 13) % 26 + base) as char)
    } else {
        Ok(ch)
    }
}
fn rot13(input: &str) -> Result<String, &'static str> {
    input.chars().map(rotate_char).collect()
}

use std::io::Write;

fn p2() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_lines("./../resources/p2.txt");
    let mut output = String::new();

    for line in lines {
        let rotated = rot13(&line)?;
        output.push_str(&rotated);
        output.push('\n');
    }

    println!("{output}");
    let mut file = fs::File::create("./../resources/p2_output.txt")?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn replace_abbreviations(phrase: &str) -> String {
    let mut result = String::new();

    let mut it = phrase.split_ascii_whitespace();
    while let Some(v) = it.next() {
        if v.eq("pt") || v.eq("ptr") {
            result.push_str("pentru");
        } else if v.eq("dl") {
            result.push_str("domnul");
        } else if v.eq("dna") {
            result.push_str("doamna");
        } else {
            result.push_str(v);
        }
        result.push(' ');
    }

    result.trim_end().to_string()
}
fn p3() {
    let input = fs::read_to_string("./../resources/p3.txt").expect("Failed to read file");
    let output = replace_abbreviations(&input);
    println!("{}", output);
}

fn p4() {
    let host_file_lines = read_lines("./../resources/p4.txt");
    for line in host_file_lines {
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() && !trimmed_line.starts_with("#") {
            let mut columns = trimmed_line.split_ascii_whitespace();

            if let Some(host) = columns.next() {
                if let Some(id_address) = columns.next() {
                    println!("{} => {}", host, id_address);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nAcesta este output-ul problemei 1:");
    p1();

    println!("\nAcesta este output-ul problemei 2:");
    p2()?;

    println!("\nAcesta este output-ul problemei 3:");
    p3();

    println!("\nAcesta este output-ul problemei 4:");
    p4();

    Ok(())
}
