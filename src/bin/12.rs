use hashbrown::HashMap;
use itertools::Itertools;

#[aoc::day(12, "Hot Springs")]
#[aoc::asserts("7622", "4964259839627")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    input
        .split(line_ending)
        .map(|l| {
            let (substr, nums) = l.split_once(' ').unwrap();
            let nums = nums
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let substr5 = (0..5).map(|_| substr).join("?");
            let nums5 = nums.repeat(5);
            (
                arrangement_count(&mut HashMap::new(), substr.as_bytes(), &nums),
                arrangement_count(&mut HashMap::new(), substr5.as_bytes(), &nums5),
            )
        })
        .fold((0, 0), |sum, (s1, s2)| (sum.0 + s1, sum.1 + s2))
}

fn arrangement_count(
    memoize: &mut HashMap<(usize, usize), usize>,
    substr: &[u8],
    nums: &[usize],
) -> usize {
    if substr.is_empty() {
        return if nums.is_empty() { 1 } else { 0 };
    }
    if nums.is_empty() {
        return if substr.contains(&b'#') { 0 } else { 1 };
    }
    let (mut acc, s0, n0) = (0, substr[0], nums[0]);
    if let Some(hit) = memoize.get(&(substr.len(), nums.len())) {
        return *hit;
    }
    if s0 == b'.' || s0 == b'?' {
        acc += arrangement_count(memoize, &substr[1..], nums)
    }
    if s0 == b'#' || s0 == b'?' {
        let fits = n0 <= substr.len();
        let none_operational = !substr.get(..n0).map(|r| r.contains(&b'.')).unwrap_or(true);
        let next_not_damaged =
            nums[0] == substr.len() || substr.get(n0).map(|r| *r != b'#').unwrap_or(false);
        if fits && none_operational && next_not_damaged {
            acc += arrangement_count(memoize, &substr[(n0 + 1).min(substr.len())..], &nums[1..])
        }
    }
    memoize.insert((substr.len(), nums.len()), acc);
    acc
}
