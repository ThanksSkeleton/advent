pub fn day_24(lines: Vec<String>)
{
    let answer : Vec<char> = (1..10).map(|w| psuedo(&Vec::new(), w, 0, 1, 11, 5)).
    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 1, 13, 5))).
    filter(|(p,z)| bounds(7, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 1, 12, 1))).
    filter(|(p,z)| bounds(7, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 1, 15, 15))).
    filter(|(p,z)| bounds(7, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 1, 10, 2))).
    filter(|(p,z)| bounds(7, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 26, -1, 2))).
    filter(|(p,z)| bounds(6, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 1, 14, 5))).
    filter(|(p,z)| bounds(6, *z)).
    
    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 26, -8, 8))).
    filter(|(p,z)| bounds(5, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 26, -7, 14))).
    filter(|(p,z)| bounds(4, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 26, -8, 12))).
    filter(|(p,z)| bounds(3, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 1, 11, 7))).
    filter(|(p,z)| bounds(3, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 26, -2, 14))).
    filter(|(p,z)| bounds(2, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 26, -2, 13))).
    filter(|(p,z)| bounds(1, *z)).

    flat_map(|(p, z)| (1..10).map(move |w| psuedo(&p, w, z, 26, -13, 6))).
    filter(|(p, z)| z == &0).
    next().unwrap().
    0;

    println!("{} answer", answer.iter().collect::<String>());
}

fn bounds(pow: i32, to_check: i32) -> bool 
{
    return to_check as f32 <= (26.0 as f32).powf(pow as f32);
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
