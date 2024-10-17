use thiserror::Error;

#[derive(Debug)]
enum CustomError {
    Overflow,
}
fn is_prime(x: u32) -> bool {
    if x <= 1 {
        return false;
    }
    for i in 2..=((x as f64).sqrt() as u32) {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}
fn get_next_prime(x: u16) -> Result<u16, String> {
    let mut next_num = x as u32 + 1;
    let maxu16 = u16::MAX as u32;
    while next_num < maxu16 {
        if is_prime(next_num) {
            break;
        }
        next_num += 1;
    }
    if next_num < maxu16 {
        return Ok(next_num as u16);
    } else {
        return Err("No prime number found".to_string());
    }
}

fn next_prime(x: u16) -> Option<u16> {
    match get_next_prime(x) {
        Ok(prime) => Some(prime),
        Err(_) => None,
    }
}

fn problem1() {
    let mut start = 1;
    while let Some(i) = next_prime(start) {
        println!("Urmatorul numar prim este: {}", i);
        start = i;
    }
    println!(
        "Nu exista un numar prim mai mare decat {}, care poate fi reprezentat pe u16",
        start
    );
}

fn addition_with_option(x: u32, y: u32) -> Option<u32> {
    if (x as u64 + y as u64) > u32::MAX as u64 {
        return None;
    }
    Some(x + y)
}

fn multiplication_with_option(x: u32, y: u32) -> Option<u32> {
    if (x as u64 * y as u64) > u32::MAX as u64 {
        return None;
    }
    Some(x * y)
}

fn problem2() {
    let addition_overflow = addition_with_option(std::u32::MAX, 42);
    match addition_overflow {
        Some(addition_overflow) => println!("Rezultatul adunarii este: {}", addition_overflow),
        None => panic!("Eroare: Rezultatul adunarii nu incape pe u32"),
    }

    let addition = addition_with_option(165, 123);
    match addition {
        Some(addition) => println!("Rezultatul adunarii este: {}", addition),
        None => panic!("Eroare: Rezultatul adunarii nu incape pe u32"),
    }

    let multiplication_overflow = multiplication_with_option(u32::MAX, 2);
    match multiplication_overflow {
        Some(multiplication_overflow) => {
            println!("Rezultatul inmultirii este: {}", multiplication_overflow)
        }
        None => panic!("Eroare: Rezultatul inmultirii nu incape pe u32"),
    }
    let multiplication = multiplication_with_option(142, 77);
    match multiplication {
        Some(multiplication) => println!("Rezultatul inmultirii este: {}", multiplication),
        None => panic!("Eroare: Rezultatul inmultirii nu incape pe u32"),
    }
}

fn addition_with_result(x: u32, y: u32) -> Result<u32, CustomError> {
    if (x as u64 + y as u64) > u32::MAX as u64 {
        return Err(CustomError::Overflow);
    }
    Ok(x + y)
}

fn multiplication_with_result(x: u32, y: u32) -> Result<u32, CustomError> {
    if (x as u64 * y as u64) > u32::MAX as u64 {
        return Err(CustomError::Overflow);
    }
    Ok(x * y)
}

fn problem3() {
    let result_1 = multiplication_with_result(u32::MAX, 2);
    match result_1 {
        Ok(result_1) => println!("Multiplication success: {:?}", result_1),
        Err(e) => println!("Error at multiplicating: {:?}", e),
    }

    let result_2 = addition_with_result(u32::MAX, 1);
    match result_2 {
        Ok(result_2) => println!("Addition success: {:?}", result_2),
        Err(e) => println!("Error at adding: {:?}", e),
    }

    let result_3 = multiplication_with_result(234, 15);
    match result_3 {
        Ok(result_3) => println!("Multiplication success: {:?}", result_3),
        Err(e) => println!("Error at multiplicating: {:?}", e),
    }

    let result_4 = addition_with_result(153, 34562);
    match result_4 {
        Ok(result_4) => println!("Addition success: {:?}", result_4),
        Err(e) => println!("Error at adding: {:?}", e),
    }
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Character is not ASCII")]
    NotAscii,
    #[error("Character is not a digit")]
    NotDigit,
    #[error("Character is not a base16 digit")]
    NotBase16Digit,
    #[error("Character is not a letter")]
    NotLetter,
    #[error("Character is not printable")]
    NotPrintable,
}

fn to_uppercase(ch: char) -> Result<char, MyError> {
    if ch >= 'A' && ch <= 'Z' {
        return Ok(ch);
    }
    if ch >= 'a' && ch <= 'z' {
        return Ok(ch.to_ascii_uppercase());
    }
    Err(MyError::NotLetter)
}

fn to_lowercase(ch: char) -> Result<char, MyError> {
    if ch >= 'a' && ch <= 'z' {
        return Ok(ch);
    }
    if ch >= 'A' && ch <= 'Z' {
        return Ok(ch.to_ascii_lowercase());
    }
    Err(MyError::NotLetter)
}

fn print_char(ch: char) -> Result<(), MyError> {
    if ch.is_ascii_graphic() || ch.is_ascii_whitespace() {
        print!("{}", ch);
        Ok(())
    } else {
        Err(MyError::NotPrintable)
    }
}
fn char_to_number(ch: char) -> Result<u32, MyError> {
    if ch.is_ascii() && ch.is_ascii_digit() {
        Ok(ch.to_digit(10).unwrap())
    } else {
        if ch.is_ascii() && !ch.is_digit(10) {
            Err(MyError::NotDigit)
        } else {
            Err(MyError::NotAscii)
        }
    }
}

fn char_to_number_hex(ch: char) -> Result<u32, MyError> {
    if ch.is_ascii() && ch.is_ascii_hexdigit() {
        Ok(ch.to_digit(16).unwrap())
    } else {
        if ch.is_ascii() && !ch.is_digit(10) {
            Err(MyError::NotBase16Digit)
        } else {
            Err(MyError::NotAscii)
        }
    }
}

fn print_error(err: MyError) {
    eprintln!("Error: {}", err);
}
fn problem4() {
    println!("Problem 4 output:\n");

    match to_uppercase('a') {
        Ok(ch) => println!("Uppercase of 'a': {}", ch),
        Err(e) => print_error(e),
    }
    match to_uppercase('5') {
        Ok(ch) => println!("Uppercase of '5': {}", ch),
        Err(e) => print_error(e),
    }

    match to_lowercase('Z') {
        Ok(ch) => println!("Lowercase of 'Z': {}", ch),
        Err(e) => print_error(e),
    }
    match to_lowercase('!') {
        Ok(ch) => println!("Lowercase of '!': {}", ch),
        Err(e) => print_error(e),
    }

    match print_char('A') {
        Ok(()) => println!(" - Character 'A' printed successfully"),
        Err(e) => print_error(e),
    }
    match print_char('\u{0007}') {
        Ok(()) => println!(" - Bell character printed successfully"),
        Err(e) => print_error(e),
    }

    match char_to_number('7') {
        Ok(num) => println!("Numeric value of '7': {}", num),
        Err(e) => print_error(e),
    }
    match char_to_number('x') {
        Ok(num) => println!("Numeric value of 'x': {}", num),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('F') {
        Ok(num) => println!("Hex value of 'F': {}", num),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('G') {
        Ok(num) => println!("Hex value of 'G': {}", num),
        Err(e) => print_error(e),
    }
}

fn future_teller(choice: u32) -> Result<String, String> {
    let fortunes = [
        "You will find unexpected joy in the little things.",
        "A great opportunity will present itself to you soon.",
        "Your hard work will pay off in the near future.",
        "You will make a new friend who will change your life.",
        "An exciting journey awaits you.",
        "You will pass Rust Course with flying colors",
    ];

    if choice >= fortunes.len() as u32 {
        Err("Bad Luck: The fortune teller couldn't tell your future".to_string())
    } else {
        Ok(fortunes[choice as usize].to_string())
    }
}
use rand::Rng;
fn problem5() {
    println!("Welcome to the Future Teller!");
    println!("I will tell you something about your future.");
    println!("Your fate will now be chosen");

    let choice = rand::thread_rng().gen_range(0..=10);

    match future_teller(choice) {
        Ok(fortune) => println!("Your future: {}", fortune),
        Err(e) => println!("{}", e),
    }
}
fn main() {
    println!("Output-ul problemei 1: \n");
    problem1();

    //problema 2 include panic, si se opreste executia celorlalte exercitii :)
    //println!("Output-ul problemei 2: \n");
    //problem2();

    println!("\nOutput-ul problemei 3: \n");
    problem3();
    println!("Output-ul problemei 4: \n");
    problem4();
    println!("\nShowcasing Future Teller: \n");
    problem5();
}
