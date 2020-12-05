#[derive(PartialEq, Debug)]
enum Possible {
    Open,
    Tree
}


fn part1(inp: &str) -> Result<i64, ()> {

    let mut forest = Vec::new();

    for (index, line) in inp.lines().enumerate() {
        forest.push(vec![]);
        for c in line.chars() {
            if c == '.' {
               forest[index].push(Possible::Open); 
            } else if c == '#' {
               forest[index].push(Possible::Tree); 
            }
        }
    }

    let forest: Vec<_> = forest.into_iter().filter(|i| i.len() != 0).collect();
    let trees = check_for_slope(&forest, 3, 1);

    Ok(trees)
}


fn part2(inp: &str) -> Result<i64, ()> {
    let mut forest = Vec::new();

    for (index, line) in inp.lines().enumerate() {
        forest.push(vec![]);
        for c in line.chars() {
            if c == '.' {
               forest[index].push(Possible::Open); 
            } else if c == '#' {
               forest[index].push(Possible::Tree); 
            }
        }
    }

    let forest: Vec<_> = forest.into_iter().filter(|i| i.len() != 0).collect();
    
    let mut trees = check_for_slope(&forest, 1, 1);
    trees *= check_for_slope(&forest, 3, 1);
    trees *= check_for_slope(&forest, 5, 1);
    trees *= check_for_slope(&forest, 7, 1);
    trees *= check_for_slope(&forest, 1, 2);

    Ok(trees)
}



fn check_for_slope(forest: &Vec<Vec<Possible>>, x: u64, y: u64) -> i64 {
    let mut trees = 0;

    let mut j = 0;
    for i in ((1 + (y as usize - 1))..forest.len()).step_by(y as usize){
       j += x as usize;
       j %= forest[0].len();

       if forest[i as usize][j as usize] == Possible::Tree {
           trees += 1;
       } 
    }

    return trees;
}

pub fn main() {  
    println!("day 3 part 1: {:?}", part1(include_str!("input1")));
    println!("day 3 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let inp = "
..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#
            ";

        assert_eq!(7, part1(inp).unwrap());
    }

    #[test]
    fn example_part2() {
        let inp = "
..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#
            ";

        assert_eq!(336, part2(inp).unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(151, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(7540141059, part2(include_str!("input1")).unwrap());
    }
}



