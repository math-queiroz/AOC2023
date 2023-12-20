use hashbrown::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { return a } 
    else { gcd(b, a % b) }
}

fn lcm(n: &[usize]) -> usize {
    if n.len() == 1 { return n[0] }
    let recurse = lcm(&n[1..]);
    n[0] / gcd(n[0], recurse) * recurse
}

#[aoc::puzzle("20.txt")]
#[aoc::assert("739960225", "231897990075517")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let modules = input
        .split(line_ending)
        .map(|line| {
            let b = line.as_bytes();
            let i = b.iter().position(|b| *b==b'>').unwrap();
            let offset = usize::from(b[0] == b'b');
            (&line[1-offset..i-2], (b[0], line[i+2..].split(", ").collect::<Vec<_>>()))
        })
        .collect::<HashMap<_,_>>();
    let to_rx = modules.iter().find(|(_,(_,v))| v.contains(&"rx")).unwrap().0;
    let mut mod_states = HashMap::new();
    let mut pre_rx = HashMap::new();
    for (key, (_, links)) in &modules {
        for l in links {
            mod_states.entry(l)
                .and_modify(|v: &mut (bool, HashMap<&str, bool>)| {
                    v.1.insert(key, false);
                })
                .or_insert((false, HashMap::new()));
            if links.contains(&to_rx) {
                pre_rx.insert(key, 0);
            }
        }
    }
    let (mut product, mut sum) = (0, (0,0));
    for cycle in 0.. {
        if cycle == 1718 {
            println!("CYCLE {}\nbutton -low-> broadcaster",cycle) 
        };
        if cycle == 1000 {
            product = sum.0 * sum.1;
        }
        let (mut pi, mut pulses) = (0, vec![("broadcaster", "button", false)]);
        let (mut ls, mut hs) = (1,0);
        while pi < pulses.len() {
            let (p, prev, is_high) = pulses.get(pi).unwrap().clone();
            pi += 1;
            let Some((kind, links)) = modules.get(p) else { continue };
            let send = match kind {
                b'%' => if !is_high {
                    mod_states.get_mut(&p).unwrap().0 = !mod_states.get(&p).unwrap().0;
                    Some(mod_states.get(&p).unwrap().0)
                } else { None }, 
                b'&' => {
                    mod_states.get_mut(&p).unwrap().1.insert(prev, is_high);
                    Some(mod_states.get(&p).unwrap().1.values().any(|s| !*s))
                },
                b'b' => Some(is_high),
                _ => unreachable!()
            };
            if let Some(send_high) = send {
                if &p == to_rx && is_high {
                    println!("{} sent high at {}", prev, cycle);
                    *pre_rx.get_mut(&prev).unwrap() = cycle + 1;
                } 
                for link in links {
                    if cycle == 1718 {
                        println!("{} -{}-> {}", p, if send_high {"high"} else {"low"}, link);
                    }
                    if send_high { hs += 1 } else { ls += 1 };
                    pulses.push((link, p, send_high));
                }
            }
        }
        sum = (ls + sum.0, hs + sum.1);
        if pre_rx.values().all(|v| *v > 0) { break }
    }
    let pre_rx_cycles = pre_rx.values().map(|v| *v).collect::<Vec<usize>>();
    (product, lcm(&pre_rx_cycles))
}
