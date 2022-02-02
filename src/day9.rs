use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Vector2Int 
{
    x: i32,
    y: i32
}

impl Vector2Int 
{
    fn add(&self, other: Vector2Int) -> Vector2Int
    {
        return Vector2Int { x: self.x + other.x, y: self.y + other.y };
    }

    fn my_valid_neighbors(&self, range: &Vector2Int) -> Vec<Vector2Int> 
    {
        let directions = [Vector2Int{ x: 1, y: 0}, Vector2Int{ x: -1, y: 0}, Vector2Int{ x: 0, y: 1}, Vector2Int{ x: 0, y: -1}];
        return  directions.into_iter().
                map(|d| self.add(d)).
                filter(|v| range.within_this(v)).
                collect();
    }

    fn within_this(&self, to_check: &Vector2Int) -> bool 
    {
        return to_check.x >= 0 && to_check.x <= self.x && to_check.y >= 0 && to_check.y <= self.y;
    }
}

pub fn day_9(lines : Vec<String>)
{
    let mut map : HashMap<Vector2Int, i32> = HashMap::new();
    let range = Vector2Int { x : (lines.get(0).unwrap().len()-1) as i32, y: (lines.len()-1) as i32 };

    for (y, line) in lines.into_iter().enumerate()
    {
        for (x, char) in line.chars().enumerate() 
        {
            map.insert(Vector2Int{ x: x as i32, y: y as i32 }, String::from(char).parse::<i32>().unwrap());
        }
    }

    let mut basins: Vec<Vector2Int> = Vec::new();
    let y_range = range.y+1;

    for x in 0..range.x+1 
    {
        for y in 0..y_range
        {
            let pt = Vector2Int{ x: x, y:y };
            let cell_value = map.get(&pt).unwrap();
            if pt.my_valid_neighbors(&range).into_iter().map(|n| map.get(&n).unwrap()).all(|i| i > cell_value) 
            {
                basins.push(pt);
            }
        }
    }
    
    let mut basin_sizes: Vec<i32>= basins.into_iter().map(|b| recursively_get_size_of_basin(&b, &range, &map, &mut HashSet::new())).collect();
    basin_sizes.sort_unstable();
    let answer = basin_sizes.get(basin_sizes.len()-3).unwrap() * basin_sizes.get(basin_sizes.len()-2).unwrap() * basin_sizes.get(basin_sizes.len()-1).unwrap();

    println!("{:?} answer", answer);
}

fn recursively_get_size_of_basin(start: &Vector2Int, range: &Vector2Int, weight_map: &HashMap<Vector2Int, i32>, basin_map: &mut HashSet<Vector2Int>) -> i32 
{
    let mut to_return = 1;
    let my_elevation = weight_map.get(start).unwrap();
    
    for n in start.my_valid_neighbors(range).into_iter() 
    {
        let neighbor_elevation = weight_map.get(&n).unwrap();
        if my_elevation < neighbor_elevation && neighbor_elevation != &9 && !basin_map.contains(&n) 
        {
            basin_map.insert(n);
            to_return = to_return + recursively_get_size_of_basin(&n, range, weight_map, basin_map);
        }
    }
    
    return to_return;
}