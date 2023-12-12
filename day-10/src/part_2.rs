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

    let mut current_coordinates = (start.0,start.1+1); // hard coded cause lazy :)
    // find a matching output and follow it There can be no branches (yet...)
    let mut previous_coordinates = start;
    let mut pipes = vec![];

    pipes.push(start);
    pipes.push(current_coordinates);

    while current_coordinates != start{
        (current_coordinates, previous_coordinates ) =  find_next_way(current_coordinates,previous_coordinates, &map);
        //print!("{}",map[current_coordinates.0][current_coordinates.1]);
        pipes.push(current_coordinates);
    }

    for i in 0..map.len(){
        for j in 0..map[0].len(){
            if pipes.contains(&(i,j)){
                //print!("{}",map[i][j])
            }
            else{
                //print!(".")
            }
        }
        //print!("\n")
    }

    let area = shoelace(&pipes);

    // A = i + b/2 - 1
    //Pick's Theorem
    let i = area + 1.0 - (pipes.len()as f32 -1.0)/2.0;

    return format!("{}",i);
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

    if previous == pos {
        return (out.0, pos);
    }
    if out.0 != previous{
        return (out.0, pos);
    }
    else if out.1 != previous {
        return (out.1, pos);
    }

    else { panic!("I can't go back!") }
}


fn shoelace(pipes : &Vec<(usize,usize)>)-> f32{

    let mut area2:f32 = 0f32;
    for i in 0..pipes.len()-1{
        area2  += matrix_add(pipes[i], pipes[i+1])
    }

    area2.abs()/2f32

}

fn matrix_add(a : (usize,usize), b : (usize,usize) )-> f32{
    (a.0 * b.1) as f32 - (a.1 * b.0) as f32
}