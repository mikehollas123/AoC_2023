
pub fn process(input : &str) -> String {
    let mut total_predicted = 0;

    for line in input.lines(){

        let data: Vec<_> = line.trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        //println!("{:?}",data);
        let mut first_diff = vec![];
        let mut diff =get_diffs(&data);
        //println!("{:?}",diff);
        first_diff.push(diff[0]);

        while !all_zero(&diff){ // this used to just check the last one
            diff = get_diffs(&diff);
            first_diff.push(diff[0]);
            //println!("{:?}",diff);
        }
        println!("{:?}",diff);
        for i in (1..first_diff.len()).rev(){
            first_diff[i-1] =  first_diff[i-1] - first_diff[i] ;
        }

        let predicted = data[0] - first_diff[0] ;

        //println!("{:?}", first_diff);
        //println!("{:?}", predicted);
        total_predicted += predicted;
    }

    return format!("{}",total_predicted);
}

fn all_zero(diff: &Vec<i64>)-> bool {
    for i in diff {
        if i != &0{
            return false;
        }
    }
    true
}

fn get_diffs(data: &Vec<i64>)-> Vec<i64>{
    let mut diffs = vec![];
    for i in 0..data.len()-1{
        diffs.push(data[i+1] - data[i]);
    }
    diffs
}