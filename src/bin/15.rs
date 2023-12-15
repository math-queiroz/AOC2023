use std::collections::VecDeque;
use hashbrown::HashMap;

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |sum, b| ((sum + b as usize) * 17 ) % 256)
}

#[aoc::puzzle("15.txt")]
#[aoc::assert("511343", "294474")]
fn main(input: String, _line_ending: &str) -> (usize, usize) {
    let steps = input.split(",");
    let mut boxes = vec![(
        VecDeque::<&str>::new(),
        HashMap::<&str, usize>::new()
    ); 256];
    for step in steps.clone() {
        let lbl = &step[..step.len()-1].trim_end_matches("=");
        if step.ends_with("-") {
            if let Some(_) = boxes[hash(lbl)].1.remove(lbl) {
                let order = boxes[hash(lbl)].0.iter().position(|l| l==lbl);
                boxes[hash(lbl)].0.remove(order.unwrap());
            } 
        } else {
            let fl = &step[step.len()-1..].parse::<usize>().unwrap();
            let previous = boxes[hash(lbl)].1.insert(lbl, *fl);
            if !previous.is_some() {
                boxes[hash(lbl)].0.push_back(lbl)
            }
        }
    }
    let power = boxes.iter().enumerate().fold(0, |p, (i, (deque, map))|
        p + deque.iter().enumerate().fold(0, |bp, (j, lbl)| {
            bp + ((i+1) * (j+1) * map.get(lbl).unwrap())
        })
    );
    (steps.map(hash).sum(), power)
}