use std::ops::Add;


pub fn day_10(lines : Vec<String>)
{
    let char_open = ['(', '[', '{', '<' ];
    let char_close = [')', ']', '}', '>' ];
    let bracket_corruption_score = [3, 57, 1197, 25137];

    let mut total_score = 0;

    for line in lines.into_iter()
    {
        let new_line_score = line_corruption_score(line, char_open, char_close, bracket_corruption_score);
        println!("{} score", new_line_score);
        total_score = total_score + new_line_score;
    }

    println!("{}", total_score);
}

pub fn day_10_part_2(lines : Vec<String>)
{
    let char_open = ['(', '[', '{', '<' ];
    let char_close = [')', ']', '}', '>' ];
    let bracket_incomplete_score = [1, 2, 3, 4];
    let irrelevant_score = [1; 4];

    let mut scores: Vec<i64> = lines.into_iter().
    filter(|x| line_corruption_score(x.to_string(), char_open, char_close, irrelevant_score) == 0).
    map(|x | line_incomplete_score(x, char_open, char_close, bracket_incomplete_score)).
    collect();

    scores.sort();

    println!("{}", scores.get(scores.len() / 2).unwrap());
}

fn line_corruption_score(line: String, opens : [char; 4], closes : [char; 4], bracket_score: [i32; 4]) -> i32
{
    let mut stack = Vec::new();

    for c in line.chars() 
    {
        let return_value = match_or_mismatch(c, stack, opens, closes);

        stack = return_value.0;

        if return_value.1.is_some() 
        {
            println!("corrupt");
            return *bracket_score.get(return_value.1.unwrap() as usize).unwrap();
        }
    }

    println!("not corrupt");

    return 0;
}

fn line_incomplete_score(line: String, opens : [char; 4], closes : [char; 4], bracket_score: [i64; 4]) -> i64
{
    let mut stack = Vec::new();

    for c in line.chars() 
    {
        for i in 0..4 
        {
            if c == opens[i] 
            {
                stack.push(c);
            } 
            else if c == closes[i] 
            {
                stack.pop();
            }
        }
    }

    let mut end_score: i64 = 0;

    while stack.len() != 0
    {
        //println!("{} pop", stack.len());
        let c = stack.pop().unwrap();
        let index = opens.into_iter().position(|x| x == c).unwrap();
        end_score = end_score * 5 + bracket_score.get(index).unwrap();
    }

    println!("{}", end_score);

    return end_score;
}


// c is the char to check
// stack is a tracking stack of all the currently opened brackets (blocks)
// opens and closes are 4 character maps with opening and closing brackets of different types
// if there is a mismatch (or the stack is empty) for a closing character, return the index of the bracket type
// else return None
fn match_or_mismatch(c : char, mut stack : Vec<char>, opens : [char; 4], closes : [char; 4]) -> (Vec<char>, Option<i32>) 
{
    for i in 0..4 
    {
        if c == opens[i] 
        {
            stack.push(c);
            return (stack.clone(), None);
        } 
        else if c == closes[i] 
        {
            let popped = stack.pop();

            //'error condition' 1
            if popped.is_none() {  return (stack.clone(), Some(i.try_into().unwrap())); }

            let expected = opens[i];

            match popped.unwrap()
            {
                // no error
                x if x == expected  => return (stack.clone(), None),
                //'error condition' 2
                _ => return (stack.clone(), Some(i.try_into().unwrap()))
            }
        }
    }
    unreachable!();
}