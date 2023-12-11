use std::collections::HashMap;
use regex::Regex;

pub fn process(input : &str) -> String {

    let patten: Vec<_> = input.lines().nth(0).unwrap().chars().collect();
    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();

    let mut map = HashMap::new();

    for line in input.lines(){

        if re.is_match(line){
            for (_ , [k , l , r] ) in re.captures_iter(line).map(|x| x.extract()){
                let _ = &map.insert(k, (l,r));
            }
        }
    }

    // find all that end in A
    let mut current_nodes = vec![];
    for key in map.keys(){
        if key.ends_with('A'){
            current_nodes.push(*key);
        }
    }

    let mut distances : Vec<i64> = vec![];

    for node in current_nodes.iter_mut(){

        let mut pattern_index = 0;
        let mut steps = 0;

        loop {

            if node.ends_with('Z'){
                break
            }

            if &pattern_index == &patten.len(){
                pattern_index = 0;
            }

                let (l ,r)  = *map.get(node).unwrap();

                let p = patten[pattern_index];

                if p == 'L'{
                    *node = l;
                }
                else if  p == 'R'{
                    *node = r;
                }
                else { panic!() }

            steps += 1;
            pattern_index +=1;
        }

        distances.push(steps);
    }

    // find lcm of distances
    let l = lcm_vec(distances);

    return format!("{}", l);
}

fn lcm_vec(input: Vec<i64>) -> i64
{
    let mut l = input[0];

    for value in input.iter().skip(1) {
        l = lcm(l, *value);
    }

    l
}

pub fn lcm(a: i64, b: i64) -> i64
   {
    let gcf = gcd(a, b);

    a * b / gcf
}

pub fn gcd(a: i64, b: i64) -> i64
{
    if a == 0 {
        return b;
    }

    gcd(b % a, a)
}