
fn part1(inp: &str) -> Result<i64, ()> {
    let nums = inp.lines().map(str::parse).collect::<Result<Vec<i64>, _>>().unwrap();

    for i in &nums {
        for j in &nums {
            if i + j == 2020 {
                return Ok(i * j);
            }
        }
    }

    Err(())
}

fn part2(inp: &str) -> Result<i64, ()> {
    let nums = inp.lines().map(str::parse).collect::<Result<Vec<i64>, _>>().unwrap();

    for i in &nums {
        for j in &nums {
            for k in &nums {
                if i + j + k == 2020 {
                    return Ok(i * j * k);
                }
            }
        }
    }

    Err(())
}


pub fn main() {  
    println!("day 1 part 1: {:?}", part1(include_str!("input1")));
    println!("day 1 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(100419, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(265253940, part2(include_str!("input1")).unwrap());
    }
}



