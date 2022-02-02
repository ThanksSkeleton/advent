pub fn day_7(lines : Vec<String>)
{
    let values : Vec<i32>= lines.get(0).unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let max = *values.iter().max().unwrap();
    let min = *values.iter().min().unwrap();
    let min_cost : i32 = (min..max+1).into_iter().map(|pt| alignment_cost_2(&values, pt)).min().unwrap();
    //println!("{:?} all values", costs)
    println!("{} min", min_cost);
}

fn alignment_cost(base : &Vec<i32>, alignment_point: i32) -> i32
{
    return base.iter().map(|i| (alignment_point - i).abs()).sum();
}

fn alignment_cost_2(base : &Vec<i32>, alignment_point: i32) -> i32
{
    return base.iter().map(|i| (alignment_point - i).abs()).map(|n| ((n * n) + n) / 2).sum();
}