fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num: i32 = buffer.trim().parse().unwrap();
    for i in 1..(num * 2) {
        for _ in 1..(num - (num - i).abs()) {
            print!(" ");
        }
        for _ in 0..((num - i).abs() * 2 + 1) {
            print!("*");
        }
        println!();
    }
}
