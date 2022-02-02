#[path ="Util.rs"]
mod Util;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};
use Util::Vector2Int;


pub fn day_13(lines : Vec<String>) 
{
    let mut points: HashSet<Vector2Int> = HashSet::new();
    let mut folds: Vec<Vector2Int> = Vec::new();

    for s in lines.into_iter()
    {
        if s.is_empty(){}
        else if s.contains(',')
        {
            let split = s.split(',').collect::<Vec<&str>>();
            points.insert(Vector2Int(split[0].parse::<i32>().unwrap(), split[1].parse::<i32>().unwrap()));
        } 
        else 
        {
            let split = s.split('=').collect::<Vec<&str>>();
            let xybool = if split[0].contains("x") { 0 } else { 1 };
            folds.push(Vector2Int(xybool, split[1].parse::<i32>().unwrap()));
        }
    }

    println!("Initial Points: {}", points.len());

    for fold in folds.into_iter()
    {
        points = day_13_fold(points, fold);
    }

    day_13_print_grid(&points);
}

fn day_13_fold(pts : HashSet<Vector2Int>, fold : Vector2Int) -> HashSet<Vector2Int>
{
    let mut folded: HashSet<Vector2Int> = HashSet::new();
    for pt in pts.into_iter()
    {
        if fold.0 == 0
        {
            if pt.0 > fold.1
            {
                folded.insert(Vector2Int((fold.1 - (pt.0 - fold.1)), pt.1));
            } 
            else if (pt.0 < fold.1)
            {
                folded.insert(pt.clone());
            } 
            else 
            {
                panic!("points on fold are not allowed");
            }
        }
        else if fold.0 == 1
        {
            if pt.1 > fold.1
            {
                folded.insert(Vector2Int(pt.0, (fold.1 - (pt.1 - fold.1))));
            } 
            else if (pt.1 < fold.1)
            {
                folded.insert(pt.clone());
            } 
            else 
            {
                panic!("points on fold are not allowed");
            }
        }   
    }

    return folded;
}

fn day_13_print_grid(pts: &HashSet<Vector2Int>)
{
    let x_max = pts.into_iter().map(|pt| pt.0).reduce(i32::max).unwrap();
    let y_max = pts.into_iter().map(|pt| pt.1).reduce(i32::max).unwrap();

    let mut to_print = String::new();

    for i in 0..y_max+1
    {
        for j in 0..x_max+1
        {
            if pts.contains(&Vector2Int(j,i))
            {
                to_print.push('X');
            } 
            else 
            {
                to_print.push('-');
            }
        }
        to_print.push_str("\n")
    }

    print!("{}", to_print);
}