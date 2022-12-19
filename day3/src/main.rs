use std::{fs, path::Path, collections::{HashMap, HashSet}};

fn main() {
    let input = fs::read_to_string(Path::new("input")).unwrap();
    let mut items = parse_and_find(&input);
    let mut res = count_item_priority(&items);
    println!("1 : {}", &res);
    items = parse_and_find_groups(&input);
    res = count_item_priority(&items);
    println!("2 : {}", &res);
}

fn parse_and_find(input: &str) -> Vec<char> {
    let mut items = Vec::new();
    for line in input.lines().into_iter() {
        if !line.is_empty() {
            let left = &line[0..(line.len() / 2)];
            let right = &line[(line.len() / 2) ..line.len()];
            assert_eq!(left.len(), right.len());
            let item = find_item(left, right).unwrap();
            items.push(item);
        }
    }
    items
}

fn find_item(left: &str, right: &str) -> Option<char> {
    for x in left.chars().into_iter(){
        for y in right.chars().into_iter(){
            if x == y {return Some(x);}
        }
    }
    None
}

fn parse_and_find_groups(input: &str) -> Vec<char> {
    let groups = group_to_three(input);
    let mut items = Vec::new();
    for group in groups.into_iter(){
        assert_eq!(group.len(), 3);
        let set0 = to_map(&group[0]);
        let set1 = to_map(&group[1]);
        let set2 = to_map(&group[2]);
        let x =determine_common_item(&set0, &set1, &set2);
        match x {
            Some(item) => items.push(item),
            None => {},
        }
    }
    items
}

fn determine_common_item(set0 : &HashSet<char>,set1 : &HashSet<char>,set2 : &HashSet<char>,)-> Option<char>{
    for c in set0.into_iter(){
        if set1.contains(&c) && set2.contains(&c){
            return Some(c.clone());
        }
    }
    None
}

fn to_map(group: &str) -> HashSet<char> {
    let mut map = HashSet::new();
    for c in group.chars(){
        map.insert(c);
    }
    map
}

fn group_to_three(input :&str) -> Vec<Vec<&str>> {
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for (i, line) in input.lines().into_iter().enumerate(){
        group.push(line);
        if ((i as i32) - 2 ) % 3 == 0{
            groups.push(group);
            group = Vec::new();
        }
    }
    if ! group.is_empty() {
        groups.push(group);
    }
    groups
}


fn count_item_priority(items: &[char]) -> i32 {
    items.into_iter().map(|c| determine_priority(&c)).sum()
}

fn determine_priority(c : &char) -> i32 {
    if c.is_uppercase() {
        return (*c as u8 - 64 + 26) as i32;
    } else {
        return (*c as u8 - 96) as i32;
    }
}


#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{parse_and_find, count_item_priority, parse_and_find_groups};

    #[test]
    fn test_day_3() {
        let input = fs::read_to_string(Path::new("test")).unwrap();
        let mut  items = parse_and_find(&input);
        println!("{:?}", &items);
        let mut res = count_item_priority(&items);
        let c = 'a';
        assert_eq!(c as u8, 97);
        println!("{} : {}", &c, &(c as u8));
        let c = 'A';
        assert_eq!(c as u8, 65);
        println!("{} : {}", &c, &(c as u8));

        assert_eq!(res, 157);
        items = parse_and_find_groups(&input);
        println!("{:?}", items);
        res = count_item_priority(&items);
        assert_eq!(res, 70);
    }

}
