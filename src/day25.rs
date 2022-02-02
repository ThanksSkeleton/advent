const EMPTY : char = '.';
const RIGHT : char = '>';
const DOWN : char = 'v';


struct MoveCommand 
{
    source : (i32, i32),
    destination : (i32, i32),
    character : char    
}

struct CucumberGrid 
{
    inner : Vec<Vec<char>>
}

impl CucumberGrid 
{
    fn dim(&self) -> (i32, i32) 
    {
        return (self.inner.get(0).unwrap().len() as i32, self.inner.len() as i32);
    }

    fn mutate(&mut self) -> bool 
    {
        let r = self.mutate_single_type((1,0), RIGHT);
        let d = self.mutate_single_type((0,1), DOWN);
        return r||d;
    }

    fn mutate_single_type(&mut self, direction: (i32, i32), character: char) -> bool
    {
        let dim = self.dim();

        let mut cuke_moves : Vec<MoveCommand> = Vec::new();
        for x in 0..dim.0 
        {
            for y in 0..dim.1 
            {
                let source = self.get((x,y));
                if source == character
                {   
                    let destination = self.get((x + direction.0, y+direction.1));
                    if destination == EMPTY
                    {
                        let mv = MoveCommand 
                        {
                            source: (x,y),
                            destination: (x + direction.0, y+direction.1),
                            character,
                        };
                        cuke_moves.push(mv);
                    }
                }
            }
        }

        let updated = cuke_moves.len() != 0;

        for mv in cuke_moves 
        {
            self.move_cukes(mv);
        }

        return updated;
    }

    fn move_cukes(&mut self, command: MoveCommand) 
    {
        self.set(command.source, EMPTY);
        self.set(command.destination, command.character);
    }

    fn get(&self, point : (i32, i32)) -> char 
    {
        let dim = self.dim();
        let row = self.inner.get(point.1.rem_euclid(dim.1) as usize).unwrap();
        return *row.get(point.0.rem_euclid(dim.0) as usize).unwrap();
    }

    fn set(&mut self, point : (i32, i32), new_char: char) 
    {
        let dim = self.dim();

        let x_coord = point.0.rem_euclid(dim.0) as usize;
        let y_coord = point.1.rem_euclid(dim.1) as usize;

        let row = self.inner.get_mut(y_coord).unwrap();
        row.splice(x_coord..x_coord+1, vec![new_char]);
    }

    fn print(&self) 
    {
        println!("cuke grid");
        for line in self.inner.iter()
        {
            let s : String = line.iter().collect();
            println!("{}", s);
        }
    }

}

pub fn day_25(lines: Vec<String>)
{
    let grid_data : Vec<Vec<char>> = lines.into_iter().map(|s| s.chars().collect()).collect();
    let mut cgrid = CucumberGrid { inner : grid_data };

    for i in 1.. 
    {
        if !cgrid.mutate() 
        {
            println!("{} answer", i);
            return;
        }

        //cgrid.print();
    } 
}
