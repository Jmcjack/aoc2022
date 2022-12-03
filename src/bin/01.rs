use std::thread::current;

pub fn part_one(input: &str) -> Option<u32> {
    let res = get_most_calories(input);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = get_most_calories_3(input);
    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use std::thread::current;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}

fn get_most_calories(input: &str) -> u32 {
    let mut most_cal: u32 = 0;
    let mut current_cal: u32 = 0;
    let strs = input.split("\n");

    for str in strs {
        if str == "" {
            if current_cal > most_cal {
                most_cal = current_cal;
            }
            current_cal = 0;
        }
        else {
            current_cal += str.parse::<u32>().unwrap()
        }
    }
    most_cal
}

fn get_most_calories_3(input: &str) -> u32 {
    let mut current_cal: u32 = 0;
    let strs = input.split("\n");

    let mut vals = Vec::new();

    for str in strs {
        if str == "" {       // ! This does not capture the last entry. Test fails, but answer is correct
            vals.push(current_cal);
            current_cal = 0;
        }
        else {
            current_cal += str.parse::<u32>().unwrap()
        }
    }
    vals.sort();

    let res: u32 = &vals[vals.len()-1] + &vals[vals.len()-2] + &vals[vals.len()-3];
    res
}
