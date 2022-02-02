use crate::HashSet;
use crate::HashMap;

struct Octo_Map 
{
    dim_x : i32,
    dim_y : i32,

    inner : HashMap<(i32, i32), i32>,

    previous_flash_count : i32,
    flash_count : i32
}

impl Octo_Map 
{
    fn new(dim_x : i32, dim_y : i32) -> Octo_Map 
    {
        return Octo_Map 
        {
            dim_x,
            dim_y,

            inner : HashMap::new(),
            
            previous_flash_count: 0,
            flash_count: 0
        }
    }

    fn get(&self, pt : &(i32, i32)) -> i32 
    {
        return *self.inner.get(&pt).unwrap();
    }

    fn set(&mut self, pt : &(i32, i32), value: i32) 
    {
        self.inner.insert(*pt, value);
    }

    fn bump(&mut self, pt : &(i32, i32)) 
    {
        let old = self.get(pt);
        self.set(pt, old+1);
    }

    fn directions() -> Vec<(i32, i32)> 
    {
        vec!
        [(-1, -1), (0, -1), (1, -1),
         (-1, 0),           (1, 0),
         (-1,  1), (0, 1), (1, 1)]
    }

    fn update(&mut self)
    {
        self.previous_flash_count = 0;

        let keys :Vec<(i32, i32)> = self.inner.keys().map(|pt| *pt).collect();

        for k in keys.iter()
        {
            self.bump(&k);
        }

        let mut set = HashSet::new();
        self.update_recursive(keys, &mut set);

        for k in set 
        {
            self.set(&k, 0);
        }

    }

    fn update_recursive(&mut self, keys_to_check : Vec<(i32, i32)>, flashes : &mut HashSet<(i32, i32)>)
    {
        let to_flash: Vec<&(i32, i32)> = keys_to_check.iter().
        filter(|pt| self.get(pt) >= 10).
        filter(|pt| !flashes.contains(pt)).collect();

        let mut to_check : Vec<(i32, i32)> = to_flash.iter().
        flat_map(|pt| self.flash(**pt, flashes))
        .collect();

        let to_check_hashset: HashSet<(i32, i32)> = to_check.drain(..).collect();

        if to_check_hashset.len() > 0 
        {
            self.update_recursive(to_check_hashset.into_iter().collect(), flashes);
        }
    }

    fn flash(&mut self, pt: (i32, i32), flashes : &mut HashSet<(i32, i32)>) -> Vec<(i32, i32)> 
    {
        //println!("flash {:?}", pt);
        let dim_x = self.dim_x;
        let dim_y = self.dim_y;

        flashes.insert(pt);

        self.flash_count = self.flash_count+1;
        self.previous_flash_count = self.previous_flash_count+1;

        return Octo_Map::directions().into_iter().
        map(|d| (d.0 + pt.0, d.1 + pt.1)).
        filter(|(x, y)| x >= &0 && x < &dim_x && y >= &0 && y < &dim_y).
        map(|pt| { self.bump(&pt); return pt })
        .collect();
    }

    fn print(&self) 
    {

    }

}

pub fn day_11(lines : Vec<String>)
{
    let dim_y = lines.len();
    let dim_x = lines.get(0).unwrap().len();

    let mut octo = Octo_Map::new(dim_x as i32, dim_y as i32);

    for (iy, line) in lines.into_iter().enumerate() 
    {
        for (ix, char) in line.chars().enumerate() 
        {
            octo.set(&(ix as i32, iy as i32), char.to_string().parse::<i32>().unwrap())
        }
    }

    for i in 1.. 
    {
        octo.update();
        if octo.previous_flash_count == dim_x as i32 * dim_y as i32
        {
             println!("finished: {}", i);
             return;
        }
    }




}