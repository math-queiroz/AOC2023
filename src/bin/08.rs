use hashbrown::HashMap;

fn gcd(a: usize, b: usize) -> usize { 
    if b==0 {return a} else {gcd(b, a % b)}
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
        .map(|l| {(
            l[0..3].to_owned(),
            (l[7..10].to_owned(), l[12..15].to_owned()),
        )})
        .collect::<HashMap<String, (String, String)>>();

    let mut cur_node = "AAA";
    while cur_node != "ZZZ" {
        let (left, right) = nodes.get(cur_node).unwrap();
        cur_node = if dirs[i1 % dirs.len()] == b'L' { left } else { right };
        i1 += 1;
    }

    let mut cur_nodes = nodes
        .keys()
        .filter_map(|l| {
            (&l[2..3] == "A").then(|| Some((l.to_owned(), l)))
        })
        .map(|v| v.unwrap())
        .collect::<HashMap<String, &String>>();
    let mut cycles_lengths = vec![];
    while cycles_lengths.len() < cur_nodes.len() {
        for (start_node, cur_node) in cur_nodes.clone() {
            let (left, right) = nodes.get(cur_node).unwrap();
            if &cur_nodes.get(&start_node).unwrap()[2..3] == "Z" { 
                cycles_lengths.push(i2) 
            }
            if dirs[i2 % dirs.len()] == b'L' {
                *cur_nodes.get_mut(&start_node).unwrap() = left
            } else {
                *cur_nodes.get_mut(&start_node).unwrap() = right
            };
        }
        i2+=1;
    }

    (i1, lcm(&cycles_lengths))
}
