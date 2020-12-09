
fn part1(inp: &str) -> Result<i64, ()> {
    Ok(0)
}

fn part2(inp: &str) -> Result<i64, ()> {
    Ok(0)
}


trait Cons {

}

macro_rules! input {
    ($($i:tt)*) => {

    };
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



