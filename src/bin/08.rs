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

#[aoc::puzzle("08.txt")]
#[aoc::assert("13207", "12324145107121")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let (dirs, input) = input.split_once(&line_ending.repeat(2)).unwrap();
    let (dirs, mut i1, mut i2) = (dirs.as_bytes(), 0, 0);
    let nodes = input
        .split(line_ending)
        .map(|l| (&l[0..3], (&l[7..10], &l[12..15])))
        .collect::<HashMap<_, _>>();
    let mut n = "AAA";
    while n != "ZZZ" {
        let (l, r) = nodes.get(n).unwrap();
        n = if dirs[i1 % dirs.len()] == b'L' { l } else { r };
        i1 += 1;
    }
    let mut ns = nodes
        .keys()
        .filter(|l| l.ends_with("A"))
        .map(|v| (v, v))
        .collect::<HashMap<_, _>>();
    let mut cycles = vec![];
    while cycles.len() < ns.len() {
        for (start_node, cur_node) in ns.clone() {
            let (left, right) = nodes.get(cur_node).unwrap();
            if ns.get(&start_node).unwrap().ends_with("Z") {
                cycles.push(i2)
            }
            *ns.get_mut(&start_node).unwrap() = match dirs[i2 % dirs.len()] {
                b'L' => left,
                b'R' => right,
                _ => panic!("Bad input!"),
            };
        }
        i2 += 1;
    }
    (i1, lcm(&cycles))
}
