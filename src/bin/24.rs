use z3::ast::{Ast, Int, Real};
use z3::{Config, Context, Solver};

const L: (f64, f64) = (200000000000000f64, 400000000000000f64);

#[aoc::day(24, "Never Tell Me The Odds")]
#[aoc::asserts("27732", "641619849766168")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let hails = parse(input, line_ending);
    let count = (0..hails.len() - 1)
        .flat_map(|i| (i + 1..hails.len()).map(move |j| (i, j)))
        .fold(0, |count, (i, j)| {
            let (Some(p), fp1, fp2) = intersect(&hails[i], &hails[j]) else {
                return count;
            };
            let bounded = |p: f64| p >= L.0 && p <= L.1;
            count + (bounded(p.0) && bounded(p.1) && fp1 && fp2) as usize
        });
    (count, find_rock_sum(&hails))
}

fn parse(input: String, line_ending: &str) -> Vec<Vec<f64>> {
    input
        .split(line_ending)
        .map(|line| {
            line.split([',', '@'])
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect()
        })
        .collect()
}

fn intersect(p1: &[f64], p2: &[f64]) -> (Option<(f64, f64)>, bool, bool) {
    let theta = |p: &[f64]| if p[3] == 0f64 { 0f64 } else { p[4] / p[3] };
    let (dx1, dx2) = (theta(p1), theta(p2));
    if dx1 == dx2 {
        return (None, false, false);
    };
    let (y01, y02) = (p1[1] - dx1 * p1[0], p2[1] - dx2 * p2[0]);
    let x = (y02 - y01) / (dx1 - dx2);
    let y = y01 + dx1 * x;
    let is_future = |p: &[f64], ax: usize| match p[3 + ax] > 0f64 {
        true => [x, y][ax] > p[ax],
        false => [x, y][ax] < p[ax],
    };
    (
        Some((x, y)),
        is_future(p1, 0) && is_future(p1, 1),
        is_future(p2, 0) && is_future(p2, 1),
    )
}

fn find_rock_sum(hails: &[Vec<f64>]) -> usize {
    let ctx = &Context::new(&Config::new());
    let solver = Solver::new(ctx);
    let [x, dx, y, dy, z, dz] = ["x", "dx", "y", "dy", "z", "dz"].map(|v| Real::new_const(ctx, v));
    for (i, h) in hails.iter().enumerate().take(3) {
        let t = Real::new_const(ctx, format!("t{}", i));
        let h = h
            .iter()
            .map(|v| Int::from_i64(ctx, *v as _).to_real())
            .collect::<Vec<_>>();
        solver.assert(&t.ge(&Real::from_real(ctx, 0, 1)));
        solver.assert(&(&x + &t * &dx)._eq(&(&h[0] + &t * &h[3])));
        solver.assert(&(&y + &t * &dy)._eq(&(&h[1] + &t * &h[4])));
        solver.assert(&(&z + &t * &dz)._eq(&(&h[2] + &t * &h[5])));
    }
    assert_eq!(solver.check(), z3::SatResult::Sat, "Satisfiable");
    let eval = solver.get_model().unwrap().eval(&(x + y + z), true);
    let sum = eval.unwrap().to_string();
    sum[..sum.len() - 2].parse().unwrap()
}
