use hashbrown::HashMap;
use std::collections::VecDeque;

#[aoc::day(20, "Pulse Propagation")]
#[aoc::asserts("739960225", "231897990075517")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let modules = input
        .split(line_ending)
        .map(|line| {
            let b = line.as_bytes();
            let i = b.iter().position(|b| *b == b' ').unwrap();
            let offset = usize::from(b[0] == b'b');
            (
                &line[1 - offset..i],
                (b[0], line[i + 4..].split(", ").collect::<Vec<_>>()),
            )
        })
        .collect::<HashMap<_, _>>();
    let to_rx = modules
        .iter()
        .find(|(_, (_, v))| v.contains(&"rx"))
        .unwrap()
        .0;
    let mut states = HashMap::new();
    let mut pre_rx = HashMap::new();
    for (key, (_, links)) in &modules {
        if links.contains(to_rx) {
            pre_rx.insert(key, 0);
        }
        for l in links {
            states
                .entry(l)
                .and_modify(|v: &mut (bool, HashMap<&str, bool>)| {
                    v.1.insert(key, false);
                })
                .or_insert((false, {
                    let mut t = HashMap::new();
                    t.insert(*key, false);
                    t
                }));
        }
    }
    let (mut product, mut sum) = (0, (0, 0));
    let mut pulses = VecDeque::new();
    for cycle in 0.. {
        pulses.push_back(("broadcaster", "button", false));
        if cycle == 1000 {
            product = sum.0 * sum.1
        }
        while let Some((p, prev, is_high)) = pulses.pop_front() {
            if &p == to_rx && is_high {
                *pre_rx.get_mut(&prev).unwrap() = cycle + 1;
            }
            if is_high {
                sum.1 += 1
            } else {
                sum.0 += 1
            };
            let Some((kind, links)) = modules.get(p) else {
                continue;
            };
            let send = match kind {
                b'%' => {
                    if !is_high {
                        states.get_mut(&p).unwrap().0 = !states.get(&p).unwrap().0;
                        states.get(&p).unwrap().0
                    } else {
                        continue;
                    }
                }
                b'&' => {
                    states.get_mut(&p).unwrap().1.insert(prev, is_high);
                    states.get(&p).unwrap().1.values().any(|s| !*s)
                }
                b'b' => is_high,
                _ => unreachable!(),
            };
            pulses.extend(links.iter().map(|&l| (l, p, send)));
        }
        if pre_rx.values().all(|v| *v > 0) {
            break;
        }
    }
    let pre_rx_cycles = pre_rx.values().copied().collect::<Vec<usize>>();
    (product, lcm(&pre_rx_cycles))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(n: &[usize]) -> usize {
    if n.len() == 1 {
        return n[0];
    }
    let recurse = lcm(&n[1..]);
    n[0] / gcd(n[0], recurse) * recurse
}
