
fn part1(inp: &str) -> Result<i64, ()> {
    let mut x = 0;
    let mut y = 0;
    let mut rot = 0;

    for i in inp.lines() {
        if i.trim().is_empty() {
            continue
        }

        let num = i[1..].parse::<i64>().unwrap();
        match i.chars().nth(0).unwrap()  {
            'N' => {
                y += num;
            },
            'S' => {
                y -= num;
            },
            'E' => {
                x += num;
            },
            'W' => {
                x -= num;
            },
            'L' => {
                rot = (rot - num).rem_euclid(360);
            }
,
            'R' => {
                rot = (rot + num).rem_euclid(360);
            }
            'F' => {
                match rot {
                    0 => {x += num},
                    90 => {y -= num},
                    180 => {x -= num},
                    270 => {y += num}
                    _ => unreachable!()
                }
            },
            
            _ => unreachable!()
        }

    }

    Ok(x.abs() + y.abs())
}


fn part2(inp: &str) -> Result<i64, ()> {
    let (sx, sy, _wx, _wy) = solve(inp);

    Ok(sx.abs() + sy.abs())
}

fn solve(inp: &str) -> (i64, i64, i64, i64) {
    let mut sx = 0;
    let mut sy = 0;

    let mut dwx = 10;
    let mut dwy = 1;


    for i in inp.lines() {
        if i.trim().is_empty() {
            continue
        }

        let num = i[1..].parse::<i64>().unwrap();
        match i.chars().nth(0).unwrap()  {
            'N' => dwy += num,
            'S' => dwy -= num,
            'E' => dwx += num,
            'W' => dwx -= num,
            'L' => match num {
                90 => {
                    let tmp = dwx;
                    dwx = -dwy;
                    dwy = tmp;
                },
                180 => {
                    dwx = -dwx;   
                    dwy = -dwy;
                }
                270 => {
                    let tmp = -dwx;
                    dwx = dwy;
                    dwy = tmp;
                }
                _ => () 
            }
            'R' => match num {
                90 => {
                    let tmp = -dwx;
                    dwx = dwy;
                    dwy = tmp;
                },
                180 => {
                    dwx = -dwx;
                    dwy = -dwy;
                }
                270 => {
                    let tmp = dwx;
                    dwx = -dwy;
                    dwy = tmp;
                }
                _ => ()
            }
            'F' => {
                sx += dwx * num;
                sy += dwy * num;
            },
            
            _ => unreachable!()
        }
    }

    (sx, sy, dwx, dwy)
}



pub fn main() {  
    println!("day 12 part 1: {:?}", part1(include_str!("input1")));
    println!("day 12 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(25, part1("
F10
N3
F7
R90
F11
        ").unwrap())
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(286, part2("
F10
N3
F7
R90
F11
        ").unwrap())
    }

    #[test]
    fn test_custom_1() {
        assert_eq!((0, 0, 1, -10), solve("
R90
        "))
    }

    #[test]
    fn test_custom_2() {
        assert_eq!((0, 0, -10, -1), solve("
R180
        "))
    }


    #[test]
    fn test_part1() {
        assert_eq!(796, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(39446, part2(include_str!("input1")).unwrap());
    }
}



