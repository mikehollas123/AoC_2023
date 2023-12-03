pub fn process(input : &str) -> String {

    let mut sum : u32 = 0;
    for line in input.lines(){

        let mut first : Option<u32> = None;
        let mut last : u32 = 0;

        let bytes = line.as_bytes();
        let mut i = 0;
        while i < line.len(){
            if  bytes[i].is_ascii_digit(){
                last = (bytes[i] as char).to_digit(10).unwrap(); //urgh

                if first.is_none(){
                    first = Some((bytes[i] as char).to_digit(10).unwrap());
                }
            }
            else {
                let (c , _pos) = check_for_num(&bytes[i..]);

                if c.is_some() {
                    last = c.unwrap();

                    if first.is_none() {
                        first = Some(c.unwrap());
                    }
                    //i += pos - 1; // I was trying to be clever! MEAN!
                }
            }

            i += 1;
        }
        let  num : u32 = format!("{}{}",first.unwrap(),last).trim().parse().unwrap();
        sum += num;
    }

    return format!("{}",sum);
}

fn check_for_num(str : &[u8]) -> (Option<u32>, usize){
    let lookup = vec!(
        "one".as_bytes(),
        "two".as_bytes(),
        "three".as_bytes(),
        "four".as_bytes(),
        "five".as_bytes(),
        "six".as_bytes(),
        "seven".as_bytes(),
        "eight".as_bytes(),
        "nine".as_bytes());
    let mut n = 0;
   'outer: for look in lookup{
       n += 1;
       let mut i = 0;
        for c in look{
            if &i >= &str.len(){
                continue 'outer
            }
            if c != &str[i]{
                continue 'outer
            }
            i += 1;
        }
       return (Some(n),look.len())
    };

    (None, 0)
}