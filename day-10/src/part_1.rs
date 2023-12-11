pub fn process(input : &str) -> String {

    let map: Vec<_> = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();

    // find S
    let mut start : (usize,usize) = (0,0);

    for i in 0..map.len(){
        for j in 0..map[i].len(){
            if map[i][j] == 'S'{
                start = (i,j);

            }
        }
    }

    let mut current_coordinates = (50,38);
    // find a matching output and follow it There can be no branches (yet...)
    let mut previous_coordinates = start;
    let mut pipes = 1;

    while current_coordinates != start{
        (current_coordinates, previous_coordinates ) =  find_next_way(current_coordinates,previous_coordinates, &map);
        //println!("{}",map[current_coordinates.0][current_coordinates.1])
        pipes += 1;
    }
    return format!("{}",pipes/2);
}

fn find_next_way(pos : (usize,usize), previous: (usize,usize),  map: &Vec<Vec<char>>) -> ((usize,usize),(usize,usize)){


    let out = match map[pos.0][pos.1] {
        'L' => { ((pos.0-1,pos.1),(pos.0,pos.1+1))},
        '7' => {((pos.0,pos.1-1),(pos.0+1,pos.1))},
        '|' => {((pos.0-1,pos.1),(pos.0+1,pos.1))},
        '-' => {((pos.0,pos.1-1),(pos.0,pos.1+1))},
        'J' => {((pos.0,pos.1-1),(pos.0-1,pos.1))},
        'F' => {((pos.0,pos.1+1),(pos.0+1,pos.1))},
        _ => panic!("OFF THE PATH!")
    };

    if out.0 != previous{
        return (out.0, pos);
    }
    else if out.1 != previous{
        return (out.1, pos);
    }
    else { panic!("I can't go back!") }

}
