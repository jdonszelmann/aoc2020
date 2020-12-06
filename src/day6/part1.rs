use std::collections::HashSet;
use itertools::Itertools;

fn part1(inp: &str) -> Result<i64, ()> {
    let mut total = 0;

    for group in inp.split("\n\n") {
        let mut set = HashSet::new();

        for question in group.chars() {
            if question.is_ascii_alphabetic() {
                set.insert(question);
            }
        }
        
        total += set.len();
        println!("{}", set.len());

    }

    Ok(total as i64)
}



fn part2(inp: &str) -> Result<i64, ()> {
        
    let result = inp.split("\n\n")
        .map(|group| 
            group.split("\n")
            .filter(|i| !i.trim().is_empty())
            .map(|people|
                people.chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<HashSet<_>>()
            ).fold1(|acc, val| {
                acc.intersection(&val).cloned().collect()
            })
            .map(|i| i.len() as i64)
            .unwrap_or(0)
        )
        .sum();


    Ok(result)
}



pub fn main() {  
    println!("day 6 part 1: {:?}", part1(include_str!("input1")));
    println!("day 6 part 2: {:?}", part2(include_str!("input1")));
}
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_example_part1() {
        assert_eq!(11, part1("abc

a
b
c

ab
ac

a
a
a
a

b").unwrap())
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(6, part2("abc

a
b
c

ab
ac

a
a
a
a

b").unwrap())
    }


    #[test]
    fn test_custom1_part2() {
        assert_eq!(1, part2("
aa
a
aaa
        ").unwrap())
    } 


    #[test]
    fn test_custom2_part2() {
        assert_eq!(0, part2("
ab
ac
cb
        ").unwrap())
    }

    #[test]
    fn test_part1() {
        assert_eq!(6382, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3197, part2(include_str!("input1")).unwrap());
    }
}



