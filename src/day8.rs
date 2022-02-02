use itertools::Itertools;

struct WirePermutation 
{
    perm_internal : [char; 7]
}

impl WirePermutation 
{
    fn matches_all(&self, input: Vec<Vec<char>>) -> bool 
    {
        return input.into_iter().all(|s| self.match_vec(&s).is_some());
    }

    fn match_vec(&self, input: &Vec<char>) -> Option<char> 
    {
        let string = &self.translate(input)[..];
        match string 
        {
            "cf" => Some('1'),
            "acf" => Some('7'),
            "bcdf" => Some('4'),

            "acdeg" => Some('2'),
            "acdfg" => Some('3'),
            "abdfg" => Some('5'),

            "abcefg" => Some('0'),
            "abdefg" => Some('6'),
            "abcdfg" => Some('9'),

            "abcdefg" => Some('8'),

            _ => None
        }
    }

    fn translate(&self, input: &Vec<char>) -> String
    {
        let mut as_u8 : Vec<char> =  input.into_iter().
                map(|c| (*c as u8) - 97).
                map(|i| self.perm_internal[i as usize]).
                collect();
        as_u8.sort_unstable();
        return String::from_iter(as_u8);
    }
}

fn build_wire_perm(input: Vec<char>) -> WirePermutation 
{
    return WirePermutation
    {
        perm_internal : 
        [
            *input.get(0).unwrap(),
            *input.get(1).unwrap(),
            *input.get(2).unwrap(),

            *input.get(3).unwrap(),
            *input.get(4).unwrap(),
            *input.get(5).unwrap(),

            *input.get(6).unwrap(),
        ]
    }
}

pub fn day_8(lines : Vec<String>)
{
    let notes_and_output : Vec<(Vec<&str>,Vec<&str>)> = 
        lines.iter().
        map(|l| l.split(" | ").collect()).
        map(|sp : Vec<&str>| (sp.get(0).unwrap().split(" ").collect(), sp.get(1).unwrap().split(" ").collect())).
        collect(); 

    let answer : i32 = notes_and_output.into_iter().map(|(n, o)| total_translate(n, o)).sum();
    
    println!("{:?}", answer);

}

fn total_translate(notes: Vec<&str>, output: Vec<&str>) -> i32
{
    let alpha = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    for perm in alpha.to_vec().into_iter().permutations(7).map(|p| build_wire_perm(p)) 
    {
        if perm.matches_all(notes.iter().map(|n| n.chars().collect()).collect()) 
        {
            return output.into_iter().
            map(|o| perm.match_vec(&(o.chars().collect())).unwrap()).
            collect::<String>().
            parse::<i32>().
            unwrap();
        }
    }
    panic!();
}