use crate::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Player_Info 
{
    player_index: usize,

    position: i32,
    score : i32
}

impl Player_Info 
{
    fn increment(&mut self, roll: i32) 
    {
        self.position = self.position + roll;
        self.score = self.score + position(self.position);
    }

    fn winner(&self) -> bool
    {
        self.score >= 21
    }
}

fn position(p: i32) -> i32 
{
    (p % 10) + 1
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct WorldState 
{
    active : Player_Info,
    passive : Player_Info,
}

impl WorldState 
{
    // Swap players
    fn reverse(&self) -> WorldState 
    {
        WorldState { active : self.passive, passive : self.active }
    }
}

pub fn day_21(lines: Vec<String>)
{
    println!("4 7 {} result", result(3,6));
    //println!("4 8 {} result", result(3,7));
}

fn result(p1_pos: i32, p2_pos: i32) -> i64 
{
    let mut p1 = Player_Info { player_index: 0, position : p1_pos, score: 0 };
    let mut p2 = Player_Info { player_index: 1, position : p2_pos, score: 0 };

    let mut memo : HashMap<WorldState, [i64; 2]> = HashMap::new();
    let mut w = WorldState { active: p1, passive: p2 };

    let a = recursive_roll(w.clone(), 1, 2, &mut memo);
    let b = recursive_roll(w.clone(), 2, 2, &mut memo);
    let c = recursive_roll(w.clone(), 3, 2, &mut memo);

    let answer = [a[0] + b[0]+ c[0], a[1] + b[1] + c[1]];

    if answer[0] > answer[1] { return answer[0] } else { answer[1] }
}

fn recursive_roll(mut world_state: WorldState, total_roll: i32, rolls_remaining: i32, memo : &mut HashMap<WorldState, [i64; 2]>) -> [i64; 2]
{
    if rolls_remaining == 0 
    {
        world_state.active.increment(total_roll);

        //println!("World state is {:?}", world_state);

        if world_state.active.winner() 
        {
            let mut to_return = [0,0];
            to_return[world_state.active.player_index] = 1;
            return to_return;
        } 
        else if memo.contains_key(&world_state)
        {
            return *memo.get(&world_state).unwrap();
        } 
        else 
        {
            let a = recursive_roll(world_state.reverse(), 1, 2, memo);
            let b = recursive_roll(world_state.reverse(), 2, 2, memo);
            let c = recursive_roll(world_state.reverse(), 3, 2, memo);

            let answer = [a[0] + b[0]+ c[0], a[1] + b[1] + c[1]];

            memo.insert(world_state, answer);

            return answer;
        }
    } 
    else 
    {
        let a = recursive_roll(world_state.clone(), total_roll + 1, rolls_remaining-1, memo);
        let b = recursive_roll(world_state.clone(), total_roll + 2, rolls_remaining-1, memo);
        let c = recursive_roll(world_state.clone(), total_roll + 3, rolls_remaining-1, memo);

        let answer = [a[0] + b[0]+ c[0], a[1] + b[1] + c[1]];

        return answer;
    }
}