// Shoelace formula [A = sum(xi(yi-1 + yi))/2] 
// Pick's theorem [A = b/2 + i -1] becomes [i = A + 1 - b/2]
fn calc_area(plan: Vec<(u8, isize)>) -> usize {
  let mut min = (0isize, 0isize);
  let p = plan.iter().fold((vec![(0, 0)], 0, 0), |(mut acc, mut x, mut y), (dir, n)| {
    match dir {
      b'U' => y+=n,
      b'D' => y-=n,
      b'L' => x-=n,
      b'R' => x+=n,
      _ => unreachable!()
    }
    min = (min.0.min(x), min.1.min(y));
    acc.push((x,y));
    (acc, x, y)
  }).0;
  let area = (1..p.len()).fold(0, |sum, i| sum + p[i].0*(p[(i+1)%p.len()].1-p[i-1].1)).abs() / 2;
  (area + 1 + plan.iter().fold(0, |sum, (_,n)| sum + n) / 2) as usize
}

#[aoc::puzzle("18.txt")]
#[aoc::assert("28911", "77366737561114")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
  let lines = input.split(line_ending);
  let plans = lines.clone().map(|l| (l.as_bytes()[0], l[2..l.len()-10].parse::<isize>().unwrap()));
  let big_plans = lines.map(|l| {(
    [b'R',b'D',b'L',b'U'][l[l.len()-2..l.len()-1].parse::<usize>().unwrap()],
    isize::from_str_radix(&l[l.len()-7..l.len()-2], 16).unwrap()
  )});
  (calc_area(plans.collect()), calc_area(big_plans.collect()))
}