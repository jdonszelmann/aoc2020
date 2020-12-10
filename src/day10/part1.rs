use std::collections::HashMap;

fn part1(inp: &str) -> Result<i64, ()> {
    let mut nums = inp.lines().filter(|i| !i.trim().is_empty()).map(|i| i.parse().unwrap()).collect::<Vec<i64>>();

    nums.sort();
        

    let chain = find(&nums, 0, *nums.iter().max().unwrap()).unwrap();
    
    let (ones, threes) = differences(&chain);

    return Ok(ones * threes)
}


fn differences(chain: &[i64]) -> (i64, i64) {
    let mut prev = chain[0];

    let mut ones = 0;
    let mut threes = 0;

    for i in chain[1..].iter() {
        if i - prev == 1 {
            ones += 1;
        }
        
        if i - prev == 3 {
            threes += 1;
        }

        prev = *i;
    }

    (ones, threes)
}


fn find(nums: &[i64], curr: i64, max: i64) -> Result<Vec<i64>, ()> {
    let mut chain = vec![curr];

    let mut added = 0;

    if curr == max {
        return Ok(vec![max, max + 3]);
    }

    for i in nums {

        if i - curr > 0 && i - curr <= 3 && added != 1 {
            if let Ok(newchain) = find(nums, *i, max) {
                chain.extend(newchain.into_iter());
                added += 1;
                break;
            }
        }
    }

    if added == 0{
        return Err(())
    }

    Ok(chain)
}

fn find2(nums: &[i64], curr: i64, max: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    let mut num = 0;

    if let Some(i) = cache.get(&curr) {
        return *i;
    }

    if curr == max {
        return 1;
    }

    for i in nums {
        if i - curr > 0 && i - curr <= 3 {
            num += find2(nums, *i, max, cache);
        }
    }

    cache.insert(curr, num);

    num
}

fn part2(inp: &str) -> Result<i64, ()> {
    let nums = inp.lines().filter(|i| !i.trim().is_empty()).map(|i| i.parse().unwrap()).collect::<Vec<i64>>();

    let mut cache = HashMap::new();
    Ok(find2(&nums, 0, *nums.iter().max().unwrap(), &mut cache))
}



pub fn main() {  
    println!("day 10 part 1: {:?}", part1(include_str!("input1")));
    println!("day 10 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(35, part1("
16
10
15
5
1
11
7
19
6
12
4
        ").unwrap())
    }

#[test]
    fn test_example_part2() {
        assert_eq!(8, part2("
16
10
15
5
1
11
7
19
6
12
4
        ").unwrap())
    }


    #[test]
    fn test_part1() {
        assert_eq!(1820, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3454189699072, part2(include_str!("input1")).unwrap());
    }
}



