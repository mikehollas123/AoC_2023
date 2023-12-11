
pub fn process(input : &str) -> String {
    let mut total_predicted = 0;

    for line in input.lines(){
        let data: Vec<_> = line.trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        println!("{:?}",data);
        let mut last_diff = vec![];
        let mut diff =get_diffs(&data);
        println!("{:?}",diff);
        last_diff.push(diff[diff.len() -1]);

        while diff.iter().sum::<i64>() != 0{ // this used to just check the last one
            diff = get_diffs(&diff);
            last_diff.push(diff[diff.len() -1]);
            println!("{:?}",diff);
        }
        for i in (1..last_diff.len()).rev(){
            last_diff[i-1] = last_diff[i] + last_diff[i-1];
        }

        let predicted = last_diff[0] + data[data.len() -1];

        println!("{:?}",last_diff);
        println!("{:?}", predicted);

        total_predicted += predicted;
    }
    //println!("{}",pre.iter().sum::<i64>());
    return format!("{}",total_predicted);
}

fn get_diffs(data: &Vec<i64>)-> Vec<i64>{
    let mut diffs = vec![];
    for i in 0..data.len()-1{
        diffs.push(data[i+1] - data[i]);
    }
    diffs
}