use std::iter::FlatMap;
use std::ops::Range;
use std::collections::HashSet;

#[derive(Debug)]
struct Command 
{
    on_off : bool,

    triple: RangeTriple
}

impl Command 
{
    fn new(s: String) -> Command 
    {
        let mut split1 = s.split(" ");
        let on_off = split1.next().unwrap() == "on";
        let triple = RangeTriple::new(split1.next().unwrap());
        return Command 
        {
            on_off,
            triple
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct MyRange 
{
    start: i64,
    end : i64,
}

impl MyRange 
{
    fn as_range(&self) -> Range<i64> 
    {
        self.start..self.end+1
    }

    fn range_size(&self) -> i64 
    {
        if self.start > self.end 
        {
            return 0;
        } 
        else
        {
            return (self.end - self.start + 1) as i64
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct RangeTriple 
{
    x_range : MyRange,
    y_range : MyRange,
    z_range : MyRange,
}

impl RangeTriple 
{
    fn new(s: &str) -> RangeTriple 
    {
        let mut split2 = s.split(",");
        return RangeTriple 
        {
            x_range : RangeTriple::parse_range(split2.next().unwrap()),
            y_range : RangeTriple::parse_range(split2.next().unwrap()),
            z_range : RangeTriple::parse_range(split2.next().unwrap()),
        }
    }

    fn new_option(x_range: Option<MyRange>, y_range: Option<MyRange>, z_range: Option<MyRange>) -> Option<RangeTriple> 
    {
        if x_range.is_some() && y_range.is_some() && z_range.is_some() 
        {
            Some(RangeTriple { x_range : x_range.unwrap(), y_range : y_range.unwrap(), z_range: z_range.unwrap() })
        } 
        else 
        {
            None
        }
    }

    fn parse_range(s: &str) -> MyRange
    {
        let mut split = s.split("=");
        split.next(); // throw away x, y or z

        let mut parsed = split.next().unwrap().split("..").map(|d| d.parse::<i64>().unwrap());
        MyRange { start: parsed.next().unwrap(), end: parsed.next().unwrap() }
    }

    fn naive_list(&self) -> Vec<(i64, i64, i64)>
    {
        self.x_range.as_range().flat_map(|x| self.y_range.as_range().flat_map(move |y| self.z_range.as_range().map(move |z| (x, y, z)))).collect()
    }

    fn size(&self) -> i64 
    {
        return &self.x_range.range_size() * &self.y_range.range_size() * &self.z_range.range_size();
    }

    fn slice(slicer: &RangeTriple, slicee: &RangeTriple) -> Vec<RangeTriple> 
    {
        if  overlaps(&slicer.x_range, &slicee.x_range) &&
            overlaps(&slicer.y_range, &slicee.y_range) &&
            overlaps(&slicer.z_range, &slicee.z_range) {

            let all_slices = vec![
                    // z less
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),

                    RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),

                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_less(&slicee.z_range, &slicer.z_range) ),

                    // z same
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),

                    
                    RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),
                    //exclude intersection
                    //RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),             
                    RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),

                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_intersect(&slicee.z_range, &slicer.z_range) ),

                    // z more
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_less(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),

                    RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_intersect(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),

                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_less(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_intersect(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),
                    RangeTriple::new_option(  clamp_more(&slicee.x_range, &slicer.x_range),  clamp_more(&slicee.y_range, &slicer.y_range),  clamp_more(&slicee.z_range, &slicer.z_range) ),
            ];

            //println!("{:?} all", all_slices);

            let slice_result : Vec<RangeTriple>  =  all_slices.into_iter().flatten().collect();

            //println!("{:?} slices", slice_result);

            return slice_result;
        } 
        else 
        {
            return vec![*slicee];
        } 
    }


    // fn intersect(a: &RangeTriple, b: &RangeTriple) -> RangeTriple 
    // {

    // }

    // fn intersect_range(a: Range, b:Range) -> Range
    // {

    // }

}

fn min(a: i64, b: i64) -> i64 
{
    if a < b { a } else { b }
}

fn max(a: i64, b: i64) -> i64 
{
    if a > b { a } else { b }
}


// fn cl_l(input: i64, clamp: i64) -> i64 
// {
//     if input < clamp { input } else { clamp }
// }

// fn cl_h(input: i64, clamp: i64) -> i64
// {
//     if input > clamp { input } else { clamp }
// }

// fn clamp(input : i64, clamp_low: i64, clamp_high: i64) -> i64 
// {
//     return cl_h(cl_l(input, clamp_low), clamp_high);
// }

fn overlaps(a : &MyRange, b: &MyRange) -> bool
{
    return !(a.end < b.start || b.end < a.start)
}


fn clamp_less(input: &MyRange, clamp_value: &MyRange) -> Option<MyRange> 
{
    if input.start < clamp_value.start 
    {
        Some(MyRange { start: input.start, end: min(input.end, clamp_value.start-1)})
    } 
    else 
    {
        None
    }
}

fn clamp_intersect(input: &MyRange, clamp_value: &MyRange) -> Option<MyRange>
{
    if input.start <= clamp_value.end && input.end >= clamp_value.start 
    {
        Some(MyRange { start: max(input.start, clamp_value.start), end: min(input.end, clamp_value.end)})
    } 
    else 
    {
        None
    }
}

fn clamp_more(input: &MyRange, clamp_value: &MyRange) -> Option<MyRange> 
{
    if input.end > clamp_value.end
    {
        Some(MyRange { start: max(input.start, clamp_value.end+1), end: input.end})
    } 
    else 
    {
        None
    }
}

pub fn day_22(lines: Vec<String>)
{
    // let small = RangeTriple { x_range: MyRange { start: 0, end: 1}, y_range: MyRange { start: 1, end: 1}, z_range: MyRange { start: 1, end: 1}};
    // let big = RangeTriple { x_range: MyRange { start: 0, end: 2}, y_range: MyRange { start: 0, end: 2}, z_range: MyRange { start: 0, end: 2}};

    // let sliced = RangeTriple::slice(&small, &big);

    // for slice in sliced.iter() 
    // {
    //     println!("{:?} slice", slice);
    // }

    // let sum : i64 = sliced.iter().map(|rt| rt.size()).sum();

    // println!("{:?} sum", sum);


    let commands : Vec<Command> = lines.into_iter().map(|s| Command::new(s)).collect();

    // let naive = naive(&commands);
    let smart = smart(&commands);

    // println!("{} naive {} smart len", naive, smart)

    println!("{} smart len", smart)

}

fn smart(commands: &Vec<Command>) -> i64 
{
    let mut disjoint_list = vec![commands.get(0).unwrap().triple];

    for command in &commands[1..]
    {
        if command.on_off
        {
            let mut new_slices = iteratively_sliced(&disjoint_list[..], &command.triple);
            println!("Command adds {:?}", new_slices.len());
            disjoint_list.append(&mut new_slices);
            println!("Total size {:?}", disjoint_list.len());

            //println!("Command {:?} adds {:?}", command, new_slices);
        } else {
            disjoint_list = disjoint_list.iter().flat_map(|rt| RangeTriple::slice(&command.triple, &rt)).collect();
            println!("Command sets size to size {:?}", disjoint_list.len());
        }
    }

    return disjoint_list.iter().map(|rt| rt.size()).sum();
}

fn recursively_sliced(disjoint_list : &[RangeTriple], slicee: &RangeTriple) -> Vec<RangeTriple>
{
    if disjoint_list.len() == 0 
    {
        return vec![*slicee];
    }

    RangeTriple::slice(&disjoint_list[0], slicee).into_iter().
    flat_map(|rt| recursively_sliced(&disjoint_list[1..], &rt)).
    collect()
}

fn iteratively_sliced(disjoint_list : &[RangeTriple], slicee: &RangeTriple) -> Vec<RangeTriple> 
{
    let mut to_slice = vec![*slicee];

    for slicer in disjoint_list 
    {
        let mut new_slices = Vec::new();
        for slicee in to_slice 
        {
            new_slices.append(&mut RangeTriple::slice(slicer, &slicee));    
        }
        to_slice = new_slices;
    }

    return to_slice;
}

fn naive(commands : &Vec<Command>) -> i64
{
    let mut set : HashSet<(i64, i64, i64)> = HashSet::new();
    for command in commands 
    {
        for pt in command.triple.naive_list() 
        {        
            if command.on_off 
            {
                set.insert(pt);
            } 
            else 
            {
                set.remove(&pt);
            }
        } 
    }

    return set.len() as i64;
}