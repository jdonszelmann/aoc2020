

fn parse_seat(v: &str) -> (i64, i64) {
    
    let (row, col) = v.chars().fold(((0i64, 128i64), (0i64, 8i64)), |(row, col), nxt| {
        let avgrow = (row.0 - row.1).abs() / 2;
        let avgcol = (col.0 - col.1).abs() / 2;

        match nxt {
            'B' => {
                ((row.0 + avgrow, row.1), col)
            }
            'F' => {
                ((row.0, row.1 - avgrow), col)
            }
            'R' => {
                (row, (col.0 + avgcol, col.1))
            }
            'L' => {
                (row, (col.0, col.1 - avgcol))
            }

            _ => (row, col)
        }
    });


    return (row.0, col.0);
}

fn part1(inp: &str) -> Result<i64, ()> {
    let max = inp.lines().map(parse_seat).map(|(r, c)| r*8 + c ).max().unwrap(); 
    Ok(max)
}


fn empty(arr: &[i64; 8]) -> bool {
    arr.iter().sum::<i64>() == 0
}

fn part2(inp: &str) -> Result<i64, ()> {
    let mut rows = [[0; 8]; 128];
    
    for i in inp.lines() {
        let (r, c) = parse_seat(i);

        rows[r as usize][c as usize] = 1;
    }

     for i in 0..128 {
        println!("{:?}", rows[i]);

        for j in 0..8 {
            if rows[i][j] == 0 && !empty(&rows[i]) && i < rows.len() && !empty(&rows[i + 1]) && i > 0 && !empty(&rows[i - 1]){
                return Ok((i * 8 + j) as i64)
            }
        }
    }

    Err(())
}



pub fn main() {  
    println!("day 5 part 1: {:?}", part1(include_str!("input1")));
    println!("day 5 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let inp = "
FBFBBFFRLR
            ";

        let (r, c) = parse_seat(inp);

        assert_eq!(357, r * 8 + c);
    }

    #[test]
    fn test_part1() {
        assert_eq!(933, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(711, part2(include_str!("input1")).unwrap());
    }
}



