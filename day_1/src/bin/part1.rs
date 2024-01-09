fn main() {
    let contents = include_str!("../../input.txt").lines();

    let mut elves = vec![ vec! [] ];

    let mut cur_elf = elves.last_mut().unwrap();
    for l in contents {
        if l.is_empty() {
            elves.push(Default::default());
            cur_elf = elves.last_mut().unwrap();
            continue;
        }
        cur_elf.push(l.parse::<u32>().unwrap());
    }

    let the_max = elves.iter().map(|v| v.iter().sum::<u32>()).max().unwrap();
    println!("Max elf: {the_max}");
}
