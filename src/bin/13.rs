fn find_mirror(rows: &Vec<Vec<u8>>, limit: usize) -> Option<usize> {
    for i in 1..rows.len() {
        let (half1, half2) = (&rows[..i], &rows[i..]);
        let min = half1.len().min(half2.len());
        let (h1, h2) = (
            half1.iter().rev().take(min).collect::<Vec<_>>(), 
            half2.iter().take(min).collect::<Vec<_>>()
        );
        let diff = h1.iter().zip(h2.iter()).fold(0, |diff, (a1, a2)| diff + a1.iter().zip(a2.iter()).fold(0, |d, (v1, v2)| d + (v1!=v2) as usize));
        if  diff == limit { return Some(i) }
    }
    None
}

fn cols_from_rows(rows: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let r = rows[1..].iter().fold(
        rows[0].iter().map(|b| vec![*b]).collect::<Vec<_>>(),
        |mut acc, l| {
            l.iter().enumerate().for_each(|(i, b)| acc[i].push(*b));
            acc
        },
    );
    r
}

#[aoc::puzzle("13.txt")]
#[aoc::assert("34100", "33106")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let double_le = line_ending.repeat(2);
    let grids = input.split(&double_le).map(|g| {
        g.split(line_ending)
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<_>>()
    });
    grids 
        .map(|rows| {(
            find_mirror(&rows, 0)
                .map(|v| v * 100)
                .unwrap_or_else(|| find_mirror(&cols_from_rows(&rows), 0).unwrap()),
            find_mirror(&rows, 1)
                .map(|v| v * 100)
                .unwrap_or_else(|| find_mirror(&cols_from_rows(&rows), 1).unwrap()),
        )})
        .fold((0,0), |acc, v| (acc.0+v.0, acc.1+v.1))
}
