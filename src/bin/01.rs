use itertools::Itertools;

fn get_grouped_sum(input: &str) -> Vec<u32> {
        input.split("\n\n")
        .map(|group| group.lines().map(
                |line| line.parse::<u32>().unwrap_or(0)).sum())
        .sorted()
        .rev()
        .collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_grouped_sum(input)[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    let calories: Vec<u32> = get_grouped_sum(input);
    Some(calories[0..3].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
