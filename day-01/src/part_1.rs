pub fn process(input : &str) -> String {

    let mut sum : u32 = 0;
    for line in input.lines(){

        let mut first : Option<char> = None;
        let mut last : char = '0';

        for c in line.chars(){
            if  c.is_ascii_digit(){
                last = c;

                if first.is_none(){
                    first = Some(c);
                }
            }
        }
        let  num : u32 = format!("{}{}",first.unwrap(),last).trim().parse().unwrap();
        sum += num;
    }

    return format!("{}",sum);
}