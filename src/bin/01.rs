use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
   let (mut first_numbers, mut second_numbers): (Vec<u32>, Vec<u32>) = input
       .lines()
       .filter_map(|line| {
           let parts: Vec<&str> = line.split_whitespace().collect();
           if parts.len() == 2 {
               let first = parts[0].parse::<u32>().ok()?;
               let second = parts[1].parse::<u32>().ok()?;
               Some((first, second))
           } else {
               None
           }
       }).unzip();
    first_numbers.sort_unstable();
    second_numbers.sort_unstable();
    let sum = first_numbers.iter()
        .zip(second_numbers.iter())
        .map(|(a, b)| {
            if a > b {
                a - b
            } else {
                b - a
            }
        }).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_numbers, second_numbers): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let first = parts[0].parse::<u32>().ok()?;
                let second = parts[1].parse::<u32>().ok()?;
                Some((first, second))
            } else {
                None
            }
        }).unzip();

    let mut occurrence_map: HashMap<u32, u32> = HashMap::new();
    second_numbers.iter().for_each(|item| {
        let value = occurrence_map.entry(*item).or_insert(0);
        *value += 1;
    });
    let result: u32 = first_numbers.iter().map(|item| {
        item * occurrence_map.get(item).copied().unwrap_or(0)
    }).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
