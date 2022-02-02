use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Vector2Int
{
    x : i32,
    y : i32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct VentLine
{
    start: Vector2Int,
    end: Vector2Int
}

impl VentLine 
{
    fn is_horizontal_or_vertical(&self) -> bool
    {
        return self.start.x == self.end.x || self.start.y == self.end.y;
    }

    fn points(&self) -> Vec<Vector2Int>
    {
        let x_pts = points_between(self.start.x, self.end.x);
        let y_pts = points_between(self.start.y, self.end.y);
        let l = line(x_pts, y_pts);

        //println!("{:?} {:?}", self, l);
        
        return l;
    }
}

fn line(x_pts : Vec<i32>, y_pts: Vec<i32>) -> Vec<Vector2Int>
{
    if x_pts.len() == y_pts.len() 
    {
        return x_pts.iter().zip(y_pts.iter()).map(|pt| Vector2Int{ x : *pt.0 , y: *pt.1}).collect();
    } 
    else if x_pts.len() > 1
    {
        return x_pts.iter().map(|x| Vector2Int{ x : *x , y: *y_pts.get(0).unwrap()}).collect();
    } 
    else 
    {
        return y_pts.iter().map(|y| Vector2Int{ x : *x_pts.get(0).unwrap() , y: *y}).collect();
    }
}


fn points_between(a: i32, b: i32) -> Vec<i32> 
{
    if a > b { return points_between(b, a).into_iter().rev().collect(); } 
    else { return (a..b+1).collect(); }
}

fn vent_line_parse(s: String) -> VentLine 
{
    let pts: Vec<&str> = s.split(" -> ").collect();
    return VentLine 
    {
        start: point_parse(pts.get(0).unwrap()),
        end: point_parse(pts.get(1).unwrap())
    };
}

fn point_parse(s: &str) -> Vector2Int 
{
    let pts : Vec<&str> = s.split(",").collect();
    return Vector2Int 
    { 
        x: pts.get(0).unwrap().parse::<i32>().unwrap(), 
        y: pts.get(1).unwrap().parse::<i32>().unwrap()
    };
}

pub fn day_5(lines : Vec<String>)
{
    let mut vent_lines: Vec<VentLine>= lines.into_iter().map(|s| vent_line_parse(s)).collect();

    let mut point_count_map : HashMap<Vector2Int, i32> = HashMap::new();

    for pt in vent_lines.into_iter().flat_map(|vl| vl.points().into_iter()) 
    {
        if point_count_map.contains_key(&pt) 
        {
            let v = point_count_map.get(&pt).unwrap();
            point_count_map.insert(pt, v+1);
        } 
        else 
        {
            point_count_map.insert(pt, 1);
        }
    }
    println!("{:?} points > 2", point_count_map.values().into_iter().filter(|v| **v >= 2).collect::<Vec<&i32>>().len());
}