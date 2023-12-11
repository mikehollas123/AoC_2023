use std::collections::HashMap;
use regex::Regex;

pub fn process(input : &str) -> String {

    let patten: Vec<_> = input.lines().nth(0).unwrap().chars().collect();
    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();

    let mut map = HashMap::new();

    for line in input.lines(){

        if re.is_match(line){
            for (_ , [k , l , r] ) in re.captures_iter(line).map(|x| x.extract()){
                let _ = &map.insert(k, (l,r));
            }
        }
    }

    let mut current_node = "PSA";

    let mut pattern_index = 0;

    let mut steps = 0;

    loop {

        if current_node == "BNZ"{
            break;
        }

        let (l ,r)  = *map.get(current_node).unwrap();

        if &pattern_index >= &patten.len(){
            pattern_index = 0;
        }

        let p = patten[pattern_index];

        if p == 'L'{
            current_node = l;
        }
        else if  p == 'R'{
            current_node = r;
        }
        else { panic!() }

        steps += 1;
        pattern_index +=1;
    }

    return format!("{}",steps);
}
