use std::collections::HashMap;
use std::iter;

pub fn part_one(input: &str) -> Option<u32> {
    let strs = input.split("\n");

    let mut priority_sum: u32 = 0;

    for stro in strs {
        if stro == "" {      // Don't really need this
            continue;
        }
        let len = stro.chars().count();
        let a: &str = &stro[..len/2];
        let b: &str = &stro[len/2..];
        let similar_char: char = get_similar_char(a, b);

        let char_int = similar_char as u32;

        if similar_char.is_lowercase() {
            priority_sum += char_int - 96;
        }
        else {
            priority_sum += char_int - 38;    
        }
    }
    Some(priority_sum)
}

fn get_similar_char(a: &str, b: &str) -> char {

    let mut common_car ='1'; 
    for c in a.chars() {
        for cc in b.chars() {
            if c == cc {
                common_car = c;
                break;
            }
        }
    }
    common_car
}

fn get_similar_char_lines(a: &str, b: &str, see: &str) -> char {

    let mut common_car ='1'; 
    let mut char_vec: Vec<char> = Vec::new();
    for c in a.chars() {
        for cc in b.chars() {
            if c == cc {
                char_vec.push(c);
            }
        }
    }

    for c in char_vec {
        for cc in see.chars() {
            if c == cc{
                common_car = c;
                break;
            }
        }
    }
    common_car
}

pub fn part_two(input: &str) -> Option<u32> {
    let strs = input.split("\n");
    let strs_c: Vec<&str> = strs.collect();

    let mut priority_sum: u32 = 0;

    let mut i = 0;
    while i < strs_c.len()-1 {
        let s1 = strs_c[i];
        let s2 = strs_c[i+1];
        let s3 = strs_c[i+2];
        let similar_char: char = get_similar_char_lines(s1,s2,s3);
        let char_int = similar_char as u32;

        if similar_char.is_lowercase() {
            priority_sum += char_int - 96;
        }
        else {
            priority_sum += char_int - 38;    
        }
        i += 3;
    }
    Some(priority_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
