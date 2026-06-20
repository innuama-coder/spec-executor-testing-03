fn main() {
    let mut count = 0;
    for arg in std::env::args().skip(1) {
        for token in arg.split_whitespace() {
            println!("word: {}", token);
            count += 1;
        }
    }
    println!("Total: {}", count);
}
