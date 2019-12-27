use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let reader = BufReader::new(File::open("src/input.txt").expect("Cannot open file.txt"));
    let mut vec: Vec<i32> = Vec::new();
    for line in reader.lines() {
        for word in line.unwrap().split(',') {
            //println!("word '{}'", word);
            //Maybe we could do all the calculations in here,
            //but I'm not very familiar with rust iterators
            vec.push(word.parse::<i32>().unwrap());
        }
    }
    //println!("{:?}", vec);
    // Part 1
    //vec[1]=12;
    //vec[2]=2;
    //run(&mut vec);
    //println!("{:?}", vec[0]);

    //Part 2
    find_values(&mut vec, 19690720);
}

fn run(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        if i % 4 == 0 {
            //println!("{}", i);
            if v[i] == 99 {
                // immediately halt
                //println!("quitting!");
                return;
            } else if v[i] == 1 {
                let idx1 = v[i + 1] as usize;
                let idx2 = v[i + 2] as usize;
                let idx3 = v[i + 3] as usize;
                // add position i+1 and i+2 then store in i+3
                v[idx3] = v[idx1] + v[idx2];
            //println!("adding!");
            } else if v[i] == 2 {
                let idx1 = v[i + 1] as usize;
                let idx2 = v[i + 2] as usize;
                let idx3 = v[i + 3] as usize;
                // mult position i+1 and i+2 then store in i+3
                v[idx3] = v[idx1] * v[idx2];
                //println!("multing!");
            }
        }
    }
}

fn find_values(v: &mut Vec<i32>, output: i32) {
    for x in 0..99 {
        for y in 0..99 {
            let mut v1 = v.clone();
            v1[1] = x;
            v1[2] = y;
            run(&mut v1);
            if v1[0] == output {
                print!("{0}{1}\n",x,y);
            }
        }
    }
}
