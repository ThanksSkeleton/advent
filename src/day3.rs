struct RunningCount {
    zeros: i32,
    ones: i32
}

impl RunningCount 
{
    fn Add(&self, i: i32) -> RunningCount
    {
        match i 
        {
            0 => return RunningCount {zeros: self.zeros + 1, ones: self.ones },
            1 => return RunningCount {zeros: self.zeros, ones: self.ones + 1 },
            _ => panic!() 
        }
    }

    fn MostCommon(&self) -> char
    {
        if self.zeros > self.ones
        {
            return '0';
        } 
        else if self.ones > self.zeros
        {
            return '1';
        } 
        else 
        {
            return '1';
        }
    }

    fn LeastCommon(&self) -> char 
    {   
        if self.zeros > self.ones
        {
            return '1';
        } 
        else if self.ones > self.zeros
        {
            return '0';
        } 
        else 
        {
            return '0';
        }
    }
}

pub fn day_3(lines : Vec<String>)
{
    let copy = lines.clone();
    let oxy = apply_with_filter(lines, |rc| rc.MostCommon());
    let scrub = apply_with_filter(copy, |rc| rc.LeastCommon());

    let product = i32::from_str_radix(&oxy, 2).unwrap() * i32::from_str_radix(&scrub, 2).unwrap();

    println!("{:?} mult", product);
}

fn apply_with_filter(lines: Vec<String>, filter_function: fn(RunningCount) -> char) -> String
{
    let mut lines_as_vec: Vec<Vec<char>> = lines.into_iter().map(|l| l.chars().collect()).collect();
    let dim = lines_as_vec.get(0).unwrap().len(); 
    for index in 0..dim 
    {
        let mut running_count = RunningCount{ zeros: 0, ones: 0 };
        for line in &lines_as_vec 
        {
            let value = line.get(index).unwrap().to_string().parse::<i32>().unwrap();
            running_count = running_count.Add(value);
        }
    
        let f = filter_function(running_count);

        lines_as_vec.retain(|vc| vc.get(index).unwrap() == &f);

        if lines_as_vec.len() == 1 
        {
            let v = lines_as_vec.get(0).unwrap();
            let s : String = v.into_iter().collect();
            return s;          
        }
    }
    panic!();
}