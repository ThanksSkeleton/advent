use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Vector2Int
{
    x : i32,
    y : i32,
}

impl Vector2Int 
{ 
    fn mult(&self, other: Vector2Int) -> Vector2Int 
    {
        return Vector2Int{ x: self.x * other.x, y: self.y * other.y};
    } 


    fn add(&self, other: Vector2Int) -> Vector2Int
    {
        return Vector2Int{ x: self.x + other.x, y: self.y + other.y};
    }

    fn within_range(&self, range_maxes: Vector2Int) -> bool 
    {
        return self.x >= 0 && self.x <= range_maxes.x && self.y >= 0 && self.y <= range_maxes.y;
    }

    fn valid_neighbors(&self, range_maxes: Vector2Int) -> Vec<Vector2Int>
    {
        return directions().
                into_iter().
                map(|d| self.add(d)).
                filter(|v| v.within_range(range_maxes)).
                collect();
    }
}

fn directions() -> [Vector2Int; 4]
{
    return 
    [
        Vector2Int { x : 0,  y : 1},
        Vector2Int { x : 0,  y : -1},
        Vector2Int { x : 1,  y : 0},
        Vector2Int { x : -1, y : 0},
    ];
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.path_info.unwrap().lowest_score.cmp(&self.path_info.unwrap().lowest_score)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Node 
{
    position: Vector2Int,

    penalty : i32,
    path_info : Option<PathInfo>
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PathInfo 
{
    lowest_score : i32,
    best_neighbor : Vector2Int
}

fn new_node(position: Vector2Int, penalty : i32) -> Node
{
    return Node { position : position, penalty : penalty, path_info : None };
}

impl Node 
{
    fn update_node(&mut self, parent: Node) -> bool
    {
        //println!("Updating path for {:?} with {:?}", self.position, self.path_info);

        if self.path_info.is_none() || self.path_info.unwrap().lowest_score > parent.path_info.unwrap().lowest_score + parent.penalty
        {
            //println!("Updating path for {:?} with {}", self.position, parent.path_info.unwrap().lowest_score + parent.penalty);
            self.path_info = Some(PathInfo
            { 
                lowest_score : parent.path_info.unwrap().lowest_score + parent.penalty,
                best_neighbor : parent.position
            });
            return true;
        } 
        
        return false;
    }

    fn update_node_start(&mut self) 
    {
        self.path_info = Some(PathInfo
        { 
            lowest_score : 0,
            best_neighbor : Vector2Int{ x: 0, y:0 }
        });
    }
}


pub fn day_15(lines : Vec<String>)
{
    // let mut test_node = new_node(Vector2Int{ x : 0, y : 0}, 2);  
    // test_node.update_node_start();
    // let mut test_node_2 = new_node(Vector2Int{ x : 1, y : 0}, 7); 
    // test_node_2.update_node(test_node);
    // let mut test_node_3 = new_node(Vector2Int{ x : 0, y : 0}, 1);    
    // test_node_3.update_node(test_node);

    
    // println!("{} test2", test_node_2.path_info.unwrap().lowest_score);
    // println!("{} test3", test_node_3.path_info.unwrap().lowest_score);

    // test_node_2.update_node(test_node_3);

    // println!("{} test2 _ 2", test_node_2.path_info.unwrap().lowest_score);

    // let mut test_node_4 = new_node(Vector2Int{ x : 2, y : 0}, 2);
    
    // test_node_4.update_node(test_node_2);

    // println!("{} test4", test_node_4.path_info.unwrap().lowest_score);

    // let mut test_node_2 = new_node(Vector2Int{ x : 1, y : 0}, 2); 
  
    // test_node_2.update_node(test_node);

    // println!("{} test", test_node_2.path_info.unwrap().lowest_score);

    // test_node_2.update_node(test_node_3);

    // println!("{} test", test_node_2.path_info.unwrap().lowest_score);

    //return;



    let mut dict: HashMap<Vector2Int, Node> = HashMap::new();

    let y_width : i32 = i32::try_from(lines.len()).unwrap();
    let y_width_true = y_width * 5;

    let first_line: Vec<char> = lines[0].chars().collect();

    let x_width : i32 = i32::try_from(first_line.len()).unwrap();
    let x_width_true = x_width * 5;

    let start = Vector2Int{ x : 0, y : 0};
    let true_end = Vector2Int{ x : x_width_true -1, y : y_width_true - 1 };

    println!("{:?} start", start);
    println!("{:?} true_end", true_end);


    for y in 0..y_width
    {
        let line : Vec<char> = lines.get(usize::try_from(y).unwrap()).unwrap().chars().collect();
        for x in 0..x_width
        {
            for x_modified in 0..5 
            {
                for y_modified in 0..5 
                {
                    let adjustment = 
                    (Vector2Int { x: x_modified, y: y_modified }).
                    mult(Vector2Int { x: x_width, y: y_width });
                    let pt = Vector2Int{ x: i32::try_from(x).unwrap(), y: i32::try_from(y).unwrap() }.add(adjustment);
                    let raw_char : char = *line.get(usize::try_from(x).unwrap()).unwrap();
                    let penalty = penalty_count((String::from(raw_char)).parse::<i32>().unwrap() + x_modified + y_modified);
                    dict.insert(pt, new_node(pt, penalty));
                }
            }
        }
    }

    let mut priority_queue = BinaryHeap::new();

    let mut zero_zero_element = *dict.get(&start).unwrap();
    zero_zero_element.update_node_start();
    priority_queue.push(zero_zero_element);

    while !priority_queue.is_empty() 
    {
        let to_explore = priority_queue.pop().unwrap();
        for n_position in to_explore.position.valid_neighbors(true_end) 
        {
            //println!("pop {:?}", n_position);
            let mut n = *dict.get(&n_position).unwrap();       
            if n.update_node(to_explore) 
            {
                dict.insert(n_position, n);
                priority_queue.push(n);
            }
        }
    }


    let mut path_score = 0;
    let mut current_path_position = true_end;

    while current_path_position != start 
    {
        let node = dict.get(&current_path_position).unwrap();
        path_score = path_score + node.penalty;
        current_path_position = node.path_info.unwrap().best_neighbor;
    }

    println!("{} best path", path_score);
}

fn penalty_count(input: i32) -> i32 
{
    if input >= 10 
    {
        input - 9
    } else {
        input
    }
}

