use std::collections::HashSet;

struct BoolMap_2D 
{
    x_range : (i32, i32),
    y_range : (i32, i32),
    
    outside_range_polarity : bool,

    inner : HashSet<(i32, i32)>
}

impl BoolMap_2D 
{
    fn new() -> Self 
    {
        BoolMap_2D 
        {
            x_range : (0,0),
            y_range : (0,0),

            outside_range_polarity : false,

            inner : HashSet::new()
        }    
    }

    fn get(&self, pt:(i32, i32)) -> bool 
    {
        if 
        pt.0 >= self.x_range.0 && pt.0 <= self.x_range.1 &&
        pt.1 >= self.y_range.0 && pt.1 <= self.y_range.1 
        {   
            return self.inner.contains(&pt);
        } 
        else {
            return self.outside_range_polarity;
        }
    }

    fn set(&mut self, pt:(i32, i32), value:bool) 
    {
        if pt.0 < self.x_range.0 
        {
            self.x_range.0 = pt.0;
        }

        if pt.0 > self.x_range.1 
        {
            self.x_range.1 = pt.0;
        }

        if pt.1 < self.y_range.0 
        {
            self.y_range.0 = pt.1;
        }

        if pt.1 > self.y_range.1 
        {
            self.y_range.1 = pt.1;
        }

        if value 
        {
            self.inner.insert(pt);
        } 
        else 
        {
            self.inner.remove(&pt);
        }
    }

    fn kernel(&self, pt:(i32, i32)) -> i32 
    {
        let directions = 
        [(-1, -1), (0, -1), (1,-1),
         (-1, 0), (0, 0), (1, 0),
         (-1, 1), (0, 1), (1, 1)];

        let binary_string = directions.to_vec().into_iter().
        map(|d| (d.0 + pt.0, d.1 + pt.1)).
        map(|p| if self.get(p) { '1' } else { '0' }).
        collect::<String>();

        return isize::from_str_radix(&binary_string, 2).unwrap() as i32;
    }

    fn increment_map(&self, code: &Vec<bool>) -> BoolMap_2D 
    {
        let mut to_return = BoolMap_2D::new();
        for ix in ((0+self.x_range.0-1)..(self.x_range.1+2)) 
        {
            for iy in ((0+self.y_range.0-1)..(self.y_range.1+2)) 
            {
                let k = self.kernel((ix, iy));
                let value = code.get(k as usize).unwrap();
                if *value 
                {
                    to_return.set((ix, iy), true);
                }
            }
        }

        if !self.outside_range_polarity 
        {
            to_return.outside_range_polarity = *code.get(0).unwrap();
        } 
        else 
        {
            to_return.outside_range_polarity = *code.get(511).unwrap();
        }

        return to_return;
    }


}

pub fn day_20(lines: Vec<String>)
{
    let mut lines_iter = lines.into_iter();
    let code = lines_iter.next().unwrap();
    let code_parsed = code.chars().map(|c| c == '#').collect();

    lines_iter.next(); // consume empty line

    let mut old_map = BoolMap_2D::new();

    for (iy, line) in lines_iter.enumerate() 
    {
        for (ix, char) in line.chars().enumerate()
        {
            old_map.set((ix as i32, iy as i32), char == '#');
        }
    }

    for i in 0..50 
    {
        old_map = old_map.increment_map(&code_parsed);
    }

    println!("{} count", old_map.inner.len());
}