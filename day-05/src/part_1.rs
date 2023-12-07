use regex::Regex;

pub fn process(input : &str) -> String {

    let re = Regex::new(r"map:").unwrap();

    let mut lines = input.lines();

    let seeds: Vec<_> = lines.nth(0).unwrap().split(":").nth(1).unwrap().trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

    let mut mappers : Vec<Mapper> = Vec::new();

    let mut i = 0;

    for (index , line) in lines.enumerate(){
        if index == 0{
            continue;
        }

        if re.is_match(line){
            // start collecting maps
            mappers.push( Mapper::new());

            continue;
        }

        if line.is_empty(){
            // stop collecting
            i += 1;

            continue;
        }

        let map: Vec<_> = line.trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

        let start = map.get(1).unwrap();
        let end = start + map.get(2).unwrap() -1;
        let out_start = map.get(0).unwrap();

        mappers[i].add_route(start,end,out_start);

       // print!("line : {:?}\n",map)
    }
    //mappers.push(&mut mapper);

  let mut smallest = 99999999999999999u64;
    for seed in seeds{
        let mut v = seed;
        for map in &mappers{
            v = map.get(v);
            //print!("{},",v);

        }
        if v < smallest{
            smallest = v;
        }
    }


    return format!("{}",smallest);
}

struct Mapper{
map : Vec<(u64,u64,u64)>
}
impl Mapper {
    fn new() -> Self{
        Mapper { map : vec![]}
    }

    fn add_route(&mut self, start: &u64, end:u64, out_start: &u64){
        self.map.push((*start,end,*out_start));
    }

    fn get(&self , input : u64) -> u64 {

        for route in &self.map{

            if input <= route.1 && input >= route.0{
                let v  = route.2 + (input - route.0);
                return v;
            }
        }
        return input;
    }

}
