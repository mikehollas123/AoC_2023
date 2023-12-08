use regex::Regex;

pub fn process(input : &str) -> String {

    let lines: Vec<_> = input.split("\n").collect();
    let time_line = lines[0];
    let distance_line = lines[1];
    let re = Regex::new(r"([0-9]+)").unwrap();

    let mut best_times = vec![];
    for (m,[_]) in re.captures_iter(time_line).map(|x| x.extract()){
        best_times.push(m.parse::<u32>().unwrap());
    }

    let mut distances = vec![];
    for (m,[_]) in re.captures_iter(distance_line).map(|x| x.extract()){
        distances.push(m.parse::<u32>().unwrap());
    }

    let mut output = 1;

    for i in 0..best_times.len(){
        let mut win_count = 0;
        for j in 1..=best_times[i]{
            if wins(distances[i], best_times[i], j){
                win_count += 1;
            }
        }
        output *= win_count;
    }

    return format!("{}", output);
}

fn wins(distance: u32, max_time: u32, hold_time : u32) -> bool{
    (distance / hold_time) < max_time - hold_time
}