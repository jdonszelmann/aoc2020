use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

#[derive(Debug, Default)]
struct Bag {
    colour: String,

    contents: HashMap<String, i64>,
}

fn parse_bagline(inp: &str) -> Bag {
    let mut bag = Bag::default(); 

    if let [left, right, ..] = inp.split("bags contain").collect::<Vec<_>>().as_slice() {
        bag.colour = left.trim().to_string(); 
       
        if right.trim() != "no other bags." {
            for i in right.trim().split(",") {
                if let [number, rest@.., last] = i.trim().split(" ").collect::<Vec<_>>().as_slice(){
                    let res = rest.iter().join(" ");
                    
                    bag.contents.insert(res, number.parse().unwrap());
                }
            }
        }
    } else {
        panic!("invalid format");
    }

    bag
}

fn has_gold(bag: &Bag, bags: &HashMap<String, Bag>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(bag);

    while let Some(i) = queue.pop_front() {
        for (bagtype, number) in &i.contents {
            if bagtype == "shiny gold" {
                return true;
            }

            if let Some(b) = bags.get(bagtype) {
                queue.push_back(b);
            } else {
                panic!("bagtype not found");
            }
        }
    }
    
    false
}


fn part1(inp: &str) -> Result<i64, ()> {
    let mut bags = HashMap::new();

    for i in inp.lines().filter(|i| !i.trim().is_empty()) {
        let bag = parse_bagline(i);

        bags.insert(bag.colour.clone(), bag);
    }

    let mut count = 0;
    for (name, val) in &bags {
        if has_gold(val, &bags) {
            count += 1;
        }
    }
    

    Ok(count)
}

fn count_bags(curr: &Bag, bags: &HashMap<String, Bag>, cache: &mut HashMap<String, i64>) -> i64 {
    if let Some(total) = cache.get(&curr.colour) {
        return *total;
    }

    let mut total = 1;

    for (colour, number) in &bags.get(&curr.colour).unwrap().contents {
        total += count_bags(bags.get(colour).unwrap(), bags, cache) * number;
    }

    cache.insert(curr.colour.to_string(), total);

    total
}


fn part2(inp: &str) -> Result<i64, ()> {
    let mut bags = HashMap::new();

    for i in inp.lines().filter(|i| !i.trim().is_empty()) {
        let bag = parse_bagline(i);

        bags.insert(bag.colour.clone(), bag);
    }

    let mut cache = HashMap::new();  
    let total = count_bags(bags.get("shiny gold").unwrap(), &bags, &mut cache) - 1;

    Ok(total)
}



pub fn main() {  
    println!("day 7 part 1: {:?}", part1(include_str!("input1")));
    println!("day 7 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(4, part1("
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
        ").unwrap())
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(32, part2("
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
        ").unwrap())
    }

    #[test]
    fn test_part1() {
        assert_eq!(238, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(82930, part2(include_str!("input1")).unwrap());
    }
}



