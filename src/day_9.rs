pub fn p1(input: &str) {
    let first_rows = parse(input);
    let pyramids = first_rows.iter().map(diff_pyramid);
    let extrapolations = pyramids.map(|pyramid: Vec<Vec<i64>>| extrapolate(&pyramid));
    let sum = extrapolations.sum::<i64>();
    println!("The sum of extrapolated values is {sum}");
}

pub fn p2(input: &str) {
    let first_rows = parse(input);
    let pyramids = first_rows.iter().map(diff_pyramid);
    let extrapolations = pyramids.map(|pyramid: Vec<Vec<i64>>| extrapolate_prev(&pyramid));
    let sum = extrapolations.sum::<i64>();
    println!("The sum of prev extrapolated values is {sum}");
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|x| i64::from_str_radix(x, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn diff_pyramid(v: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut pyramid = vec![v.clone()];
    while pyramid.last().unwrap().iter().any(|x| *x != 0) {
        let prev_row = pyramid.last().unwrap();
        let mut new_row = Vec::with_capacity(prev_row.len() - 1);
        for idx in 0..(prev_row.len() - 1) {
            new_row.push(prev_row[idx + 1] - prev_row[idx]);
        }
        pyramid.push(new_row);
    }
    pyramid
}

fn extrapolate(pyramid: &Vec<Vec<i64>>) -> i64 {
    let mut l = 0;
    pyramid
        .iter()
        .rev()
        .skip(1)
        .for_each(|x| l = l + x.last().unwrap());
    l
}

fn extrapolate_prev(pyramid: &Vec<Vec<i64>>) -> i64 {
    let mut l = 0;
    pyramid
        .iter()
        .rev()
        .skip(1)
        .for_each(|x| l = x.first().unwrap() - l);
    l
}
