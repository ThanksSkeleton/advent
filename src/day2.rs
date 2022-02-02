struct Vector2Int(i32, i32);
struct Vector3Int(i32, i32, i32);

impl Vector2Int 
{
    fn add(&self, other: Vector2Int) -> Vector2Int 
    {
        return Vector2Int(self.0 + other.0, self.1 + other.1);
    }
}

pub fn day_2(lines : Vec<String>)
{
    let answer = 
        lines.
        into_iter().
        map(|l| parse_line(l)).
        reduce(|acc, v| acc.add(v)).
        unwrap();

    println!("{} x*y", answer.0 * answer.1);
}

pub fn day_2_part_2(lines : Vec<String>)
{
    let answer = 
        lines.
        into_iter().
        map(|l| parse_line(l)).
        fold(Vector3Int(0,0,0), |acc, v| travel(acc, v));

    println!("{} x*y", answer.0 * answer.2);
}

fn travel(acc: Vector3Int, v: Vector2Int) -> Vector3Int 
{
    return Vector3Int(
        acc.0 + v.0, // moving
        acc.1 + v.1, // changing aim
        acc.2 + (v.0 * acc.1) // travel down based on previous aim
    );
}

fn parse_line(line: String) -> Vector2Int 
{
    let split: Vec<&str> = line.as_str().split(' ').collect();
    match split.get(0).unwrap() 
    {
        &"forward" => return Vector2Int(split.get(1).unwrap().parse::<i32>().unwrap(), 0),
        &"up" => return Vector2Int(0, -split.get(1).unwrap().parse::<i32>().unwrap()),
        &"down" => return Vector2Int(0, split.get(1).unwrap().parse::<i32>().unwrap()), 
        _ => panic!()
    }
}