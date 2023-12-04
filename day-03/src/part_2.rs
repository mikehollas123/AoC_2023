use regex::Regex;
use std::collections::HashMap;

pub fn process(input : &str) -> String {

    let mut gear_map:HashMap<(usize,usize) , Vec<u32>> = HashMap::new();
    let lines : Vec<&str> = input.lines().collect();
    let re = Regex::new(r"[0-9]+").unwrap();

    let mut sum = 0;

    for i in 0..lines.len(){
        for caps in re.captures_iter(&lines[i]) {
            for cap in caps.iter(){

                let loc = cap.unwrap().range();
                let num = cap.unwrap().as_str().parse::<u32>().unwrap();

                if loc.start > 0{
                    if is_gear(&lines[i].chars().nth(loc.start - 1).unwrap()) {
                        add_gear((i,loc.start - 1),num,&mut gear_map);
                    }
                }
                if loc.end < lines.len() -1{
                    if is_gear(&lines[i].chars().nth(loc.end).unwrap()){
                        add_gear((i,loc.end ),num,&mut gear_map);
                    }
                }
                // a little awkward
                let s =  if loc.start == 0 { 0} else {loc.start - 1};
                let e =  if loc.end > lines.len() -1 { loc.end - 1 } else {loc.end}; //end is NOT inclusive

                for j in s..=e {
                    if i > 0 {
                        if is_gear(&lines[i - 1].chars().nth(j).unwrap()) {
                            add_gear((i-1 ,j),num,&mut gear_map);
                        }
                    }
                    if i < lines.len() -1 {
                        if is_gear(&lines[i + 1].chars().nth(j).unwrap()) {
                            add_gear((i+1 ,j),num,&mut gear_map);
                        }
                    }
                }
            }
        }
    }

    for (_gear, nums) in gear_map{
        if nums.len() ==2{
            let ratio = nums[0] * nums[1];
            sum += ratio;
        }
    }

    return format!("{}",sum);
}

fn add_gear(gear_coordinates : (usize,usize), num : u32, map : &mut HashMap<(usize,usize) , Vec<u32>>){
    if map.contains_key(&gear_coordinates){
        map.entry(gear_coordinates).and_modify(|x| x.push(num));
    }
    else{
        map.insert(gear_coordinates, vec![num]);
    }
}

fn is_gear(c :&char) -> bool{
    c == &'*'
}