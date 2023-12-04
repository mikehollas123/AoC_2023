use regex::Regex;

pub fn process(input : &str) -> String {

    // put all the lines into a vector
    let lines : Vec<&str> = input.lines().collect();

    // rege to find numbers
    let re = Regex::new(r"[0-9]+").unwrap();

    let mut sum = 0;

    for i in 0..lines.len(){
        //print!("line {}: ", i+1);
        for caps in re.captures_iter(&lines[i]) {
            for cap in caps.iter(){

                let mut has_symbol = false;
                let loc = cap.unwrap().range();
                let num = cap.unwrap().as_str().parse::<u32>().unwrap();

                // symbols to the left and right
                if loc.start > 0{
                    if is_symbol(&lines[i].chars().nth(loc.start - 1).unwrap()) {
                        has_symbol = true;
                    }
                }
                if loc.end < lines.len() -1{
                    if is_symbol(&lines[i].chars().nth(loc.end).unwrap()){
                        has_symbol = true;
                    }
                }

                if !has_symbol {
                    // a little awkward
                    let s =  if loc.start == 0 { 0} else {loc.start - 1};
                    let e =  if loc.end > lines.len() -1 { loc.end - 1 } else {loc.end}; //end is NOT inclusive

                    for j in s..=e {
                        if i > 0 {
                            if is_symbol(&lines[i - 1].chars().nth(j).unwrap()) {
                                has_symbol = true;
                                break;
                            }
                        }
                        if i < lines.len() -1 {
                            if is_symbol(&lines[i + 1].chars().nth(j).unwrap()) {
                                has_symbol = true;
                                break;
                            }
                        }
                    }
                }

                if has_symbol{
                    sum += num;
                }
                //print!("{}:{}-{} symbol:{} ",num, loc.start, loc.end, has_symbol);
            }
        }
        //print!("\n");
        }

    return format!("{}",sum);
}

fn is_symbol(c :&char) -> bool{
    !c.is_ascii_digit() && c != &'.'
}
