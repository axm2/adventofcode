use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let mass = line.parse::<i32>().unwrap();
        result+= calc(mass)+recursive_calc(calc(mass));
    }

    print!("{}\n", result);

    //print!("{}\n", calc(100756)+recursive_calc(calc(100756)));
}

fn calc(mass: i32) -> i32 {
    let fuel = (mass/3 as i32)-2;
    return fuel;
}

fn recursive_calc(mass: i32) -> i32{
    if mass>8 { // Because floor(8/3)-2 = 0
        return calc(mass)+recursive_calc(calc(mass));
    }
    return 0;

}