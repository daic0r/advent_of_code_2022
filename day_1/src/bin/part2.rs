use std::collections::BinaryHeap;

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

    let mut sorted = BinaryHeap::new();
    elves.iter().for_each(|v| sorted.push(v.iter().sum::<u32>()));

    let mut sum = 0;
    for _ in 0..3 {
        sum += sorted.pop().unwrap();
    }
    println!("Top 3: {sum}");
}
