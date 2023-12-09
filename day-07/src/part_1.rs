use std::cmp::Ordering;
use std::collections::HashMap;

pub fn process(input : &str) -> String {

    let mut games = vec![];
    for game in  input.lines(){
        games.push(Game {hand:game.split(" ").nth(0).unwrap(), bet:game.split(" ").nth(1).unwrap().parse().unwrap() })
    }

    games.sort();

    let mut total_winnings = 0;
    for (r, g)  in games.iter().rev().enumerate(){ // whoops backwards :)
        total_winnings += (r+1) as u32 * g.bet;
    }

    return format!("{}",total_winnings);
    }

pub struct Game<'a> {
    hand : &'a str,
    bet : u32
}

impl<'a> Game<'a> {
    fn get_rank(&self) -> u32{

        let mut map = HashMap::new();

        for char in self.hand.chars(){

            if !map.contains_key(&char){
                map.insert(char, 1);
            }
            else {
                *map.get_mut(&char).unwrap() += 1;
            }
        }

        if map.keys().count() == 1
        {
            return  1;
        }
        else if map.keys().count() == 2{
            if map.values().max().unwrap() == &4{
                return 2; // 4 kind
            }
            return  3 // full house
        }
        else if map.keys().count() == 3{
            if map.values().max().unwrap() == &3{
                return 4 //3 of a kind
            }
            return 5 // two pairs
        }
        else if map.keys().count() == 4{
            return 6 // one pair
        }
        else{
            return 7;
        }
    }
}

impl<'a> PartialEq<Self> for Game<'a> {
    fn eq(&self, other: &Self) -> bool {
        &self.get_rank() == &other.get_rank()
    }
}

impl<'a> Eq for Game<'a> {}

impl<'a> Ord for Game<'a>{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<'a> PartialOrd<Self> for Game<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_rank() < other.get_rank(){
            return Some(Ordering::Less);
        }
        if self.get_rank() > other.get_rank(){
            return Some(Ordering::Greater);
        }
        else
        {
            // if the same rank order by card values l to r

            let bytes =  self.hand.as_bytes();
            let other_bytes = other.hand.as_bytes();

            for i in 0..bytes.len()
            {
                if parse_card(bytes[i] as char) < parse_card(other_bytes[i] as char){
                return Some(Ordering::Less);
                }
                else if parse_card(bytes[i] as char) > parse_card(other_bytes[i] as char){
                    return Some(Ordering::Greater);
                }
                else {
                    continue;
                }
            }

            panic!();
        }
    }
}

fn parse_card(card : char) -> u32
{
    return match card
    {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 4,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
            '5' => 10,
        '4' => 11,
            '3' => 12,
        '2' => 13,
        _ => panic!(),
    }
}

