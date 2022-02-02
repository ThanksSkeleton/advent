// #############
// #...........#
// ###C#C#B#D###
//   #D#A#B#A#
//   #########


use std::collections::HashSet;
use std::collections::HashMap;
use priority_queue::PriorityQueue;

const L1: i32 = 0;
const L2: i32= 1;

const A1: i32 = 2;
const A2: i32 = 3;
const A3: i32 = 4;
const A4: i32 = 5;

const AB: i32 = 6;

const B1: i32 = 7;
const B2: i32 = 8;
const B3: i32 = 9;
const B4: i32 = 10;

const BC: i32 = 11;

const C1: i32 = 12;
const C2: i32 = 13;
const C3: i32 = 14;
const C4: i32 = 15;

const CD: i32 = 16;

const D1: i32 = 17;
const D2: i32 = 18;
const D3: i32 = 19;
const D4: i32 = 20;

const R1: i32 = 21;
const R2: i32 = 22;

// Room Connections

#[derive(PartialEq, Eq)]
enum Affinity 
{
    A,
    B,
    C,
    D,
    Hallway
}

fn target_affinity(c: char) -> Option<Affinity> 
{
    match c 
    {
        'A' => Some(Affinity::A),
        'B' => Some(Affinity::B),
        'C' => Some(Affinity::C),
        'D' => Some(Affinity::D),
         _ => None
    }
}

fn affinity_as_char(a: &Affinity) -> Option<char> 
{
    match a 
    {
        Affinity::A => Some('A'),
        Affinity::B => Some('B'),
        Affinity::C => Some('C'),
        Affinity::D => Some('D'),
        _ => None
    }
}

struct Room 
{
    name: i32,

    affinity: Affinity,
    family_index: i32,

    connected_1: Vec<i32>,
    connected_2: Vec<i32>,
}

struct House 
{
    rooms: [Room; 23]
}

impl House 
{
    fn by_affinity(&self, target: &Affinity) -> Vec<&Room> 
    {
        return self.rooms.iter().
        filter(|rm| &rm.affinity == target).
        collect();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Residence 
{
    rooms: [Option<char>; 23]
}

impl Residence 
{
    fn ancestor(&self, character : char, from: i32, to :i32) -> Residence 
    {
        let mut to_return = self.clone();
        to_return.rooms[from as usize] = None;
        to_return.rooms[to as usize] = Some(character);
        return to_return;
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct ScoredResidence 
{
    current: Residence,
    goal : Residence,
    score : i32,
}

impl ScoredResidence 
{
    fn ancestor(&self, character: char, from: i32, to: i32, cost: i32) -> ScoredResidence 
    {
        ScoredResidence 
        {
            current : self.current.ancestor(character, from, to),
            goal : self.goal.clone(),
            score : self.score + cost
        }
    }

    fn winner(&self) -> bool
    {
        return self.current == self.goal;
    }
}

struct PodType 
{
    character: char,
    cost : i32,
    affinity: Affinity,
}

fn run(input: Residence) -> i32 
{
    let h = House { rooms :
    [
        Room { name: L1, affinity: Affinity::Hallway, family_index: 0, connected_1: vec![L2], connected_2 : vec![]},
        Room { name: L2, affinity: Affinity::Hallway, family_index: 0, connected_1: vec![L1], connected_2 : vec![A4, AB]},

        Room { name: A1, affinity: Affinity::A, family_index: 0, connected_1: vec![A2], connected_2 : vec![]},
        Room { name: A2, affinity: Affinity::A, family_index: 1, connected_1: vec![A1, A3], connected_2 : vec![]},
        Room { name: A3, affinity: Affinity::A, family_index: 2, connected_1: vec![A2, A4], connected_2 : vec![]},
        Room { name: A4, affinity: Affinity::A, family_index: 3, connected_1: vec![A3], connected_2 : vec![L2, AB]},

        Room { name: AB, affinity: Affinity::Hallway, family_index: 0, connected_1: vec![], connected_2: vec![L2, A4, B4, BC]},

        Room { name: B1, affinity: Affinity::B, family_index: 0, connected_1: vec![B2], connected_2 : vec![]},
        Room { name: B2, affinity: Affinity::B, family_index: 1, connected_1: vec![B1, B3], connected_2 : vec![]},
        Room { name: B3, affinity: Affinity::B, family_index: 2, connected_1: vec![B2, B4], connected_2 : vec![]},
        Room { name: B4, affinity: Affinity::B, family_index: 3, connected_1: vec![B3], connected_2 : vec![AB, BC]},

        Room { name: BC, affinity: Affinity::Hallway, family_index: 0, connected_1: vec![], connected_2: vec![AB, B4, C4, CD]},

        Room { name: C1, affinity: Affinity::C, family_index: 0, connected_1: vec![C2], connected_2 : vec![]},
        Room { name: C2, affinity: Affinity::C, family_index: 1, connected_1: vec![C1, C3], connected_2 : vec![]},
        Room { name: C3, affinity: Affinity::C, family_index: 2, connected_1: vec![C2, C4], connected_2 : vec![]},
        Room { name: C4, affinity: Affinity::C, family_index: 3, connected_1: vec![C3], connected_2 : vec![BC, CD]},

        Room { name: CD, affinity: Affinity::Hallway, family_index: 0, connected_1: vec![], connected_2: vec![BC, C4, D4, R2]},

        Room { name: D1, affinity: Affinity::D, family_index: 0, connected_1: vec![D2], connected_2 : vec![]},
        Room { name: D2, affinity: Affinity::D, family_index: 1, connected_1: vec![D1, D3], connected_2 : vec![]},
        Room { name: D3, affinity: Affinity::D, family_index: 2, connected_1: vec![D2, D4], connected_2 : vec![]},
        Room { name: D4, affinity: Affinity::D, family_index: 3, connected_1: vec![D3], connected_2 : vec![CD, R2]},

        Room { name: R1, affinity: Affinity::Hallway, family_index: 0, connected_1: vec![R2], connected_2: vec![]},
        Room { name: R2, affinity: Affinity::Hallway, family_index: 0, connected_1: vec![R1], connected_2: vec![D4, CD]},
    ]};

    let target_residence = Residence { rooms: 
        [
            None,
            None,

            Some('A'),
            Some('A'),
            Some('A'),
            Some('A'),

            None,

            Some('B'),
            Some('B'),
            Some('B'),
            Some('B'),

            None,

            Some('C'),
            Some('C'),
            Some('C'),
            Some('C'),

            None,

            Some('D'),
            Some('D'),
            Some('D'),
            Some('D'),

            None,
            None,
        ]
    };

    let mut pod_types: HashMap<char, PodType> = HashMap::new();
    pod_types.insert('A', PodType { character : 'A', cost : 1, affinity : Affinity::A});
    pod_types.insert('B', PodType { character : 'B', cost : 10, affinity : Affinity::B});
    pod_types.insert('C', PodType { character : 'C', cost : 100, affinity : Affinity::C});
    pod_types.insert('D', PodType { character : 'D', cost : 1000, affinity : Affinity::D});

    let sr = ScoredResidence { current : input, goal : target_residence, score: 0 };

    let mut priority_queue = PriorityQueue::new();
    let mut explored : HashSet<Residence> = HashSet::new();

    let score = -sr.score;
    priority_queue.push(sr, score);
    while !priority_queue.is_empty()
    {
        let to_check = priority_queue.pop().unwrap();

        println!("{} priority", -to_check.1);

        if to_check.0.winner() 
        {
            return -to_check.1;
        } 
        else if explored.contains(&to_check.0.current) 
        {
            // do nothing    
        } 
        else 
        {
            let ancestors = find_all_ancestors(&to_check.0, &pod_types, &h);
            for a in ancestors 
            {
                let score = -a.score;
                priority_queue.push(a, score);
            }

            explored.insert(to_check.0.current);
        }
    }
    unreachable!();
}


pub fn day_23(lines: Vec<String>) 
{

    
    // #############
    // #...........#
    // ###C#C#B#D###
    //   #D#A#B#A#
    //   #########

    let actual = Residence { rooms: 
        [
            None,
            None,

            Some('D'),
            Some('D'),
            Some('D'),
            Some('C'),

            None,

            Some('A'),
            Some('B'),
            Some('C'),
            Some('C'),

            None,

            Some('B'),
            Some('A'),
            Some('B'),
            Some('B'),

            None,

            Some('A'),
            Some('C'),
            Some('A'),
            Some('D'),

            None,
            None,
        ]
    };

    // #############
    // #...........#
    // ###B#C#B#D###
    //   #D#C#B#A#
    //   #D#B#A#C#
    //   #A#D#C#A#
    //   #########

    let test = Residence { rooms: 
        [
            None,
            None,

            Some('A'),
            Some('D'),
            Some('D'),
            Some('B'),

            None,

            Some('D'),
            Some('B'),
            Some('C'),
            Some('C'),

            None,

            Some('C'),
            Some('A'),
            Some('B'),
            Some('B'),

            None,

            Some('A'),
            Some('C'),
            Some('A'),
            Some('D'),

            None,
            None,
        ]
    };


    println!("{} answer", run(actual));

}

fn find_all_ancestors(sr: &ScoredResidence, pt: &HashMap<char, PodType>, h: &House) -> Vec<ScoredResidence> 
{
    h.rooms.iter().
    map(|r| r.name).
    flat_map(|n| find_room_ancestors(n, &sr, pt, h)).
    collect()
}



fn find_room_ancestors(room: i32, sr: &ScoredResidence, pt: &HashMap<char, PodType>, h: &House) -> Vec<ScoredResidence>
{
    if sr.current.rooms[room as usize].is_none() 
    {
        return vec![];
    }

    let resident_character = sr.current.rooms[room as usize].unwrap();

    let distance_map = find_reachable_with_distance(room, &sr, h);

    if h.rooms[room as usize].affinity == Affinity::Hallway 
    {
        let target = target_affinity(resident_character).unwrap();
        let affine_rooms : Vec<i32> = h.by_affinity(&target).iter().map(|rm| rm.name).collect();

        // if there are any nonmatching creatures in those hallways, we won't consider moving in.
        let permitted = affine_rooms.iter().all(|rm| sr.current.rooms[*rm as usize].is_none() || sr.current.rooms[*rm as usize].unwrap() == resident_character);
        if !permitted 
        {
            return vec![];
        }

        let reachable_affine_rooms : Vec<i32> = distance_map.keys().filter(|k| affine_rooms.contains(k)).map(|k| *k).collect();

        return reachable_affine_rooms.iter().map(|rr| sr.ancestor(resident_character, room, *rr, distance_map[rr] * pt.get(&resident_character).unwrap().cost)).collect()
    } 
    else 
    {
        // if this room and all preivous rooms are full of matching creatures, we do not need to consider moving this creature
        // if there are any nonmatching we still have to check it.
        let current_room = &h.rooms[room as usize];
        let all_previous_complete = h.by_affinity(&current_room.affinity).iter().
        filter(|rm| rm.family_index <= room)
        .all(|rm| sr.current.rooms[rm.name as usize].is_some() && sr.current.rooms[rm.name as usize].unwrap() == affinity_as_char(&current_room.affinity).unwrap());

        if all_previous_complete 
        {
            return vec![]
        }

        // all reachable hallway rooms
        let hallway_rooms : Vec<i32> = h.by_affinity(&Affinity::Hallway).iter().map(|rm| rm.name).collect();
        let reachable_hallway_rooms : Vec<i32> = distance_map.keys().filter(|k| hallway_rooms.contains(k)).map(|k| *k).collect();
        return reachable_hallway_rooms.iter().map(|rr| sr.ancestor(resident_character, room, *rr, distance_map[rr] * pt.get(&resident_character).unwrap().cost)).collect()
    }
}

fn find_reachable_with_distance(room: i32, sr: &ScoredResidence, h: &House) -> HashMap<i32, i32>
{
    let mut to_return = HashMap::new();

    for c in &h.rooms[room as usize].connected_1 
    {
        find_reachable_recursive(1, *c, &sr, h, &mut to_return);
    }

    for c in &h.rooms[room as usize].connected_2 
    {
        find_reachable_recursive(2, *c, &sr, h, &mut to_return);
    }

    return to_return;
}

fn find_reachable_recursive(cost: i32, room: i32, sr: &ScoredResidence, h: &House, distance_map : &mut HashMap<i32, i32>) 
{
    if sr.current.rooms[room as usize].is_some() 
    {
        return;
    }

    if distance_map.contains_key(&room) && distance_map.get(&room).unwrap() < &cost
    {
        return;
    } 
    else 
    {
        distance_map.insert(room, cost);
    }

    for c in &h.rooms[room as usize].connected_1 
    {
        find_reachable_recursive(1+cost, *c, sr, h, distance_map);
    }

    for c in &h.rooms[room as usize].connected_2 
    {
        find_reachable_recursive(2+cost, *c, sr, h, distance_map);
    }
}


// inp w

// Block 1:
// mul x 0
// add x z
// mod x 26
// add x special_2
// eql x w
// eql x 0
// (x = {0, 1})

// div z special_1

// Block 2:
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y

// multiply z by 1 or 26

// Block 3:
// mul y 0
// add y w
// add y special_3 (always positive)
// mul y x

fn psuedo(oldPath : &Vec<char>, w: i32, z: i32, special_1: i32, special_2: i32, special_3: i32) -> (Vec<char>, i32)
{
    let x = if (z % 26 + special_2) != w { 1 } else { 0 };
    
    let y1 = (x * 25) + 1;
    let y2 = (w + special_3) * x;

    let z_out = ((z / special_1).saturating_mul(y1)).saturating_add(y2);

    let mut newPath = oldPath.clone();
    newPath.push(*w.to_string().chars().collect::<Vec<char>>().get(0).unwrap());

    return (newPath, z_out);
}
