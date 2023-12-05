use std::collections::HashSet;

pub fn process(input : &str) -> String {

    let mut sum = 0;
    let re = regex::Regex::new(r"[0-9]+").unwrap();

    for card in  input.lines(){

        let card_split = card.split(':').nth(1).unwrap();
        let numbers_string = card_split.split('|').nth(1).unwrap();
        let winning_string = card_split.split('|').nth(0).unwrap();

        let mut winning = HashSet::new();

        for caps in re.captures_iter(winning_string){
            for cap in caps.iter(){
                let c = cap.unwrap().as_str();
                winning.insert(c);
            }
        }

        let mut winning_count = 0;

        for  caps in re.captures_iter(numbers_string){
            for cap in caps.iter(){
                let c = cap.unwrap().as_str();

                if winning.contains(c){
                    winning_count += 1;
                }
            }
        }

        let points = if winning_count != 0 {2i32.pow(winning_count-1)} else {0};
        sum += points;
    }

    return format!("{}",sum);
}
