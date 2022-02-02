use std::collections::HashSet;

// const X_START : i32 = 119;
// const X_END: i32 = 176;

// const Y_START: i32 = -141;
// const Y_END : i32 = -84;

struct Ranges 
{
    X_START : i32, 
    X_END: i32, 
    
    Y_START: i32, 
    Y_END : i32,
}


#[derive(Debug)]
struct Hit 
{
    speed: i32,

    hit_times: Vec<i32>,

    time_floor: Option<i32>
}

pub fn day_17()
{
    let r_actual = Ranges 
    {    
        X_START : 119,
        X_END: 176,
        
        Y_START: -141,
        Y_END : -84
    };

    let r_test = Ranges 
    {    
        X_START : 20,
        X_END: 30,
        
        Y_START: -10,
        Y_END : -5
    };

    run(&r_actual);

}

fn run(r: &Ranges) 
{
    let collision_info = find_x_collisions(&r);
    //println!("{:?} x hits", collision_info);

    //let collision_info = vec![Hit{ speed : 6, hit_times: vec![6,7], time_floor: Some(7) }];

    let all_collisions = find_y_collisions(&r,&collision_info);
    //println!("{:?} found", all_collisions);
    println!("{}", all_collisions.len())
}


fn find_x_collisions(r : &Ranges) -> Vec<Hit>
{
    return (1..r.X_END+1).into_iter().
    map(|speed| find_hits(r,speed)).
    filter(|h| h.hit_times.len() > 0).
    collect();
}

fn find_y_collisions(r: &Ranges, hits : &Vec<Hit>) -> Vec<(i32, i32)> 
{
    return (r.Y_START..(-r.Y_START+2)).into_iter().rev().
    flat_map(|y_speed| find_hits_y(r, y_speed, &hits)).
    collect();
}


fn find_hits(r: &Ranges, speed : i32) -> Hit
{
    let mut specific_hit_times = Vec::new();
    let mut time_floor = None;

    let mut position = 0;
    for (i, current_speed) in (0..speed+1).rev().enumerate() 
    {
        position = position + current_speed;
        if position > r.X_END 
        {
            break;
        } 
        else if position >= r.X_START 
        {   
            let hit_time = (i+1) as i32;     
            specific_hit_times.push(hit_time);

            if current_speed == 0
            {
                if time_floor.is_none() 
                {
                    time_floor = Some(hit_time);
                } 
                else 
                {
                    time_floor = Some(if hit_time < time_floor.unwrap() { hit_time } else { time_floor.unwrap() })
                }
            }
        }
    }

    return Hit { speed, hit_times: specific_hit_times, time_floor };
}

fn find_hits_y(r: &Ranges, y_speed: i32, hits :&Vec<Hit>) -> Vec<(i32, i32)> 
{
    return hits.iter().flat_map(|h| find_hits_y_inner(r, y_speed, h)).collect();
}

fn find_hits_y_inner(r: &Ranges, y_speed: i32, hit :&Hit) -> Option<(i32, i32)>
{
    //println!("Checking if {} y speed hits a target time", y_speed);
        
    let mut current_time = 0;
    let mut current_speed = y_speed;
    let mut position = 0;
    let mut max_position = 0;

    loop
    {
        position = position + current_speed;
        max_position = if position > max_position { position } else { max_position };

        current_speed = current_speed - 1;
        current_time = current_time + 1;

        if position < r.Y_START 
        {
            //println!("Fell out.");
            return None;
        } 
        else if position <= r.Y_END 
        {
            if hit.time_floor.is_some() && hit.time_floor.unwrap() <= current_time 
            {
                //println!("speed {} is in the area, with a time of {} - above floor", y_speed, current_time);
                return Some((hit.speed, y_speed));
            } 
            else if hit.hit_times.contains(&current_time)
            {                
                //println!("speed {} is in the area, with a time of {}", y_speed, current_time);
                return Some((hit.speed, y_speed));
            }
            else 
            {
                //println!("speed {} is in the area, but at the wrong time {}", y_speed, current_time);
            }
        }
    }
}
