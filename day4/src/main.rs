use std::{fs, path::Path};

fn main() {
    let input = fs::read_to_string(Path::new("data")).unwrap();
    let plans = parse_input(&input);
    let result = count_fully_contained(&plans);
    println!("1 : {}", &result);
    let result_2 = count_overlaps(&plans);
    println!("2 : {}", &result_2);
}

fn parse_input(input: &str) -> Vec<Vec<(i32, i32)>> {
    let mut pairs = Vec::new();
    for line in input.lines() {
        let mut pair = Vec::new();
        for elve in line.split(',').into_iter() {
            let from_to = elve.split('-').collect::<Vec<&str>>();
            let from: i32 = from_to[0].parse().unwrap();
            let to: i32 = from_to[1].parse().unwrap();
            pair.push((from, to));
        }
        pairs.push(pair);
    }
    pairs
}

fn count_fully_contained(plans: &[Vec<(i32, i32)>]) -> i32 {
    let mut counter = 0;
    for plan in plans {
        let (from_1, to_1) = plan[0];
        let (from_2, to_2) = plan[1];
        if (from_1 <= from_2 && to_1 >= to_2) || (from_2 <= from_1 && to_2 >= to_1) {
            counter += 1;
        }
    }
    counter
}

fn count_overlaps(plans: &[Vec<(i32, i32)>]) -> i32{
    let mut counter = 0;
    for plan in plans {
        let (from_1, to_1) = plan[0];
        let (from_2, to_2) = plan[1];
        if !(from_1 > to_2) && !(from_2 > to_1){
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{count_fully_contained, parse_input, count_overlaps};

    #[test]
    fn test_day_4() {
        let input = fs::read_to_string(Path::new("test")).unwrap();
        let plans = parse_input(&input);
        assert_eq!(plans.len(), 6);
        let result = count_fully_contained(&plans);
        assert_eq!(result, 2);
        let result_2 = count_overlaps(&plans);
        assert_eq!(result_2, 4);
    }
}
