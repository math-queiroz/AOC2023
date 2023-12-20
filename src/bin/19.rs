use hashbrown::HashMap;

type Workflows<'a> = HashMap<&'a str, (Vec<(usize, &'a str, usize, &'a str)>, &'a str)>;

fn process(workflows: &Workflows, piece: &Vec<usize>) -> bool {
    let mut lbl = "in";
    let mut res = None;
    while res.is_none() {
        let (rules, defaults) = workflows.get(lbl).unwrap();
        let mut tlbl = lbl;
        for (i, cmp, val, nlbl) in rules {
            match cmp {
                &"<" => if piece[*i] < *val { tlbl = nlbl; break; },
                &">" => if piece[*i] > *val { tlbl = nlbl; break; },
                _ => { unreachable!() }
            }
        }
        lbl = if tlbl != lbl { tlbl } else { defaults };
        if lbl=="A" || lbl=="R" { res = Some(lbl=="A"); }
    }
    res.unwrap()
}

fn sum_distinct(workflows: &Workflows, mut ranges: Vec<(usize, usize)>, lbl: &str) -> usize {
    return match lbl {
        "R" => 0,
        "A" => ranges.iter().map(|v| v.1-v.0+1).product(),
        _ => {
            let (rules, defaults) = workflows.get(lbl).unwrap();
            let mut has_defaults = true;
            let mut acc = 0;
            for (i, cmp, val, nlbl) in rules {
                let (min, max) = ranges[*i];
                let (keep, pass) = match cmp {
                    &">" => ((min,*val),(val+1,max)),
                    &"<" => ((*val,max),(min,val-1)),
                    _ => unreachable!()
                };
                if pass.0 <= pass.1 {
                    let mut nranges = ranges.clone();
                    nranges[*i] = pass;
                    acc += sum_distinct(workflows, nranges, nlbl);
                }
                if keep.0 <= keep.1 {
                    ranges[*i] = keep;
                } else {
                    has_defaults = false;
                    break;
                }
            }
            if has_defaults {
                acc += sum_distinct(workflows, ranges, defaults);
            }
            acc
        }
    }
}

#[aoc::puzzle("19.txt")]
#[aoc::assert("509597", "143219569011526")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let input = input.split_once(&line_ending.repeat(2)).unwrap();
    let workflows = input.0.split(line_ending).map(|line| {
        let crly = line.find(|c| c=='{').unwrap();
        let trim = &line[crly+1..line.len()-1];
        let splt = trim.split(",").collect::<Vec<_>>();
        let stps = splt.iter().take(splt.len()-1).map(|flow| {
            let coln = flow.find(|c| c==':').unwrap();
            (
                "xmas".find(&flow[0..1]).unwrap(), 
                 &flow[1..2], 
                 flow[2..coln].parse::<usize>().unwrap(), 
                 &flow[coln+1..]
            )
        })
        .collect::<Vec<_>>();
        (&line[..crly], (stps, *splt.last().unwrap()))
        })
        .collect::<HashMap<_,_>>();
    let pieces = input.1
        .split(line_ending)
        .map(|line| line[1..line.len()-1]
            .split(',')
            .map(|rate| rate[2..].parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        );
    let sum = pieces
        .filter(|p| process(&workflows, p))
        .map(|r| r.iter().sum::<usize>())
        .sum();
    (sum, sum_distinct(&workflows, vec![(1,4000);4], "in"))
}

