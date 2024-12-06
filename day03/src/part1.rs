#![allow(unused)]
use string_patterns::PatternCapture;
use tracing::info;

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day02".to_uppercase());
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    // info!("{:#?}", lines);

    let mut result: i32 = 0;

    for l in lines {
        // println!("{:?}", l);
        result += scan_line(l);
    }

    Ok(result.to_string())
}

fn scan_line(line: String) -> i32 {
    let mut line_sum: i32 = 0;
    let muls: Vec<String> = vec![];

    let pattern = r#"mul\(\d{1,3},\d{1,3}\)"#;
    let matches = line.pattern_matches_vec(pattern, false);
    // println!("{:#?}", matches);

    for m in matches {
        let pattern_digits = r#"\d{1,3}"#;
        let matches_digits = m.as_str().pattern_matches_vec(pattern_digits, false);
        // println!("{:#?}", matches_digits);
        let a: i32 = matches_digits
            .first()
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let b: i32 = matches_digits
            .last()
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        line_sum += a * b;
    }

    line_sum
}
