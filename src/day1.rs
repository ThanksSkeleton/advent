struct Window(i32, i32, i32);

impl Window {
    fn bump(&self, input : i32 ) -> Window
    {
        return Window(self.1, self.2, input);
    }

    fn is_more_than(&self, other: Window) -> bool 
    {
        return self.0.saturating_add(self.1).saturating_add(self.2) > other.0.saturating_add(other.1).saturating_add(other.2);
    }
}

pub fn day_1_part_2(lines : Vec<String>)
{
    let mut prev = Window(i32::MAX, i32::MAX, i32::MAX);
    let mut more_count = 0;

    for l in lines 
    {
        let parsed = l.parse::<i32>().unwrap();

        let new_window = prev.bump(parsed);

        if new_window.is_more_than(prev)
        {
            more_count = more_count + 1;
        }

        prev = new_window;
    }

    println!("{} more_count", more_count);
}
