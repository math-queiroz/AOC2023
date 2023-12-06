/* Verbatim implementation of AxlLind's clever and efficient solution (that I rewrote for learning purposes)
 * Can be found in https://github.com/axllind/adventofcode2022/blob/main/src/bin/03.rs */

use hashbrown::HashMap;
#[aoc::puzzle("03.txt")]
#[aoc::assert("544664", "84495585")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let mut symbols = HashMap::<(u8, (usize, usize)), Vec::<usize>>::new();
    let lines = input.split(line_ending).map(str::as_bytes).collect::<Vec<_>>();
    for (y, line) in lines.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let (num_start_index, mut symbol) = (x, None);
            while x < line.len() && line[x].is_ascii_digit() {
                for n in &[(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)] {
                    let pos = ((x as i32 + n.0) as usize, (y as i32 + n.1) as usize);
                    let Some(&n_char) = lines.get(pos.1).and_then(|l| l.get(pos.0)) else {continue};
                    if n_char != b'.' && !n_char.is_ascii_digit() {
                        symbol = Some((n_char, (pos.0, pos.1)));
                        break;
                    }
                }
                x += 1;
            } 
            if x > num_start_index {
                if let Some(symbol) = symbol {
                    let number = line[num_start_index..x].iter().fold(0, |a, n| a * 10 + (n - b'0') as usize);
                    symbols.entry(symbol).or_insert(Vec::new()).push(number);
                }
            }
            x += 1;
        }
    }
    (
        symbols.values().flat_map(|v| v).sum(), 
        symbols.iter().filter(|(&(c,(_,_)), n)| c == b'*' && n.len() == 2).map(|v| v.1[0] * v.1[1]).sum()
    )
}
