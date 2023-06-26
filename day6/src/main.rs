use std::{fs, path::Path};

fn main() {
    let input = fs::read_to_string(Path::new("data")).expect("no such file");
    println!("The start-of-packet marker is  : {}", find_first_start_of_packet_marker(&input));
    println!("The start-of-message marker is : {}", find_marker(&input, 14));
}

fn find_first_start_of_packet_marker(s : &str,) -> usize{
    
    find_marker(s , 4) 

}

fn find_marker(s: &str, marker_length : usize) -> usize {
    let code : Vec<char>= s.chars().collect();
    for i in marker_length .. code.len(){
                let possible_marker = &code[(i-marker_length)  .. i];
                if !has_repeating_characters(possible_marker){
                    return i;
                }
            
    
    }
    code.len() + 1
}

fn has_repeating_characters(possible_marker: &[char]) -> bool {
    for i in 0..possible_marker.len()-1{
        let c = possible_marker[i];
        for j in i+1..possible_marker.len() {
            if c == possible_marker[j]{
                return true;
            }
        }
    }
    false
}



#[cfg(test)]
mod tests{
    use crate::{find_first_start_of_packet_marker, find_marker};

    
    #[test]
    fn test_day_6(){
        let test1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let test2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let test3 = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(find_first_start_of_packet_marker(test1), 7);
        assert_eq!(find_first_start_of_packet_marker(test2), 5);
        assert_eq!(find_first_start_of_packet_marker(test3), 6);

        let test1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let test2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let test3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(find_marker(test1, 14), 19);
        assert_eq!(find_marker(test2, 14), 23);
        assert_eq!(find_marker(test3, 14), 29);
    }
}