use std::iter::Peekable;

#[derive(Debug)]
struct BingoBoard 
{
    name: i32,
    raw_board : [[i32; 5]; 5],
    marked_board: [[bool; 5]; 5]
}

impl BingoBoard 
{    
    fn add(&mut self, mark: i32) -> Option<i32>  
    {
        for pt in (0..5).into_iter().flat_map(|x| (0..5).into_iter().map(move |y| (x.clone(), y.clone()))) 
        {
            if self.raw_board[pt.0][pt.1] == mark 
            {
                self.marked_board[pt.0][pt.1] = true;
                return self.check_win(mark, pt.0, pt.1);
            }
        }
        None
    }

    fn check_win(&self, mark: i32, x_mark: usize, y_mark: usize) -> Option<i32> 
    {
        if (0..5).into_iter().all(|x| self.marked_board[x][y_mark]) 
        || (0..5).into_iter().all(|y| self.marked_board[x_mark][y])
        {
            return Some(self.score(mark));
        }
        None            
    }

    fn score(&self, mark: i32) -> i32
    {
        return (0..5).into_iter().flat_map(|x| (0..5).into_iter().map(move |y| (x,y))).
        filter(|pt| !self.marked_board[pt.0][pt.1]).
        map(|pt| self.raw_board[pt.0][pt.1]).
        sum::<i32>() * mark;
    }
}

fn parse_line(l : String) -> [i32; 5] 
{
    let line: Vec<&str> = l.split(" ").filter(|x| x != &"").collect();
    return   [ 
                line.get(0).unwrap().parse::<i32>().unwrap(),
                line.get(1).unwrap().parse::<i32>().unwrap(),
                line.get(2).unwrap().parse::<i32>().unwrap(),
                line.get(3).unwrap().parse::<i32>().unwrap(),
                line.get(4).unwrap().parse::<i32>().unwrap()
             ];
}

fn new_board(name: i32, l1 : String, l2: String, l3: String, l4: String, l5: String) -> BingoBoard 
{
    return BingoBoard
                        { 
                            name: name,
                            raw_board: [parse_line(l1), parse_line(l2),parse_line(l3),parse_line(l4),parse_line(l5)],
                            marked_board: [[false; 5]; 5]
                        };
}

pub fn day_4(lines : Vec<String>)
{
    let result = result_loser(lines);
    println!("{} result", result);
}

fn result(lines : Vec<String>) -> i32 
{
    let mut iter = lines.into_iter().peekable();
    let first_line = iter.next().unwrap();
    let numbers_called : Vec<i32> = first_line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut boards = build_boards(iter);

    for num in numbers_called 
    {
        let r = inside(&mut boards, num);
        if r.is_some()
        {
            return r.unwrap();
        }
    }
    panic!()
}

fn result_loser(lines: Vec<String>) -> i32 
{
    let mut iter = lines.into_iter().peekable();
    let first_line = iter.next().unwrap();
    let numbers_called : Vec<i32> = first_line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut boards = build_boards(iter);

    for num in numbers_called 
    {
        if boards.len() > 1 
        {
            let winners = inside_index(&mut boards, num);
            let winners_index_shift: Vec<usize> = winners.into_iter().enumerate().map(|(i, usize)| usize - i ).collect();
            for w in winners_index_shift 
            {
                boards.remove(w);
            }
        } 
        else 
        {
            let r = inside(&mut boards, num);
            if r.is_some()
            {
                return r.unwrap();
            }
        }
    }
    panic!()
}

fn inside_index(boards: &mut Vec<BingoBoard>, num : i32) -> Vec<usize>
{   
    let mut to_return = Vec::new();
    for (i, board) in boards.into_iter().enumerate() 
    {
        let b = board.add(num);
        if b.is_some()
        {
            to_return.push(i);
        }
    }
    return to_return;
}

fn inside(boards: &mut Vec<BingoBoard>, num : i32) -> Option<i32>
{   
    for board in boards 
    {
        let b = board.add(num);
        if b.is_some()
        {
            return b;
        }
    }
    None
}

fn build_boards(mut iter : Peekable<impl Iterator<Item = String>>) -> Vec<BingoBoard> 
{
    let mut name = 0;
    let mut boards = Vec::new();

    while iter.peek().is_some() 
    {
        iter.next();
        boards.push(
            new_board(
                name,
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap()
            ));
        name = name + 1;
    }

    return boards;
}