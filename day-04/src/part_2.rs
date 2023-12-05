use std::collections::HashSet;

pub fn process(input : &str) -> String {

    let re = regex::Regex::new(r"[0-9]+").unwrap();
    let mut multipier = vec![1; input.lines().count()];

    for (index, card) in  input.lines().enumerate(){

        let card_split = card.split(':').nth(1).unwrap();
        let numbers_string = card_split.split('|').nth(1).unwrap();
        let winning_string = card_split.split('|').nth(0).unwrap();

        let mut winning = HashSet::new();

        for  caps in re.captures_iter(winning_string){
            for cap in caps.iter(){
                let c = cap.unwrap().as_str();
                winning.insert(c);
            }
        }

        let mut winning_count = 0;

        for caps in re.captures_iter(numbers_string){
            for cap in caps.iter(){
                let c = cap.unwrap().as_str();

                if winning.contains(c){
                    winning_count += 1;
                }
            }
        }

        for n in index+1..=(index+winning_count){
            multipier[n] += 1 * multipier[index];
        }
    }

    return format!("{}",multipier.iter().sum::<i32>());
}
