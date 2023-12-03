use regex::Regex;
pub fn process(input : &str) -> String {

    let mut sum = 0;

    let re_blue = Regex::new(r"([0-9]+) blue").unwrap();
    let re_red = Regex::new(r"([0-9]+) red").unwrap();
    let re_green = Regex::new(r"([0-9]+) green").unwrap();
    for line in input.lines(){

        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for (_, [m]) in re_blue.captures_iter(&line).map(|x| x.extract() ){

            let v = m.parse::<u32>().unwrap();
            if  v > min_blue {
                min_blue = v;
            }
        }
        for (_, [m]) in re_red.captures_iter(&line).map(|x| x.extract() ){

            let v = m.parse::<u32>().unwrap();
            if  v > min_red {
                min_red = v;
            }
        }
        for (_, [m]) in re_green.captures_iter(&line).map(|x| x.extract() ){

            let v = m.parse::<u32>().unwrap();
            if  v > min_green {
                min_green = v;
            }
        }

        let power = min_blue * min_red * min_green;
        sum += power;
        //print!("Game {}: red:{}, blue:{}, green:{}\n",id,min_red,min_blue,min_green);
    };

    format!("{}",sum)
}
