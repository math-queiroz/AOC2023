#[aoc::puzzle("11.txt")]
#[aoc::assert("9647174", "377318892554")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let lines = input
        .split(line_ending)
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();
    let glxs = lines.iter()
        .enumerate()
        .flat_map(|(y, l)| 
            l.iter().enumerate().filter_map(move |(x, c)| 
                if *c==b'#' { Some((c,(x,y))) }
                else { None })
        )
        .collect::<Vec<_>>();
    let expd_lin = lines.iter().skip(1)
        .fold(vec![0], |mut acc, l| {
            acc.push(acc[acc.len()-1] + l.iter().all(|c| *c==b'.') as usize);
            acc
        });
    let mut cols = lines[0].iter().map(|c| vec![c]).collect::<Vec<_>>();
    let mut expd_col = Vec::<usize>::new();
    for row in &lines[1..] {
        for (acc, col) in cols.iter_mut().zip(row.into_iter()) { acc.push(col) }
        expd_col = cols.iter().skip(1)
            .fold(vec![0], |mut acc, c| {
                acc.push(acc[acc.len()-1] + c.iter().all(|c| *c==&b'.') as usize);
                acc
            });
    }
    let mut paths = (Vec::<usize>::new(), Vec::<usize>::new());
    for i1 in 0..glxs.len() {
        for i2 in i1+1..glxs.len() {
            let (min_x, max_x) = (glxs[i1].1.0.min(glxs[i2].1.0), glxs[i1].1.0.max(glxs[i2].1.0));
            let (min_y, max_y) = (glxs[i1].1.1.min(glxs[i2].1.1), glxs[i1].1.1.max(glxs[i2].1.1));
            let added_cols = expd_col[max_x as usize] - expd_col[min_x as usize];
            let added_lin = expd_lin[max_y as usize] - expd_lin[min_y as usize];
            paths.0.push(max_x-min_x+added_cols + max_y-min_y+added_lin);
            paths.1.push(max_x-min_x+added_cols*999999 + max_y-min_y+added_lin*999999);
        }
    }
    (paths.0.iter().sum(), paths.1.iter().sum())
}