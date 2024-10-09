fn bottle_of_beers(n: u32) {
    for i in (2..=n).rev() {
        println!("{} bottles of beer on the wall,", i);
        println!("{} bottles of beer.", i);
        println!("Take one down and pass it around, ");
        println!("{} bottles of beer on the wall.", i - 1);
        println!("");
    }
    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer.");
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
}
fn main() {
    bottle_of_beers(99);
}
