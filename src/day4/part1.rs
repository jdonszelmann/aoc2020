use std::collections::HashMap;

fn all_fields(map: &HashMap<&str, &str>) -> bool { 
    for i in &[ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !map.contains_key(i) {
            return false;
        }
    }

    return true;
}

fn part1(inp: &str) -> Result<i64, ()> {
   
    let mut valid = 0;

    for line in inp.split("\n\n") {        
        let mut pp = HashMap::default();

        for kv in line.split(&[' ', '\n'][..]) {
            if let [key, value, ..] = kv.split(':').collect::<Vec<_>>().as_slice() {
                pp.insert(*key, *value);
            } 
        }

        if all_fields(&pp) {
            valid += 1;
        }
    }

    Ok(valid)
}

fn is_int_in_bounds(val: &str, lower: i64, upper: i64) -> bool {
    if let Ok(i) = val.parse::<i64>() {
        i >= lower && i <= upper
    } else {
        false
    }
}


fn parse_height(v: &str) -> bool {
    match v.split_at(v.len()-2) {
        (v, "cm") => is_int_in_bounds(v, 150, 193),
        (v, "in") => is_int_in_bounds(v, 59, 76),
        _ => false,
    }
}

fn parse_color(v: &str) -> bool {
    v.chars().nth(0).unwrap() == '#' && i64::from_str_radix(&v[1..], 16).is_ok()
}

fn validate(pp: &HashMap<&str, &str>) -> bool {
    if !all_fields(pp) {
        return false;
    }

    for (k, v) in pp {
        match *k {
            "byr" if is_int_in_bounds(v, 1920, 2002) => true,
            "iyr" if is_int_in_bounds(v, 2010, 2020) => true,
            "eyr" if is_int_in_bounds(v, 2020, 2030) => true,
            "hgt" if parse_height(v) => true,
            "hcl" if parse_color(v) => true,
            "ecl" if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v) =>true,
            "pid" if v.len() == 9 => true,
            "cid" => true, 
            _ => return false,
        };
    }

    return true;
}

fn part2(inp: &str) -> Result<i64, ()> {

   
    let mut valid = 0;

    for line in inp.split("\n\n") {        
        let mut pp = HashMap::default();

        for kv in line.split(&[' ', '\n'][..]) {
            if let [key, value, ..] = kv.split(':').collect::<Vec<_>>().as_slice() {
                pp.insert(*key, *value);
            } 
        }

        if validate(&pp) {
            valid += 1;
        }
    }

    Ok(valid)
}



pub fn main() {  
    println!("day 4 part 1: {:?}", part1(include_str!("input1")));
    println!("day 4 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let inp = "
eyr: 1972
        ";

        assert_eq!(0, part1(inp).unwrap());
    }

    #[test]
    fn example_part2() {
        let inp = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
            ";

        assert_eq!(4, part2(inp).unwrap());
    }



    #[test]
    fn test_part1() {
        assert_eq!(239, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(188, part2(include_str!("input1")).unwrap());
    }
}



