use hashbrown::{ HashMap, HashSet };

// Bitwise Direction Connection Order = U R L D
const PIPES: [(u8, usize); 8] = [
    (b'|', 0b1001), (b'-', 0b0110), (b'L', 0b1100), (b'J', 0b1010), 
    (b'7', 0b0011), (b'F', 0b0101), (b'.', 0b0000), (b'S', 0b1111)];
const NEIGHBOURS: [((isize, isize), (usize, usize)); 4] = [
    ((0,-1), (0b1000, 0b0001)), (( 1,0), (0b0100, 0b0010)), 
    ((0, 1), (0b0001, 0b1000)), ((-1,0), (0b0010, 0b0100))];

#[aoc::puzzle("10.txt")]
#[aoc::assert("7173", "291")]
fn main(input: String, line_ending: &str) -> (isize, isize) {
    let connections = 
        PIPES
        .into_iter()
        .collect::<HashMap<_,_>>();
    let mut shape_pipes = HashSet::new();
    let mut pipes = input
        .split(line_ending)
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let s_pos = pipes
        .iter()
        .enumerate()
        .find_map(|(y,l)| l.iter()
            .enumerate()
            .find_map(|(x,v)| 
                if *v == b'S' { Some((x,y)) } 
                else { None })
            )
            .unwrap();
    let (mut i, mut prev_pos, mut pos) = (0, s_pos, s_pos);
    let (mut min_coord, mut max_coord) = ((usize::MAX, usize::MAX), (0, 0));
    loop {
        i += 1;
        shape_pipes.insert(pos);
        let next = NEIGHBOURS.iter().find_map(|n| {
            let p = ((pos.0 as isize + n.0.0) as usize, (pos.1 as isize + n.0.1) as usize);
            if p == prev_pos { return None };
            let Some(pipe) = pipes.get(p.1).and_then(|l| l.get(p.0)) else { return None };
            let conn1 = connections.get(pipes.get(pos.1).and_then(|l| l.get(pos.0)).unwrap()).unwrap();
            let conn2 = connections.get(pipe).unwrap();
            if conn1 & n.1.0 > 0 && conn2 & n.1.1 > 0 { Some(p) }
            else { None }
        }).unwrap();
        prev_pos = pos; 
        pos = next;
        min_coord = (min_coord.0.min(next.0), min_coord.1.min(next.1));
        max_coord = (max_coord.0.max(next.0), max_coord.1.max(next.1));
        if *pipes.get(pos.1).and_then(|l| l.get(pos.0)).unwrap() == b'S' {
            break
        }
    }
    pipes[s_pos.1][s_pos.0] = match (
        "|F7".as_bytes().contains(&pipes.get(s_pos.1-1).and_then(|l| l.get(s_pos.0)).unwrap_or(&b'S')),
        "|LJ".as_bytes().contains(&pipes.get(s_pos.1+1).and_then(|l| l.get(s_pos.0)).unwrap_or(&b'S'))
    ) {
        (true, true) => b'|',  (true, false) => b'J', (false, true) => b'7',
        _ => unreachable!()
    };
    let ii = pipes.clone()[min_coord.1..max_coord.1+1].iter().enumerate().fold(0, |count, (y, l)| {
        let mut points_inside = count;
        l[min_coord.0..max_coord.0+1].
            iter()
            .enumerate()
            .fold((0,0), |parity, (x, v)| {
                if shape_pipes.contains(&(x+&min_coord.0,y+&min_coord.1)) {(
                    parity.0 + "|F7".as_bytes().contains(v) as u8,
                    parity.1 + "|LJ".as_bytes().contains(v) as u8
                )} else { 
                    if parity.0%2==1 || parity.1%2==1 {  points_inside += 1 };
                    parity
                 }
            });
        points_inside
    });
    (i/2, ii)
}