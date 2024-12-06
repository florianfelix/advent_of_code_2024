#![allow(unused)]
use itertools::Itertools;
use tracing::info;

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day02".to_uppercase());
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    let reports: Vec<Vec<i32>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // info!("{:#?}", lines);
    // info!("{:?}", reports);

    let mut safe_reports: i32 = 0;

    for r in reports.iter() {
        if report_is_safe(r) {
            safe_reports += 1;
        }
    }

    Ok(safe_reports.to_string())
}

enum Kind {
    Increasing,
    Decreasing,
    Neither,
}

fn report_is_safe(r: &Vec<i32>) -> bool {
    // report is only increasing/deceasing if the last and first
    // elements are the lowest/highest or vice versa
    let kind = match r.iter().position_minmax() {
        itertools::MinMaxResult::MinMax(minpos, maxpos) => {
            if minpos == 0 && maxpos == r.len() - 1 {
                Kind::Increasing
            } else if maxpos == 0 && minpos == r.len() - 1 {
                Kind::Decreasing
            } else {
                Kind::Neither
            }
        }
        _ => Kind::Neither,
    };

    match kind {
        Kind::Neither => {
            // println!("Kind::Neither  {:?}", r);
            return false;
        }
        Kind::Increasing => {
            for w in r.windows(2) {
                let step = w.last().unwrap() - w.first().unwrap();
                if ![1, 2, 3].contains(&step) {
                    // println!("Wrong Stepsize {:?}, {:?}, {:?}", r, w, step);
                    return false;
                }
            }
        }
        Kind::Decreasing => {
            for w in r.windows(2) {
                let step = w.first().unwrap() - w.last().unwrap();
                if ![1, 2, 3].contains(&step) {
                    // println!("Wrong Stepsize {:?}, {:?}, {:?}", r, w, step);
                    return false;
                }
            }
        }
    }

    true
}
