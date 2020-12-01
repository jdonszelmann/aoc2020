
fn part1(inp: &str) -> Result<i64, ()> {
    let mut nums: Vec<i64> = Vec::new();
    for i in inp.lines() {
        nums.push(i.parse().unwrap())
    }


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
    let mut nums: Vec<i64> = Vec::new();
    for i in inp.lines() {
        nums.push(i.parse().unwrap())
    }


    for i in &nums {
        for j in &nums {
            for k in &nums {
                if i + j + k == 2020 {
                    return Ok(i * j);
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





