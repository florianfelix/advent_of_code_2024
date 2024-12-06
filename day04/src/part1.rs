#![allow(unused)]
use array2d::Array2D;
use miette::IntoDiagnostic;
use tracing::info;

#[tracing::instrument(skip(input, test))]
pub fn process(input: &str, test: &str) -> miette::Result<String> {
    info!("{} - PART 1", "day04".to_uppercase());
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let lines: Vec<Vec<String>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect())
        .collect();

    let arr: Array2D<String> = Array2D::from_rows(&lines).into_diagnostic()?;

    let t = arr.count_all_xmas();
    // info!("{:?}", t);

    Ok(t.to_string())
}

trait ArrayExt {
    fn count_all_xmas(&self) -> i32;
    fn count_xmas_at(&self, x: usize, y: usize) -> i32;
    fn test_right(&self, x: usize, y: usize) -> Option<String>;
    fn test_left(&self, x: usize, y: usize) -> Option<String>;
    fn test_down(&self, x: usize, y: usize) -> Option<String>;
    fn test_up(&self, x: usize, y: usize) -> Option<String>;
    fn test_right_down(&self, x: usize, y: usize) -> Option<String>;
    fn test_right_up(&self, x: usize, y: usize) -> Option<String>;
    fn test_left_down(&self, x: usize, y: usize) -> Option<String>;
    fn test_left_up(&self, x: usize, y: usize) -> Option<String>;
}

impl ArrayExt for Array2D<String> {
    fn count_all_xmas(&self) -> i32 {
        let mut count: i32 = 0;

        for x in 0..self.num_columns() {
            for y in 0..self.num_rows() {
                count += self.count_xmas_at(x, y);
            }
        }

        count
    }
    fn count_xmas_at(&self, x: usize, y: usize) -> i32 {
        let mut count: i32 = 0;
        if let Some(chars) = self.test_right(x, y) {
            if chars.contains("XMAS") {
                count += 1
            }
        }
        if let Some(chars) = self.test_left(x, y) {
            if chars.contains("XMAS") {
                count += 1
            }
        }
        if let Some(chars) = self.test_up(x, y) {
            if chars.contains("XMAS") {
                count += 1
            }
        }
        if let Some(chars) = self.test_down(x, y) {
            if chars.contains("XMAS") {
                count += 1
            }
        }
        if let Some(chars) = self.test_right_up(x, y) {
            if chars.contains("XMAS") {
                count += 1
            }
        }
        if let Some(chars) = self.test_left_up(x, y) {
            if chars.contains("XMAS") {
                count += 1
            }
        }
        if let Some(chars) = self.test_right_down(x, y) {
            if chars.contains("XMAS") {
                count += 1
            }
        }
        if let Some(chars) = self.test_left_down(x, y) {
            if chars.contains("XMAS") {
                count += 1
            }
        }
        count
    }
    fn test_right(&self, x: usize, y: usize) -> Option<String> {
        let mut res: String = String::new();
        for i in 0..4 {
            if let Some(val) = self.get(y, x + i) {
                res.push_str(val);
            } else {
                return None;
            }
        }
        Some(res)
    }
    fn test_left(&self, x: usize, y: usize) -> Option<String> {
        let mut res: String = String::new();
        if x < 3 {
            return None;
        }
        for i in 0..4 {
            if let Some(val) = self.get(y, x - i) {
                res.push_str(val);
            } else {
                return None;
            }
        }
        Some(res)
    }
    fn test_down(&self, x: usize, y: usize) -> Option<String> {
        let mut res: String = String::new();
        for i in 0..4 {
            if let Some(val) = self.get(y + i, x) {
                res.push_str(val);
            } else {
                return None;
            }
        }
        Some(res)
    }
    fn test_up(&self, x: usize, y: usize) -> Option<String> {
        if y < 3 {
            return None;
        }
        let mut res: String = String::new();
        for i in 0..4 {
            if let Some(val) = self.get(y - i, x) {
                res.push_str(val);
            } else {
                return None;
            }
        }
        Some(res)
    }
    fn test_right_down(&self, x: usize, y: usize) -> Option<String> {
        let mut res: String = String::new();
        for i in 0..4 {
            if let Some(val) = self.get(y + i, x + i) {
                res.push_str(val);
            } else {
                return None;
            }
        }
        Some(res)
    }
    fn test_right_up(&self, x: usize, y: usize) -> Option<String> {
        if y < 3 {
            return None;
        }
        let mut res: String = String::new();
        for i in 0..4 {
            if let Some(val) = self.get(y - i, x + i) {
                res.push_str(val);
            } else {
                return None;
            }
        }
        Some(res)
    }
    fn test_left_down(&self, x: usize, y: usize) -> Option<String> {
        if x < 3 {
            return None;
        }
        let mut res: String = String::new();
        for i in 0..4 {
            if let Some(val) = self.get(y + i, x - i) {
                res.push_str(val);
            } else {
                return None;
            }
        }
        Some(res)
    }
    fn test_left_up(&self, x: usize, y: usize) -> Option<String> {
        if y < 3 || x < 3 {
            return None;
        }
        let mut res: String = String::new();
        for i in 0..4 {
            if let Some(val) = self.get(y - i, x - i) {
                res.push_str(val);
            } else {
                return None;
            }
        }
        Some(res)
    }
}
