use std::collections::HashSet;
use cgmath::{Deg, Euler, Quaternion, Vector3};

#[derive(Debug, Copy, Clone)]
struct Transform 
{
    rotation : Quaternion<f32>,
    translation : Vector3<i32>
}

impl Transform 
{
    fn apply_to_point(&self, v : Vector3<i32>) -> Vector3<i32> 
    {
        return integer_rotate(v, self.rotation) + self.translation;
    }
}

fn get_transformations(a1 : Vector3<i32>, a2: Vector3<i32>, b1: Vector3<i32>, b2: Vector3<i32>) -> Vec<Transform> 
{
    println!("Finding plausible transforms that map a1: {:?} onto b1: {:?} and a2: {:?} onto b2: {:?}", a1, b1, a2, b2);
    let r1 = rotate_onto(a1, b1);
    println!("rotate onto: {:?}", r1);
    let i1 = r1.into_iter().filter(|t| maps_given_transform(a2, b2, t));

    println!("Finding plausible transforms that map a1: {:?} onto b2: {:?} and a2: {:?} onto b1: {:?}", a1, b2, a2, b1);
    let r2 = rotate_onto(a1, b2);
    println!("rotate onto: {:?}", r2);
    let i2 = r2.into_iter().filter(|t| maps_given_transform(a2, b1, t));

    return i1.chain(i2).collect();
}

fn rotate_onto(a: Vector3<i32>, b: Vector3<i32>) -> Vec<Transform> 
{
    return get_all_rotations().into_iter().map(|q| Transform{ rotation: q, translation: a - integer_rotate(b , q)}).collect();
}

fn maps_given_transform(a: Vector3<i32>, b: Vector3<i32>, t : &Transform) -> bool 
{
    println!("Testing transform: {:?} == {:?} ({:?})", a, t.apply_to_point(b), b);

    return a == t.apply_to_point(b);
}

fn get_all_rotations() -> Vec<Quaternion<f32>> 
{
    let mut toReturn = Vec::new();
    for x in [0.0, 90.0, 180.0, 270.0] 
    {
        for y in [0.0, 90.0, 180.0, 270.0] 
        {
            for z in [0.0, 90.0, 180.0, 270.0] 
            {
                let rotation = Quaternion::from(Euler {
                    x: Deg(x),
                    y: Deg(y),
                    z: Deg(z),
                });
                toReturn.push(rotation);
            }
        }
    }
    return toReturn;
}

fn integer_rotate(v : Vector3<i32>, r: Quaternion<f32>) -> Vector3<i32> 
{
    let f = Vector3::<f32>{ x: v.x as f32, y: v.y as f32, z: v.z as f32};
    let rotated = r * f;
    return Vector3::<i32>{ x: rotated.x.round() as i32, y: rotated.y.round() as i32, z: rotated.z.round() as i32 };
}

#[derive(Debug)]
struct TriangluarArray_i32 
{
    inner: Vec<Vec<i32>>
}

impl TriangluarArray_i32 
{
    pub fn new(dim: usize) -> Self 
    {
        if dim == 0 || dim == 1 
        {
            panic!("{} too small for TriangluarArray_i32 dimension", dim);
        }

        let mut toReturn : Vec<Vec<i32>> = Vec::new();
        for i in (1..dim).rev()
        {
            toReturn.push(vec![0; i]);
        }

        return Self 
        {
            inner: toReturn
        }
    }

    fn dim(&self) -> usize 
    {
        self.inner.len()-1
    }

    fn get(&self, key : (usize, usize)) -> i32 
    {
        let x = key.0;
        let y = key.1;

        if x == y 
        {
            panic!("Invalid Get {}=={}", x, y);
        }

        if x > y
        {
            return self.get((y, x));
        }

        return *self.inner.get(x).unwrap().get(y-(x+1)).unwrap();
    }

    fn set(&mut self, key : (usize, usize) , input: i32) 
    {
        let x = key.0;
        let y = key.1;

        if x == y 
        {
            panic!("Invalid Set {}=={}", x, y);
        }

        if x > y
        {
            self.set((y, x), input);
            return;
        }

        let mut mut_x = self.inner.get_mut(x).unwrap();
        let mut mut_value = mut_x.get_mut(y-(x+1)).unwrap();
        *mut_value = input;
    }

    fn keys(&self) -> Vec<(usize, usize)> 
    {
        let mut to_return : Vec<(usize, usize)> = Vec::new();
        for x in 0..self.dim()-1 
        {
            for y in 1..self.dim() 
            {
                to_return.push((x, y));
            }
        }

        return to_return;
    }
 
    fn build_all_dist_packages(&self) -> Vec<DistPackage> 
    {
        return (0..self.dim()+2).into_iter().map(|i| self.build_dist_package(i)).collect();
    }

    fn build_dist_package(&self, i: usize) -> DistPackage
    {
        let mut z : Vec<(usize, i32)> = (0..self.dim()+2).into_iter().filter(|x| *x != i).map(|x| (x, self.get((i, x)))).collect();
        z.sort_unstable_by(|v1, v2| v1.1.partial_cmp(&v2.1).unwrap());
        return DistPackage { starting_index: i, ending_indexes_and_distances: z };
    }

}

fn integer_magnitude(v : Vector3<i32>) -> i32 
{
    return ((v.x * v.x + v.y * v.y + v.z * v.z) as f32).sqrt().round() as i32
}

fn build_distances(points : &Vec<Vector3<i32>>) -> TriangluarArray_i32 
{
    let mut to_return = TriangluarArray_i32::new(points.len());
    for (ia, a) in points.iter().enumerate()
    {
        for (ib, b) in points.iter().enumerate()
        {
            if ia != ib 
            {
                to_return.set((ia, ib), integer_magnitude(a-b));
            }
        }
    }
    return to_return;
}

#[derive(Debug)]
struct DistPackage 
{
    starting_index : usize,
    ending_indexes_and_distances: Vec<(usize, i32)>
}

impl DistPackage 
{
    fn overlaps(&self, other: DistPackage, threshhold : i32) -> Option<((usize, usize), (usize, usize))> 
    {
        let mut endMatch : Option<((usize, usize), (usize, usize))>  = None;
        let mut matchCount = 0;
        for ei_d in self.ending_indexes_and_distances.iter() 
        {
            let matching = other.ending_indexes_and_distances.iter().filter(|o| o.1 == ei_d.1).next();
            if matching.is_some() 
            {
                endMatch = Some(((self.starting_index, other.starting_index), (ei_d.0, matching.unwrap().0)));
                matchCount = matchCount + 1;
            }
        }

        if matchCount >= (threshhold-1)
        {   
            println!("{} {}", matchCount, (threshhold-1));
            return endMatch;
        } 
        else 
        {
            //println!("Threshold {}", matchCount);
            return None;
        }
    }
}


#[derive(Debug)]
struct ProbeMap 
{
    name: i32,

    origin: Option<Vector3<i32>>,
    points: Vec<Vector3<i32>>,
    distances: TriangluarArray_i32,
    match_minimum_count : i32,
}

impl ProbeMap 
{
    fn build(name: i32, points : Vec<Vector3<i32>>, match_minimum : i32) -> ProbeMap 
    {
        ProbeMap 
        {
            name: name,

            origin : None,            
            distances: build_distances(&points),
            points : points,
            match_minimum_count : match_minimum
        }
    }


    fn transform_mut(&mut self, t: Transform) 
    {
        self.origin = Some(t.apply_to_point(Vector3::new(0,0,0)));
        for pt in self.points.iter_mut()
        {
            *pt = t.apply_to_point(*pt);
        }
    }

    fn overlaps_with(&self, other: &ProbeMap) -> Option<Transform> 
    {                
        println!("checking if #{} overlaps with #{}", self.name, other.name);
        for my_dist_pkg in self.distances.build_all_dist_packages() 
        {
            for other_dist_pkg in other.distances.build_all_dist_packages() 
            {
                println!("checking between index {:?} and index {:?}", my_dist_pkg.starting_index, other_dist_pkg.starting_index);


                let connected = my_dist_pkg.overlaps(other_dist_pkg, self.match_minimum_count);

                println!("connected: {:?}", connected);

                if connected.is_some() 
                {           
                    let c_unwrapped = connected.unwrap();

                    for t in get_transformations(
                            *self.points.get(c_unwrapped.0.0).unwrap(),                            
                            *self.points.get(c_unwrapped.0.1).unwrap(),
                            *other.points.get(c_unwrapped.1.0).unwrap(),
                            *other.points.get(c_unwrapped.1.1).unwrap()
                            ) 
                    {
                        let mut testStarMap = HashSet::new();
                        for point in self.points.iter() 
                        {
                            testStarMap.insert(*point);
                        }

                        for point in other.points.iter() 
                        {
                            testStarMap.insert(t.apply_to_point(*point));
                        }

                        let no_overlap_length = self.points.len() + other.points.len();
                        let length_with_overlaps = testStarMap.len();

                        println!("{} total, {} with overlaps, {} threshold", no_overlap_length, length_with_overlaps, self.match_minimum_count);

                        if length_with_overlaps <= no_overlap_length - self.match_minimum_count as usize 
                        {
                            return Some(t);
                        }
                    }
                }
            }
        }

        return None;
    }



}

pub fn day_19(lines: Vec<String>, overlap_count : i32)
{
    let mut probe_maps_todo = Vec::new();
    let mut line_iter = lines.into_iter().peekable();
    let mut i = 0;
    while line_iter.peek().is_some()
    {
        line_iter.next(); // consume title
        let mut points = Vec::new();
        while line_iter.peek().is_some() && !line_iter.peek().unwrap().is_empty()
        {
            let string = line_iter.next().unwrap();
            //println!("parse {:?}", string);

            let mut split = string.split(",");
            points.push(Vector3::<i32> 
            {
                x: split.next().unwrap().parse::<i32>().unwrap(),
                y: split.next().unwrap().parse::<i32>().unwrap(),
                z: split.next().unwrap().parse::<i32>().unwrap()
            });
        }            
        line_iter.next(); // consume empty line;
        probe_maps_todo.push(ProbeMap::build(i, points, overlap_count));
        i = i + 1;
    }

    for p in probe_maps_todo.iter() 
    {
        println!("{:?}", p);
    }

    let mut probe_maps_done = Vec::new();
    let mut probe_maps_to_check = vec![probe_maps_todo.pop().unwrap()];

    while probe_maps_to_check.len() > 0 
    {        
        let current_target = probe_maps_to_check.pop().unwrap();

        let partition : (Vec<(Option<Transform>, ProbeMap)>, Vec<(Option<Transform>, ProbeMap)>) = 
        probe_maps_todo.into_iter().
        map(|pm| (current_target.overlaps_with(&pm), pm)).  
        partition(|(t, _pm)| t.is_some());

        println!("there are {} overlaps and {} non-overlaps", partition.0.len(), partition.1.len());

        probe_maps_todo = partition.1.into_iter().map(|(_t, pm)| pm).collect();

        let mut brand_new_maps: Vec<(Transform, ProbeMap)>  = partition.0.into_iter().map(|(t, pm)| (t.unwrap(), pm)).collect();

        for (t, pm) in brand_new_maps.iter_mut() 
        {
            pm.transform_mut(*t);
        }

        for (_t, pm) in brand_new_maps.into_iter()
        {
            probe_maps_to_check.push(pm);
        }

        probe_maps_done.push(current_target);
    }

    if probe_maps_todo.len() > 0 
    {
        println!("Error: There are remaining maps to link but no current candidates");
        return;
    }

    let mut final_star_count : HashSet<Vector3<i32>> = HashSet::new();

    println!("Probe maps Done: {:?}", probe_maps_done);

    for pm in probe_maps_done 
    {
        for point in pm.points 
        {
            final_star_count.insert(point);
        }
    }

    println!("{:?} final stars:", final_star_count);

    println!("{:?} final star count", final_star_count.len());
    // let pm_zero = probe_maps.get(0).unwrap();
    // let pm_one = probe_maps.get(1).unwrap();

    // let overlap = pm_one.overlaps_with(pm_zero);
    // println!("{:?} overlap", overlap);

}
