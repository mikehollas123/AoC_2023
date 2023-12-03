use regex::Regex;
pub fn process(input : &str) -> String {

    let mut sum = 0;

    let re_id = Regex::new(r"Game ([0-9]+):").unwrap();
    let re_blue = Regex::new(r"([0-9]+) blue").unwrap();
    let re_red = Regex::new(r"([0-9]+) red").unwrap();
    let re_green = Regex::new(r"([0-9]+) green").unwrap();
    for line in input.lines(){

        let mut possible : bool = true;
        let id = &re_id.captures_iter(&line).next().unwrap()[1];

        for (_, [m]) in re_blue.captures_iter(&line).map(|x| x.extract() ){
            if  m.parse::<u32>().unwrap() > 14u32 {
                possible = false;
                break;
            }
        }
        for (_, [m]) in re_red.captures_iter(&line).map(|x| x.extract() ){
            if  m.parse::<u32>().unwrap() > 12u32 {
                possible = false;
                break;
            }
        }
        for (_, [m]) in re_green.captures_iter(&line).map(|x| x.extract() ){
            if  m.parse::<u32>().unwrap() > 13u32 {
                possible = false;
                break;
            }
        }

        //print!("Game {}:{}\n",id,possible);
        if possible{
            sum += id.parse::<u32>().unwrap();
        }
    };

    format!("{}",sum)
}