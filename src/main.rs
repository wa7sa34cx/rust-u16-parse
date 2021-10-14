fn main() {
    let items: [&str; 5] = ["-10", "Hello, world!", "42", "65536", "65535"];

    for item in items {
        match item.parse::<u16>() {
            Ok(num) => println!("{}", num),
            Err(e) => println!("{}", e.to_string()),
        }
    }
}
