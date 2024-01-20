use std::{fs::read_to_string};
use regex::Regex;
use std::collections::{ VecDeque, BinaryHeap };
use std::cell::RefCell;
use std::boxed::Box;
use std::fmt::Debug;

type Op = dyn Fn(i64) -> i64;

struct Monkey
{
    items: VecDeque<i64>,
    op: Box<Op>,
    test_divisor: i64,
    true_monkey: usize,
    false_monkey: usize,
    inspected: usize,
}

impl Monkey
{
    fn new(items: VecDeque<i64>, op: Box<Op>, test_divisor: i64, true_monkey: usize, false_monkey: usize) -> Self {
        Self {
            items,
            op,
            test_divisor,
            true_monkey,
            false_monkey,
            inspected: 0
        }
    }
}

impl Debug for Monkey
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!("{:?}", self.items));
        let _ = f.write_fmt(format_args!("{}", self.test_divisor));
        let _ = f.write_fmt(format_args!("{}", self.true_monkey));
        let _ = f.write_fmt(format_args!("{}", self.false_monkey));
        f.write_str("\n")
    }
}

fn main() {
    let contents = read_to_string("input.txt").unwrap();
    let monkeys_txt = contents.split("\n\n").collect::<Vec<&str>>();
    
    let mut monkeys = VecDeque::new();
    for monkey_txt in &monkeys_txt {
        let mut line_iter = monkey_txt.lines();
        line_iter.next();

        let mut start_items_line_iter = line_iter.next().unwrap().split(':');
        start_items_line_iter.next();
        let start_items_txt = start_items_line_iter.next().unwrap();
        let mut start_items = VecDeque::new();
        for start_item_txt in start_items_txt.split(',') {
            start_items.push_back(start_item_txt.trim().parse::<i64>().unwrap());
        }
        
        let op_line = line_iter.next().unwrap().trim_start();
        println!("{op_line}");
        let regex = Regex::new(r"Operation: new = old (.) (\d+|old)").unwrap();
        let caps = regex.captures(op_line).unwrap();
        let op_ch = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let op_param = caps.get(2).unwrap().as_str();
        let op: Box<dyn Fn(i64)->i64> = match (op_ch, op_param) {
            ('+', _) => {
                let cp = op_param.parse::<i64>().unwrap();
                Box::new(move |i: i64| i + cp)
            },
            ('-', _) => {
                let cp = op_param.parse::<i64>().unwrap();
                Box::new(move |i: i64| i - cp)
            },
            ('*', "old") => {
                Box::new(move |i: i64| i * i)
            },
            ('*', _) => {
                let cp = op_param.parse::<i64>().unwrap();
                Box::new(move |i: i64| i * cp)
            },
            ('/', _) => {
                let cp = op_param.parse::<i64>().unwrap();
                Box::new(move |i: i64| i / cp)
            },
            _ => panic!("Invalid operation")
        };

        let test_line = line_iter.next().unwrap().trim_start();
        
        let regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
        let caps = regex.captures(test_line).unwrap();
        let divisor = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        
        let true_line = line_iter.next().unwrap().trim_start();
        let regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
        let caps = regex.captures(true_line).unwrap();
        let true_monkey = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        
    
        let false_line = line_iter.next().unwrap().trim_start();
        let regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
        let caps = regex.captures(false_line).unwrap();
        let false_monkey = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

        let new_monkey = Monkey::new(start_items, op, divisor, true_monkey, false_monkey);
        
        monkeys.push_back(RefCell::from(new_monkey));
    }

    println!("{:?}", monkeys);

    for _ in 0..20 {
        for monkey in &monkeys {
            loop {
                if monkey.borrow().items.is_empty() {
                    break;
                }
                let mut item = monkey.borrow_mut().items.pop_front().unwrap();
                println!("Inspecting item {}", item);
                monkey.borrow_mut().inspected += 1;
                item = monkey.borrow().op.as_ref()(item);
                println!("Op returned {}", item);
                item /= 3;
                println!("Division by 3: {}", item);
                let divisible = (item % monkey.borrow().test_divisor) == 0;
                if divisible {
                    println!("Divisible by {}", monkey.borrow().test_divisor);
                    monkeys.get(monkey.borrow().true_monkey).unwrap().borrow_mut().items.push_back(item);
                } else {
                    println!("Not divisible by {}", monkey.borrow().test_divisor);
                    monkeys.get(monkey.borrow().false_monkey).unwrap().borrow_mut().items.push_back(item);
                }
                println!();
            }
            println!("==================================");
        }
    }

    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected {} items", idx, monkey.borrow().inspected);
    }

    let mut the_heap = monkeys.iter().map(|m| m.borrow().inspected).collect::<BinaryHeap<_>>();
    let monkey_business = the_heap.pop().unwrap() * the_heap.pop().unwrap();

    println!("Monkey business = {}", monkey_business);
}
