
fn part1(inp: &str) -> Result<i64, ()> {
    let mut count = 0;

    for i in inp.lines() {
        let splits = i.split(&['-', ':', ' '][..]);

        if let &[from, to, letter, _, pw] = splits.collect::<Vec<_>>().as_slice() {
            let num = pw.matches(&letter).count();

            if num >= from.parse().unwrap() && num <= to.parse().unwrap() {
                count += 1;
            }
        }
        
    }

    Ok(count)
}

fn part2(inp: &str) -> Result<i64, ()> {
    let mut count = 0;

    for i in inp.lines() {
        let splits = i.split(&['-', ':', ' '][..]);

        if let &[from, to, letter, _, pw] = splits.collect::<Vec<_>>().as_slice() {
            let first = pw.chars().nth(from.parse::<usize>().unwrap() - 1).unwrap();
            let second = pw.chars().nth(to.parse::<usize>().unwrap() - 1).unwrap();

            if (first.to_string() == letter) != (second.to_string() == letter) {
                count += 1;
            }
        }
    }

    Ok(count)
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
        assert_eq!(536, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(558, part2(include_str!("input1")).unwrap());
    }
}



