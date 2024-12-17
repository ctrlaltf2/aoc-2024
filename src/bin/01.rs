use std::collections::BTreeMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<u32>().unwrap());
        right.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    Some(left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(b.clone()))
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = BTreeMap::new();
    let mut right = BTreeMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        let left_part = parts.next().unwrap().parse::<u32>().unwrap();
        let right_part = parts.next().unwrap().parse::<u32>().unwrap();

        left.entry(left_part)
            .and_modify(|curr| *curr += 1)
            .or_insert(1);

        right.entry(right_part)
                .and_modify(|curr| *curr += 1)
                .or_insert(1);
    }

    let mut combined_score = 0;
    for (l, l_count) in &left {
        combined_score += l * l_count * right.get(&l).unwrap_or(&0);
    }

    Some(combined_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
