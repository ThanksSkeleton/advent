use std::io::BufRead;
use crate::File;
use crate::BufReader;
use crate::Index;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Vector2Int(pub i32, pub i32);

impl Index<&'_ i32> for Vector2Int {
    type Output = i32;
    fn index(&self, s: &i32) -> &i32 {
        match s {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("unknown field: {}", s),
        }
    }
}

pub fn open_file(file_name : &str) -> Vec<String> {

    let mut v = Vec::new();

    let base_path = "C:\\Users\\Chris\\Rust\\advent\\src\\";
    let mut full_path = base_path.to_owned();
    full_path.push_str(file_name);


    let f = File::open(full_path).expect("Unable to open file");
    let f = BufReader::new(f);

    v.extend(f.lines().map(|l | l.unwrap() ));

    return v;
}    
