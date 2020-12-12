
fn calculate_iteration_part2(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut newgrid = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        let mut newrow = Vec::new();

        for (j, col) in row.iter().enumerate() {
            let mut occupied_around = 0;

            for xindex in (j+1)..row.len() {
                if grid[i][xindex] == '#' {
                    occupied_around += 1;
                    break;
                } else if grid[i][xindex] == 'L' {
                    break;
                }
            }

            for xindex in (0..((j as i64))).rev() {
                if grid[i][xindex as usize] == '#' {
                    occupied_around += 1;
                    break;
                } else if grid[i][xindex as usize] == 'L' {
                    break;
                }
            }

            for yindex in (i+1)..grid.len() {
                if grid[yindex][j] == '#' {
                    occupied_around += 1;
                    break;
                } else if grid[yindex][j] == 'L' {
                    break;
                }
            }

            for yindex in (0..((i as i64))).rev() {
                if grid[yindex as usize][j] == '#' {
                    occupied_around += 1;
                    break;
                } else if grid[yindex as usize][j] == 'L' {
                    break;
                }
            }

            let mut yindex = i as i64 + 1;
            let mut xindex = j as i64 + 1;
            while xindex >= 0 && yindex >= 0 && xindex < row.len() as i64 && yindex < grid.len() as i64{
                if grid[yindex as usize][xindex as usize] == '#' {
                    occupied_around += 1;
                    break;
                } else if grid[yindex as usize][xindex as usize] == 'L' {
                    break;
                }
           

                yindex += 1;
                xindex += 1;
            }

            let mut yindex = i as i64 - 1;
            let mut xindex = j as i64 + 1;
            while xindex >= 0 && yindex >= 0 && xindex < row.len() as i64 && yindex < grid.len() as i64{
                if grid[yindex as usize][xindex as usize] == '#' {
                    occupied_around += 1;
                    break;
                } else if grid[yindex as usize][xindex as usize] == 'L' {
                    break
                }
           

                yindex -= 1;
                xindex += 1;
            }

            let mut yindex = i as i64 - 1;
            let mut xindex = j as i64 - 1;
            while xindex >= 0 && yindex >= 0 && xindex < row.len() as i64 && yindex < grid.len() as i64{
                if grid[yindex as usize][xindex as usize] == '#' {
                    occupied_around += 1;
                    break;
                } else if grid[yindex as usize][xindex as usize] == 'L' {
                    break
                }
           

                yindex -= 1;
                xindex -= 1;
            }

            let mut yindex = i as i64 + 1;
            let mut xindex = j as i64 - 1;
            while xindex >= 0 && yindex >= 0 && xindex < row.len() as i64 && yindex < grid.len() as i64{
                if grid[yindex as usize][xindex as usize] == '#' {
                    occupied_around += 1;
                    break;
                } else if grid[yindex as usize][xindex as usize] == 'L'  {
                    break
                }
           

                yindex += 1;
                xindex -= 1;
            }

            if *col == 'L' && occupied_around == 0 {
               newrow.push('#') 
            } else if *col == '#' && occupied_around >= 5 {
                newrow.push('L')
            } else {
                newrow.push(*col)
            }
        }

        newgrid.push(newrow)
    }

    newgrid
}

fn calculate_iteration_part1(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut newgrid = Vec::new();
    
    for (i, row) in grid.iter().enumerate() {
        let mut newrow = Vec::new();

        for (j, col) in row.iter().enumerate() {
            let mut occupied_around = 0;

            if i > 0 {
                if j > 0 {
                    occupied_around += if grid[i - 1][j - 1] == '#' { 1 } else {0};
                }
                occupied_around += if grid[i - 1][j] == '#' { 1 } else {0};

                if j < row.len() - 1 {
                    occupied_around += if grid[i - 1][j + 1] == '#' { 1 } else {0};
                }
            }

            if i < grid.len() - 1 {
                if j > 0 {
                    occupied_around += if grid[i + 1][j - 1] == '#' { 1 } else {0};
                }
                occupied_around += if grid[i + 1][j] == '#' { 1 } else {0};

                if j < row.len() - 1 {
                    occupied_around += if grid[i + 1][j + 1] == '#' { 1 } else {0};
                }
            }

            if j > 0 {
                occupied_around += if grid[i][j - 1] == '#' { 1 } else {0};
            }

            if j < row.len() - 1 {
                occupied_around += if grid[i][j + 1] == '#' { 1 } else {0};
            }

            if *col == 'L' && occupied_around == 0 {
               newrow.push('#') 
            } else if *col == '#' && occupied_around >= 4 {
                newrow.push('L')
            } else {
                newrow.push(*col)
            }
        }

        newgrid.push(newrow)
    }

    newgrid
}



fn part1(inp: &str) -> Result<i64, ()> {
    let mut grid = Vec::new();
    
    for line in inp.trim().lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut row = Vec::new();

        for c in line.trim().chars() {
            row.push(c); 
        }

        grid.push(row);
    }


    loop {
        let prevgrid = grid.clone();

        grid = calculate_iteration_part1(grid);

        if grid == prevgrid {
            break
        }
    }

    let filled = grid.iter().flatten().filter(|i| **i == '#').count();



    Ok(filled as i64)
}


fn part2(inp: &str) -> Result<i64, ()> {
    let mut grid = Vec::new();
    
    for line in inp.trim().lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut row = Vec::new();

        for c in line.trim().chars() {
            row.push(c); 
        }

        grid.push(row);
    }


    loop {
        let prevgrid = grid.clone();

        grid = calculate_iteration_part2(grid);

        if grid == prevgrid {
            break
        }
    }

    let filled = grid.iter().flatten().filter(|i| **i == '#').count();



    Ok(filled as i64)
}



pub fn main() {  
    println!("day 11 part 1: {:?}", part1(include_str!("input1")));
    println!("day 11 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(37, part1("
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
        ").unwrap())
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(26, part2("
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
        ").unwrap())
    }


    #[test]
    fn test_part1() {
        assert_eq!(2418, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2144, part2(include_str!("input1")).unwrap());
    }
}



