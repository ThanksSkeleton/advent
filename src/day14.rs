use std::collections::HashMap;

#[path ="Util.rs"]
mod Util;

#[derive(Clone, Copy)]
struct Rule(char, char, char);

#[derive(Eq, PartialEq, std::hash::Hash, Clone, Copy)]
struct SegmentGrowthKey(i32, char, char);

pub fn day_14(lines : Vec<String>, debug_print : bool){
    let startingString = &lines[0];
    let mut rules = Vec::new();
    for i in 2..lines.len()
    {
        rules.push(Rule(lines[i].chars().skip(0).next().unwrap(), lines[i].chars().skip(1).next().unwrap(), lines[i].chars().skip(lines[i].len() - 1).next().unwrap()));
    }

    let input_string : Vec<char> = startingString.chars().collect();

    let mut memo_map : HashMap<SegmentGrowthKey, HashMap<char, i64>> = HashMap::new();
    let mut total_hashmap: HashMap<char, i64> = HashMap::new();

    for i in 0..input_string.len()-1 
    {
        let first = input_string.get(i).unwrap();
        let second  = input_string.get(i+1).unwrap();

        let returnMap =polymer_recursive_memo(SegmentGrowthKey(40, *first, *second), &mut memo_map, &rules);
        if (i == 0)
        {
            total_hashmap = returnMap;
        } 
        else 
        {
            total_hashmap = Merge_HashMaps(total_hashmap, returnMap, *first);
        }
    }

    let max = total_hashmap.values().into_iter().map(|x| *x).reduce(i64::max).unwrap();
    let min = total_hashmap.values().into_iter().map(|x| *x).reduce(i64::min).unwrap();

    println!("{}", max-min);
}

fn polymer_recursive_memo(sg_key : SegmentGrowthKey, sg_map : &mut HashMap<SegmentGrowthKey, HashMap<char, i64>>, base_rules : &Vec<Rule>) -> HashMap<char, i64>
{
    if (sg_map.contains_key(&sg_key))
    {
        return sg_map.get(&sg_key).unwrap().clone();    
    } 
    else if (sg_key.0 == 0)
    {
        return Bottom_Level_HashMap(sg_key.1, sg_key.2);
    } 
    else
    {
        println!("Finding {} {} {}", sg_key.0, sg_key.1, sg_key.2);

        for r in base_rules.into_iter()
        {
            if (r.0 == sg_key.1 && r.1 == sg_key.2)
            {
                
                let recursive_call_1 = polymer_recursive_memo(SegmentGrowthKey(sg_key.0-1, sg_key.1, r.2), sg_map, base_rules);
                let recursive_call_2 = polymer_recursive_memo(SegmentGrowthKey(sg_key.0-1, r.2, sg_key.2), sg_map, base_rules);

                let merged = Merge_HashMaps(recursive_call_1, recursive_call_2, r.2);

                sg_map.insert(sg_key.clone(), merged);

                // wtf? do it again?
                let recursive_call_1 = polymer_recursive_memo(SegmentGrowthKey(sg_key.0-1, sg_key.1, r.2), sg_map, base_rules);
                let recursive_call_2 = polymer_recursive_memo(SegmentGrowthKey(sg_key.0-1, r.2, sg_key.2), sg_map, base_rules);

                let merged2 = Merge_HashMaps(recursive_call_1, recursive_call_2, r.2);


                return merged2;
            }
        } 

        return Bottom_Level_HashMap(sg_key.1, sg_key.2);
    }
}

fn Bottom_Level_HashMap(c1: char, c2: char) -> HashMap<char, i64> 
{
    let mut to_return = HashMap::new();
    if (c1 == c2)
    {
        to_return.insert(c1, 2);
    } 
    else 
    {
        to_return.insert(c1, 1);
        to_return.insert(c2, 1);
    }

    return to_return;
}

fn Merge_HashMaps(h1 : HashMap<char, i64>, h2 : HashMap<char, i64>, shared_character : char) -> HashMap<char, i64>
{
    let mut to_return : HashMap<char, i64> = HashMap::new();
    for h1_key in h1.keys().into_iter()
    {
        to_return.insert(*h1_key,  h1.get(h1_key).unwrap_or(&0) + h2.get(h1_key).unwrap_or(&0));
    }

    for h2_key in h2.keys().into_iter()
    {
        to_return.insert(*h2_key,  h1.get(h2_key).unwrap_or(&0) + h2.get(h2_key).unwrap_or(&0));
    }

    let sharedCharCount = to_return.get(&shared_character).unwrap();
    to_return.insert(shared_character, sharedCharCount-1);

    return to_return;
}




fn polyPuzzle(starting_string: Vec<char>, rules : Vec<Rule>, debug_print : bool)
{
    let mut iter_string = starting_string;

    for i in 0..40 
    {    
        let next = polymer(iter_string, &rules, debug_print);
        println!("iteration {}", i);
        iter_string = next;
        if (debug_print)
        {
            let my_string : String =  iter_string.iter().collect();
            println!("{}", my_string);
        }
    }

    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in iter_string.into_iter()
    {
        if counts.contains_key(&c) {
            let oldCount = counts[&c];
            counts.insert(c, oldCount+1);
        } else {
            counts.insert(c, 1);
        }
    }

    let max = counts.values().into_iter().map(|x| *x).reduce(i32::max).unwrap();
    let min = counts.values().into_iter().map(|x| *x).reduce(i32::min).unwrap();

    println!("{}", max-min);
}

fn polymer(input_string : Vec<char>, rules: &Vec<Rule>, debug_print: bool) -> Vec<char>{
    let mut return_string = Vec::new();
    for i in 0..input_string.len()-1 {
        let first = input_string.get(i).unwrap();
        let second  = input_string.get(i+1).unwrap();

        if (debug_print){
            println!("Testing {}{}", first, second);
        }

        let mut found_rule: Option<&Rule> = None;
        for r in rules.into_iter(){
            if (r.0 == *first && r.1 == *second)
            {
                found_rule = Some(r);
            }
        }

        if (found_rule.is_some())
        {
            return_string.push(found_rule.unwrap().0);
            return_string.push(found_rule.unwrap().2);
        } 
        else {
            return_string.push(*first);
        }
    }

    return_string.push(*input_string.get(input_string.len()-1).unwrap());

    return return_string;
}