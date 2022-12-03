pub fn part_one(input: &str) -> Option<u32> {
    let score: u32 = get_all_scores(&input);
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score: u32 = get_decrypt_score(&input);
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));

        get_all_scores(&input);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

fn choice_score(a: &str) -> u32
{
    let mut score: u32 = 0;
    if a == "A" || a =="X" {
        score = 1
    }
    else if a == "B" || a == "Y" {
        score =  2
    }
    else if a == "C" || a == "Z" {
        score =  3
    } else {
        panic!("Invalid character");
    }

    score
}

fn outcome_score(a: &str, b: &str) -> u32
{
    let mut score : u32 = 0;
    if a == "A" {
        if b == "Y" {
            score = 6;
        } 
        else if b == "X" {
            score = 3;
        }
    } else if a == "B" 
    {
        if b == "Z" {
            score = 6;
        } 
        else if b == "Y" {
            score = 3;
        }
    } else if a == "C" 
    {
        if b == "X" {
            score = 6;
        } 
        else if b == "Z" {
            score = 3;
        }
    }

    score
}

fn get_all_scores(input: &str) -> u32 {
    let strs = input.split("\n");

    let mut score: u32 = 0;

    for str in strs {
        if str == "" {
            continue;
        }
        let symbs: Vec<&str> = str
            .split_whitespace()
            .collect();
        score += pair_score(symbs[0], symbs[1]);
    }
    score
}

fn pair_score(a: &str, b: &str) -> u32
{
    let mut score: u32 = outcome_score(a, b) + choice_score(b);
    score
}

fn get_decrypt_score(input: &str) -> u32 {
    let strs = input.split("\n");

    let mut score: u32 = 0;

    for str in strs {
        if str == "" {
            continue;
        }
        let symbs: Vec<&str> = str
            .split_whitespace()
            .collect();
        
        let mut new_symb = "meow";
        if symbs[0] == "A" {
            if symbs[1] == "X" {
                new_symb = "Z";
            }
            else if symbs[1] == "Y" {
                new_symb = "X";
            }
            if symbs[1] == "Z" {
                new_symb = "Y";
            }
        }
        else if symbs[0] == "B" {
            if symbs[1] == "X" {
                new_symb = "X";
            }
            else if symbs[1] == "Y" {
                new_symb = "Y";
            }
            if symbs[1] == "Z" {
                new_symb = "Z";
            }
        }
        else if symbs[0] == "C" {
            if symbs[1] == "X" {
                new_symb = "Y";
            }
            else if symbs[1] == "Y" {
                new_symb = "Z";
            }
            if symbs[1] == "Z" {
                new_symb = "X";
            }
        }
        score += pair_score(symbs[0], new_symb);
    }
    score
}
