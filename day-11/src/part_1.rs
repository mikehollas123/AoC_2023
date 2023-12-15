pub fn process(input : &str) -> String {

    let map: Vec<_> = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();
    let mut empty_rows = 0;
    let mut empty_cols = vec![0;map[0].len()];

    for row in 0..map.len(){
       for col in 0..map[row].len(){
           if map[row][col] == '#'{
               empty_cols[col] += 1;
           }
       }
    }
    let mut empty_cols_count = 0;
    for i in 0..empty_cols.len(){
        if empty_cols[i] == 0{
            empty_cols_count += 1;
        }
        empty_cols[i] = empty_cols_count;
    }

    let mut galaxies = vec![];

    for m in 0..map.len(){
        if map[m].contains(&'#'){
            for n in 0..map[m].len(){
                if map[m][n] == '#' {
                    galaxies.push ((m + empty_rows , n + empty_cols[n]));
                }
            }
        }
        else {
            empty_rows += 1;
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len(){
        for j in 0..galaxies.len(){
            if i != j {
                let y = (galaxies[j].0 as i32 - galaxies[i].0 as i32).abs()  as i32;
                let x = ( galaxies[j].1 as i32 - galaxies[i].1 as i32).abs();

                sum += y + x;
            }
        }
    }

    return format!("{}",sum/2);
}
