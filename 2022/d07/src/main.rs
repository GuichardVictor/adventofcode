use std::iter::Peekable;

fn get_size(lines: &mut Peekable<impl Iterator<Item = &'static str>>, total_size: &mut usize) -> usize {
    let mut directory_size = 0usize;

    while let Some(line) = lines.next() {
        match line {
            "$ cd .." => break, // Break and return the current directory size
            "$ ls" => { // List files and compute directory content size
                directory_size = std::iter::from_fn(|| // Iter until the line is a command
                    lines.next_if(|b| !b.starts_with('$'))
                ).filter(|line| !line.starts_with("dir")) // Skip directory
                .filter_map(|line| {
                     line.split(' ').next().unwrap().parse::<usize>().ok()
                }).sum()
            },
            _ => directory_size += get_size(lines, total_size) // Compute subdirectory size
        }
    }

    if directory_size < 100_000 {
        *total_size += directory_size;
    }

    directory_size
}

fn part_1() {
    let input = include_str!("../input");
    let mut total_size = 0usize;

    get_size(
        &mut input.split('\n').peekable(),
        &mut total_size
    );

    println!("{}", total_size);
}

fn main() {
    part_1();
}
