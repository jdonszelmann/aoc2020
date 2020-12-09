
fn construct_sum(nums: &[i64], res: i64) -> bool {
    for i in nums {
        for j in nums {
            if i + j == res {
                return true;
            }
        }
    }

    return false;
}

fn get_numbers(inp: &str) -> Vec<i64> {
    inp.lines().filter(|i| !i.trim().is_empty()).map(|i| i.parse().unwrap()).collect()
}

fn part1(inp: &str, preamble_size: usize) -> Result<i64, ()> {
    let numbers = get_numbers(inp);

    for preamble in numbers.windows(preamble_size + 1) {
        if !construct_sum(&preamble[0..preamble_size], preamble[preamble_size]) {
            return Ok(preamble[preamble_size])
        }
    }

    // All OK, but one *should* have be an error
    Err(())
}


fn part2(inp: &str, preamble_size: usize) -> Result<i64, ()> {
    let numbers = get_numbers(inp);
    let inv_num = part1(inp, preamble_size).unwrap();

    for setlen in 2..numbers.len() {
        for set in numbers.windows(setlen) {
            if set.iter().sum::<i64>() == inv_num {
                let min = set.iter().min().unwrap();
                let max = set.iter().max().unwrap();

                return Ok(min + max)
            }
        }
    }

    
    Err(())
}



pub fn main() {  
    println!("day 9 part 1: {:?}", part1(include_str!("input1"), 25));
    println!("day 9 part 2: {:?}", part2(include_str!("input1"), 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(127, part1("
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

        ", 5).unwrap())
    }

#[test]
    fn test_example_part2() {
        assert_eq!(62, part2("
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
        ", 5).unwrap())
    }


    #[test]
    fn test_part1() {
        assert_eq!(50047984, part1(include_str!("input1"), 25).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(5407707, part2(include_str!("input1"), 25).unwrap());
    }
}



