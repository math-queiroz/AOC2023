use hashbrown::HashSet;

fn bounded<T>(pos: (isize, isize), grid: &Vec<&[T]>) -> bool {
  pos.0 >= 0 && pos.0 < grid[0].len() as isize && 
  pos.1 >= 0 && pos.1 < grid.len() as isize
}

fn trace_from(grid: &Vec<&[u8]>, pos: (isize, isize), dir: (isize, isize)) -> usize {
  let (mut seen, mut beams) = (HashSet::new(), vec![vec![(pos,dir)]]);
  while beams.len() > 0 {
    let mut i = 0;
    while i < beams.len() {
      let ((x,y),(dx,dy)) = beams[i][beams[i].len()-1];
      let tip = match (grid[y as usize][x as usize], (dx,dy)) {
        (b'|', (_,1)) | (b'|', (_,-1)) | 
        (b'-', (1,_)) | (b'-', (-1,_)) | 
        (b'.', (_,_)) => vec![((dx+x,dy+y), ( dx, dy))],
        (b'/', (_,_)) => vec![((x-dy,y-dx), (-dy,-dx))],
        (b'\\',(_,_)) => vec![((dy+x,dx+y), ( dy, dx))],
        (b'|', (_,_)) => vec![((x,y-1), (0isize,-1isize)), ((x,y+1),(0isize,1isize))],
        (b'-', (_,_)) => vec![((x-1,y), (-1isize,0isize)), ((x+1,y),(1isize,0isize))],
        _ => unreachable!()
      };
      if bounded(tip[0].0, &grid) && !seen.contains(&tip[0]) { 
        seen.insert(tip[0].clone());
        beams[i].push(tip[0].clone());
      } else { 
        beams.swap_remove(i);
        i -= 1;
      }
      if tip.get(1).is_some() && bounded(tip[1].0, &grid) { 
        seen.insert(tip[1].clone());
        beams.push(vec![tip[1].clone()]);
      }
      i += 1;
    }
  }
  seen.into_iter().map(|(p,_)| p).collect::<HashSet<_>>().len() + 1
}

#[aoc::puzzle("16.txt")]
#[aoc::assert("6883", "7228")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
  let grid: Vec<&[u8]> = input.split(line_ending).map(str::as_bytes).collect();
  let (w,h) = (grid[0].len() as isize, grid.len() as isize);
  let corners = 
    (0..h).map(|i| (( 0, i),( 1,0))).chain(
    (0..h).map(|i| ((w-1,i),(-1,0))).chain(
    (0..w).map(|i| (( i, 0),(0, 1))).chain(
    (0..w).map(|i| ((i,h-1),(0,-1))))));
  (
    trace_from(&grid,(0,0),(1,0)), 
    corners.map(|(pos, dir)| trace_from(&grid, pos, dir)).max().unwrap()
  )
}