fn main() {
    let results = include_str!("../input")
            .split("\n\n")
            .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
            .max()
            .unwrap();
    println!("{}", results);
}
