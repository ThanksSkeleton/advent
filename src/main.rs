mod Util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
// did 6 in C#
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
//mod day18;

use std::collections::HashMap;
use std::iter::Peekable;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};
use Util::Vector2Int;
use Util::open_file;
use day1::day_1_part_2;
use day2::day_2;
use day2::day_2_part_2;
use day3::day_3;
use day4::day_4;
use day5::day_5;
// did 6 in C#
use day7::day_7;
use day8::day_8;
use day9::day_9;
use day10::day_10;
use day10::day_10_part_2;
use day11::day_11;
use day13::day_13;
use day14::day_14;
use day15::day_15;
use day16::day_16;
use day17::day_17;
use day18::day_18;
use day19::day_19;
use day20::day_20;
use day21::day_21;
use day22::day_22;
use day23::day_23;

use day24::day_24;
use day25::day_25;
//use day18::day_18;


fn main() {


    
    //let d1 = open_file("day1.txt");
    //day_1_part_2(d1);

    // let d2 = open_file("day2.txt");
    // day_2(d2);

    // let d2 = open_file("day2.txt");
    // day_2_part_2(d2);

    // let d3 = open_file("day3.txt");
    // day_3(d3);

    // let d4 = open_file("day4.txt");
    // day_4(d4);

    // let d7 = open_file("day7_example.txt");
    // day_7(d7);

    // let d7 = open_file("day7.txt");
    // day_7(d7);

    // let d8 = open_file("day8_example.txt");
    // day_8(d8);

    // let d8 = open_file("day8.txt");
    // day_8(d8);

    // let d9 = open_file("day9_example.txt");
    // day_9(d9);

    // let d9 = open_file("day9.txt");
    // day_9(d9);


    //let d10 = open_file("day10.txt");
    //day_10_part_2(d10);

  

    // let d11_t = open_file("day11_tiny.txt");
    // day_11(d11_t);

    // let d11_m = open_file("day11_medium.txt");
    // day_11(d11_m);

    // let d11 = open_file("day11.txt");
    // day_11(d11);

    //let v = open_file("day13.txt");
    //day_13(v);

    //let d14 = open_file("day14.txt");
    //day_14(d14, false);

    //let d14_ex = open_file("day14_example.txt");
    //day_14(d14_ex, true);

    //let day15 = open_file("day15.txt");
    //day_15(day15);

    // let d16_e1 = open_file("day16_example_1.txt");
    // day_16(d16_e1);
    // let d16_e2 = open_file("day16_example_2.txt");
    // day_16(d16_e2);
    // let d16_e3 = open_file("day16_example_3.txt");
    // day_16(d16_e3);

    // let d16 = open_file("day16.txt");
    // day_16(d16);

    //day_17();

    // let d18 = open_file("day18.txt");
    // day_18(d18);

    let d19_tiny = open_file("day19_tiny.txt");
    day_19(d19_tiny, 3);

    
    let d19_medium = open_file("day19_medium.txt");
    day_19(d19_medium, 12);

    // let d20_tiny = open_file("day20_tiny.txt");
    // day_20(d20_tiny);

    // let d20 = open_file("day20.txt");
    // day_20(d20);

    // let d19_tiny2 = open_file("day19_tiny.txt");
    // day_19(d19_tiny2, 3, false);

    // let d19_medium = open_file("day19_medium.txt");
    // day_19(d19_medium, 12, true);

    // let d22_t = open_file("day22_tiny.txt");
    // day_22(d22_t);

    // let d22_s = open_file("day22_small_2.txt");
    // day_22(d22_s);

    // let d21 = open_file("day22.txt");
    // day_21(d21);

    // let d22 = open_file("day22.txt");
    // day_22(d22);

    // let d22 = open_file("day22_tiny_3.txt");
    // day_22(d22);

    // let d23 = open_file("day22_tiny_3.txt");
    // day_23(d23);

    // let d24 = open_file("day25.txt");
    // day_24(d24);

    // let d25 = open_file("day25.txt");
    // day_25(d25);

    //go();
}