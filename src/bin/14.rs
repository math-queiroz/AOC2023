use hashbrown::HashMap;

const CYCLES: usize = 1000000000;

fn move_all(grid: &mut Vec<Vec<u8>>, stones: &mut Vec<(usize, usize)>) {
  let mut moving = true;
  while moving {
    moving = false;
    for (i, (x,y)) in stones.clone().into_iter().enumerate() {
      if grid.get(y-1).and_then(|row| Some(row[x])) == Some(b'.') {
          moving = true;
          grid[y][x] = b'.';
          grid[y-1][x] = b'O';
          stones[i] = (x,y-1);
      }
    }
  }
}

fn rotated_grid(grid: &mut Vec<Vec<u8>>, stones: &mut Vec<(usize, usize)>) {
  let mut rotated = vec![vec![0;grid[0].len()];grid.len()];
  (0..grid.len()).for_each(|y| (0..grid[0].len()).for_each(|x| {
    rotated[x][(grid[0].len()-1)-y] = grid[y][x]
  }));
  let translated = stones.iter().map(|c| ((grid[0].len()-1)-c.1, c.0)).collect::<Vec<_>>();
  (*grid, *stones) = (rotated, translated);
}

fn eval_north_stress(grid: &mut Vec<Vec<u8>>) -> usize {
    grid.iter().rev().enumerate().fold(0, |l_acc, (row, l)| {
        l_acc + 
        l.iter()
          .fold(0, |c_acc, c| 
            c_acc + (row + 1) * (*c == b'O') as usize)
    })
}

#[aoc::puzzle("14.txt")]
#[aoc::assert("113424", "96003")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let mut stones: Vec<(usize, usize)> = Vec::new();
    let mut grid = input
        .split(line_ending)
        .enumerate()
        .map(|(y, l)| {
            let b = l.as_bytes().to_vec();
            b.iter().enumerate().for_each(|(x, b)| {
                if *b == b'O' { stones.push((x, y)) }
            });
            b
        })
        .collect::<Vec<_>>();

    let (mut p1_grid, mut p1_resting) = (grid.clone(), stones.clone());
    move_all(&mut p1_grid, &mut p1_resting);

    let mut cfgs = HashMap::new();
    for i in 1..CYCLES {
      for _rot in 0..4 {
        move_all(&mut grid, &mut stones);
        rotated_grid(&mut grid, &mut stones);
      }
      if let Some(cfg_i) = cfgs.insert(grid.clone(),i) {
        if (CYCLES - i) % (i - cfg_i) == 0 { break };
      }
    }
    (eval_north_stress(&mut p1_grid), eval_north_stress(&mut grid)
  )
}