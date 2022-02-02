use std::collections::HashMap;

type NodeId = i32;

const LEFT : usize = 0;
const RIGHT : usize = 1;

fn opposite(x: usize) -> usize
{
    return 1-x;
}

#[derive(Copy, Clone)]
enum SnailfishNumber
{
    Regular(i32),
    Composite([NodeId; 2]),
}

impl SnailfishNumber 
{
    fn as_reg(&self) -> i32 
    {
        match self 
        {
            SnailfishNumber::Regular(i) => *i,
            SnailfishNumber::Composite(_) => panic!("wrong type")
        }
    }

    fn as_comp(&self) -> [NodeId; 2] 
    {
        match self 
        {
            SnailfishNumber::Regular(_) => panic!("wrong type"),
            SnailfishNumber::Composite(i) => *i
        }
    }
}

struct SnailfishNode 
{
    id: NodeId,

    location_string : Vec<usize>,

    num: SnailfishNumber
}

impl SnailfishNode 
{
    fn is_comp(&self) -> bool 
    {
        match self.num 
        {
            SnailfishNumber::Regular(_) => false,
            SnailfishNumber::Composite(_) => true
        }
    }
}

struct SnailfishTree 
{
    current_node_id : NodeId,

    inner: HashMap<NodeId, SnailfishNode>
}

impl SnailfishTree 
{
    fn pretty_print(&self, node_id: NodeId) -> String
    {
        match self.inner.get(&node_id).unwrap().num 
        {            
            SnailfishNumber::Regular(i) => i.to_string(),
            SnailfishNumber::Composite([l, r]) => 
            { 
                let mut to_return = "[".to_owned();
                to_return.push_str(&self.pretty_print(l));
                to_return.push_str(",");
                to_return.push_str(&self.pretty_print(r));
                to_return.push_str("]");
                return to_return;
            }
        }
    }

    fn mag_estimate(&mut self, node_id: NodeId) -> [i32; 2] 
    {
        let resid = self.reduce(node_id, 3);
        let raw = 3 * self.mag(node_id);
        let resid_max = 3*3*3*(resid[0] + resid[1]);
        return [raw, raw+resid_max];
    }

    fn left_magnitude_estimate(&mut self, node_id: NodeId) -> [i32; 2]
    {
        let resid = self.reduce(node_id, 3);
        let raw = 3 * self.mag(node_id);
        let right_pushed_max = 2*3*3*resid[1];
        return [raw, raw + right_pushed_max];
    }

    fn right_magnitude_estimate(&mut self, node_id: NodeId) -> [i32; 2]
    {
        let resid = self.reduce(node_id, 3);
        let raw = 2 * self.mag(node_id);
        let right_pushed_max = 3*2*2*resid[0];
        return [raw, raw + right_pushed_max];
    }

    fn mag(&self, node_id: NodeId) -> i32 
    {
        match self.inner.get(&node_id).unwrap().num 
        {
            SnailfishNumber::Regular(i) => i,
            SnailfishNumber::Composite([l, r]) => 3 * self.mag(l) + 2 *self.mag(r)
        }
    }

    fn add(&mut self, a: NodeId, b: NodeId) -> NodeId 
    {
        self.prepend_direction(a, LEFT);
        self.prepend_direction(b, RIGHT);
        let to_return = self.create_node_comp(Vec::new(), a, b);
        self.reduce(to_return, 4);
        return to_return;
    }

    fn prepend_direction(&mut self, node_id: NodeId, direction: usize) 
    {
        let node = self.inner.get_mut(&node_id).unwrap();
        node.location_string.insert(0, direction);
        match node.num 
        {
            SnailfishNumber::Regular(_) => (),
            SnailfishNumber::Composite([l, r]) => { self.prepend_direction(l, direction); self.prepend_direction(r, direction); }
        }
    }

    fn is_normal_pair(&self, id: NodeId) -> bool
    {
        match self.inner.get(&id).unwrap().num 
        {
            SnailfishNumber::Regular(_) => false,
            SnailfishNumber::Composite([l, r]) => !self.inner.get(&l).unwrap().is_comp() && !self.inner.get(&l).unwrap().is_comp()
        }
    }

    fn create_node_reg(&mut self, location: Vec<usize>, value: i32) -> NodeId
    {
        self.current_node_id += 1;
        self.inner.insert(self.current_node_id, SnailfishNode{ id: self.current_node_id, location_string: location, num: SnailfishNumber::Regular(value)});
        return self.current_node_id;
    }

    fn create_node_comp(&mut self, location: Vec<usize>, left: NodeId, right: NodeId) -> NodeId
    {
        self.current_node_id += 1;
        self.inner.insert(self.current_node_id, SnailfishNode{ id: self.current_node_id, location_string: location, num: SnailfishNumber::Composite([left, right])});
        return self.current_node_id;
    }

    fn reduce(&mut self, root: NodeId, recurse_level: usize) -> [i32; 2]
    {
        //println!("attempting to reduce {}", self.pretty_print(root));
        let mut resid = [0, 0];
        loop 
        {
            let (reduced, residulum) = self.reduce_iteration(root, recurse_level);

            if !reduced 
            {
                //println!("done reducing");
                return resid;
            } 
            else 
            {
                resid[0] = resid[0] + residulum[0];
                resid[1] = resid[1] + residulum[1];
            }            
            
            //println!("reduced to {} - (resid {:?})", self.pretty_print(root), [lresid, rresid]);
        }
    }

    fn reduce_iteration(&mut self, root: NodeId, recurse_level: usize) -> (bool, [i32; 2]) 
    {
        let (exploded, residulum) = self.find_and_explode(root, recurse_level);
        if exploded 
        {
           (true, residulum) 
        } else {
            (self.find_and_split(root), [0,0])
        }
    }

    fn find_and_explode(&mut self, root: NodeId, recurse_level: usize) -> (bool, [i32; 2])
    {
        let mut explodeable = self.all_nodes_left_first(root).into_iter().
                          map(|id| self.inner.get(&id).unwrap()).
                          filter(|n| n.location_string.len() >= recurse_level).
                          filter(|n| self.is_normal_pair(n.id));

        let to_explode = explodeable.next();
        if to_explode.is_some() 
        {
            return (true, self.explode(to_explode.unwrap().id, root));
        } 
        else 
        {
            return (false, [0, 0]);
        }
    }

    fn explode(&mut self, node: NodeId, root: NodeId) -> [i32; 2]
    {
        let n1 = self.inner.get(&node).unwrap().num;
        let mut resid = [0, 0];
        for direction in [LEFT, RIGHT] 
        {
            let child_id = &n1.as_comp()[direction];
            let child_node = self.inner.get(child_id).unwrap();
            let child_node_value = child_node.num.as_reg();

            let neighbor_id = self.find_neighbor_lr(direction, node, root);
            if neighbor_id.is_some() 
            {
                self.bump_node(neighbor_id.unwrap(), child_node_value);
            } 
            else 
            {
                resid[direction] = child_node_value;
            }
        }
        let mut n = self.inner.get_mut(&node).unwrap();
        n.num = SnailfishNumber::Regular(0);

        return resid;
    }

    fn bump_node(&mut self, node: NodeId, value: i32) 
    {                
        let mut neighbor = self.inner.get_mut(&node).unwrap();
        let new_neighbor_value = neighbor.num.as_reg() + value;
        neighbor.num = SnailfishNumber::Regular(new_neighbor_value);
    }

    fn find_and_split(&mut self, root: NodeId) -> bool
    {
        let mut splittable = self.all_nodes_left_first(root).into_iter().
                          map(|id| self.inner.get(&id).unwrap()).
                          filter(|n| !n.is_comp()).
                          filter(|n| n.num.as_reg() >= 10);

        let to_split = splittable.next();
        if to_split.is_some() 
        {
            self.split(to_split.unwrap().id);
            return true;
        } 
        else 
        {
            return false;
        }
    }

    fn split(&mut self, node_id: NodeId) 
    {
        let loc = self.inner.get(&node_id).unwrap().location_string.clone();
        let node_value = self.inner.get(&node_id).unwrap().num.as_reg();

        let mut l_loc = loc.clone();
        l_loc.push(LEFT);
        let new_l_node = self.create_node_reg(l_loc, node_value/2);
        

        let mut r_loc = loc.clone();
        r_loc.push(RIGHT);
        let new_r_node = self.create_node_reg(r_loc, node_value/2 + node_value % 2);

        self.inner.get_mut(&node_id).unwrap().num = SnailfishNumber::Composite([new_l_node, new_r_node]);
    }

    fn all_nodes_left_first(&self, node_id: NodeId) -> Vec<NodeId>
    {
        match self.inner.get(&node_id).unwrap().num
        {
            SnailfishNumber::Regular(_) => return vec![node_id],
            SnailfishNumber::Composite([l, r]) =>
            {
                let mut to_return = self.all_nodes_left_first(l);
                to_return.push(node_id);
                to_return.append(&mut self.all_nodes_left_first(r));
                return to_return;
            }
        }
    }

    fn find_neighbor_lr(&self, left_right: usize, node: NodeId, root: NodeId) -> Option<NodeId> 
    {
        let loc = self.inner.get(&node).unwrap().location_string.clone();
        //println!("trying to find the {} neighbor of nodeId {} with direction string {:?}", left_right, node, loc);
        let mut direction_iter = loc.into_iter().rev().peekable();
        let mut remainder_list : Vec<usize> = Vec::new();
        
        loop {
            let next = direction_iter.next();
            if next.is_none()
            {
                //println!("direction string exhausted - there is no {} neighbor", left_right);
                return None;
            }
            
            if next.unwrap() != left_right 
            {
                remainder_list = direction_iter.rev().collect();                
                //println!("found different direction: remainder is {:?}", remainder_list);
                break;
            }
        }

        remainder_list.push(left_right);

        let followed = self.find_node_lr(opposite(left_right), self.get_from_directions(remainder_list, root));
        //println!("followed directions to neighbor {} with value {:?}", followed, self.pretty_print(followed));

        return Some(followed);
    }

    fn get_from_directions(&self, directions: Vec<usize>, node_under: NodeId) -> NodeId 
    {
        if directions.len() == 0 
        {
            return node_under;
        } 
        else 
        {
            match self.inner.get(&node_under).unwrap().num
            {
                SnailfishNumber::Regular(x) => panic!("there are remaining directions"),
                SnailfishNumber::Composite(slice) => return self.get_from_directions(directions[1..].to_vec(), slice[directions[0]]),
            }
        }
    }


    fn find_node_lr(&self, left_right: usize, node_under: NodeId) -> NodeId
    {      
        match self.inner.get(&node_under).unwrap().num
        {
            SnailfishNumber::Regular(_x) => return node_under,
            SnailfishNumber::Composite(slice) => return self.find_node_lr(left_right, slice[left_right]),
        }
    }

}

fn snailfish_parse(s: String, t: &mut SnailfishTree) -> NodeId 
{
    snailfish_parse_recursively(&s.chars().collect::<Vec<char>>()[..], Vec::new(), t)
}

fn snailfish_parse_recursively(s: &[char], loc: Vec<usize>, t: &mut SnailfishTree) -> NodeId
{
    let open_bracket = '[';
    if s.get(0).unwrap() != &open_bracket
    {
        return t.create_node_reg(loc, s.iter().collect::<String>().parse::<i32>().unwrap());
    } 
    else 
    {
        let comma_index = find_central_comma_index(&s);

        let mut l_loc = loc.clone();
        l_loc.push(LEFT);
        let left = snailfish_parse_recursively(&s.split_at(comma_index).0[1..], l_loc, t);

        let mut r_loc = loc.clone();
        r_loc.push(RIGHT);
        let right = snailfish_parse_recursively(s.split_at(comma_index+1).1.split_last().unwrap().1, r_loc, t);

        return t.create_node_comp(loc, left, right);
    }
}

fn find_central_comma_index(s: &[char]) -> usize 
{
    let mut bracket_count = 0;
    for (i, c) in s.iter().enumerate()
    {  
        match c 
        {
            '[' => bracket_count += 1,
            ']' => bracket_count += -1,
            ',' => if bracket_count == 1 { return i } else { },
            _ => (),// do nothing,
        }
    }
    panic!("no middle comma {:?}", s);
}

pub fn day_18(lines: Vec<String>)
{
    //let mut t = SnailfishTree{ current_node_id : 0, inner: HashMap::new() };

    // {
    //     let explode : String = "[[[[[9,8],1],2],3],4]".to_string();
    //     let e_id = snailfish_parse(explode, &mut t);
    //     t.reduce(e_id, 4);
    //     println!("e1: {}", t.pretty_print(e_id));
    // }

    // {
    //     let explode : String = "[7,[6,[5,[4,[3,2]]]]]".to_string();
    //     let e_id = snailfish_parse(explode, &mut t);
    //     t.reduce(e_id, 4);
    //     println!("e2: {}", t.pretty_print(e_id));
    // }

    // {
    //     let explode : String = "[[6,[5,[4,[3,2]]]],1]".to_string();
    //     let e_id = snailfish_parse(explode, &mut t);
    //     t.reduce(e_id, 4);
    //     println!("e3: {}", t.pretty_print(e_id));
    // }

    // {
    //     let explode : String = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string();
    //     let e_id = snailfish_parse(explode, &mut t);
    //     t.reduce(e_id);
    //     println!("e3: {}", t.pretty_print(e_id));
    // }

    // {
    //     let split : String = "11".to_string();
    //     let s_id = snailfish_parse(split, &mut t);
    //     t.reduce(s_id);
    //     println!("s1: {}", t.pretty_print(s_id));
    // }

    // {
    //     let split : String = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string();
    //     let s_id = snailfish_parse(split, &mut t);
    //     t.reduce(s_id);
    //     println!("s1: {}", t.pretty_print(s_id));
    // }

    // {
    //     let roots : Vec<NodeId> = lines.clone().into_iter().
    //     map(|s| snailfish_parse(s, &mut t)).
    //     collect();

    //     roots.iter().for_each(|n| println!("{}", t.pretty_print(*n)));

    //     let final_root = roots.into_iter().
    //     reduce(|acc, id| t.add(acc, id))
    //     .unwrap();

    //     println!("root: {} , mag: {}", t.pretty_print(final_root), t.mag(final_root));
    // }

    // {
    //     let reduce1 : String = "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".to_string();
    //     let r1_id = snailfish_parse(reduce1, &mut t);
    //     t.reduce(r1_id, 3);

    //     let reduce2 : String = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]".to_string();
    //     let r2_id = snailfish_parse(reduce2, &mut t);
    //     t.reduce(r2_id, 3);

    //     let sum_id = t.add(r1_id, r2_id);

    //     println!("fake sum: {}, mag: {}", t.pretty_print(sum_id), t.mag(sum_id));

    // }

    // {
    //     let reduce1 : String = "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".to_string();
    //     let r1_id = snailfish_parse(reduce1, &mut t);
    //     let reduce2 : String = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]".to_string();
    //     let r2_id = snailfish_parse(reduce2, &mut t);
        
    //     let sum_id = t.add(r1_id, r2_id);

    //     println!("true sum: {}, mag: {}", t.pretty_print(sum_id), t.mag(sum_id));
    // }

    // println!("left");

    // {
    //     let roots : Vec<NodeId> = lines.clone().into_iter().
    //     map(|s| snailfish_parse(s, &mut t)).
    //     collect();

    //     //roots.iter().for_each(|n| println!("{}", t.pretty_print(*n)));

    //     let mut mags: Vec<i32> = roots.into_iter().map(|n| t.estimate_magnitude(n)).collect(); 
    //     println!("{:?} mags", mags);
    //     mags.sort_by(|a, b| b.cmp(a));
    //     println!("{:?} mags", mags);
    //     let m1 = mags.get(0).unwrap();
    //     let m2 = mags.get(1).unwrap();
    //     println!("{} {} {}", m1, m2, 3*m1 + 2*m2)
    // }

    {
        let mut t1 = SnailfishTree{ current_node_id : 0, inner: HashMap::new() };
        let roots : Vec<NodeId> = lines.clone().into_iter().
        map(|s| snailfish_parse(s, &mut t1)).
        collect();
    
        //roots.iter().for_each(|n| println!("{}", t.pretty_print(*n)));
    
        let mut roots_with_estimate : Vec<(NodeId, [i32; 2])> = roots.iter().map(|n| (*n, t1.mag_estimate(*n))).collect();
        roots_with_estimate.sort_by(|a, b| b.1[0].cmp(&a.1[0]));
        println!("All Roots sorted: {:?}", roots_with_estimate);
        let floor = roots_with_estimate.get(1).unwrap().1[0];
        println!("Floor: {:?}", floor);
        let remaining_roots : Vec<NodeId> = roots_with_estimate.iter().filter(|x| x.1[1] >= floor).map(|r| r.0).collect();
        println!("Candidate Roots: {:?}", remaining_roots);


        let mut values = Vec::new();

        for r1 in remaining_roots.iter() 
        {
            for r2 in remaining_roots.iter() 
            {        
                if r1 != r2 
                {
                    let mut t2 = SnailfishTree{ current_node_id : 0, inner: HashMap::new() };
                    let roots_again : Vec<NodeId> = lines.clone().into_iter().
                    map(|s| snailfish_parse(s, &mut t2)).
                    collect();
    
                    let new_node_id = t2.add(*r1, *r2);
                    let mag = t2.mag(new_node_id);
    
                    values.push(mag);
                }
            }
        }

        println!("{} max value", values.iter().max().unwrap());
    }


    // println!("right");

    // {
    //     let roots : Vec<NodeId> = lines.clone().into_iter().
    //     map(|s| snailfish_parse(s, &mut t)).
    //     collect();
    
    //     //roots.iter().for_each(|n| println!("{}", t.pretty_print(*n)));
    
    //     roots.into_iter().for_each(|n| { let estimate = t.right_magnitude_estimate(n); println!("min {} max {}",estimate[0], estimate[1]) });
    // }
 

    //println!("root: {} , mag: {}", t.pretty_print(final_root), t.mag(final_root));
}