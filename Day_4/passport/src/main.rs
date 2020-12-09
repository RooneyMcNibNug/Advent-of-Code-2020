use regex::Regex;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;



fn is_valid(passport: &str) -> bool {
    let byr_reg = Regex::new(r"byr:(.)+").unwrap(); // Birth Year
    let iyr_reg = Regex::new(r"iyr:(.)+").unwrap(); // Issue Year
    let eyr_reg = Regex::new(r"eyr:(.)+").unwrap(); // Exp Year
    let hgt_reg = Regex::new(r"hgt:(.)+").unwrap(); // Height
    let hcl_reg = Regex::new(r"hcl:(.)+").unwrap(); // Hair Color
    let ecl_reg = Regex::new(r"ecl:(.)+").unwrap(); // Eye Color
    let pid_reg = Regex::new(r"pid:(.)+").unwrap(); // Passport ID
    let cid_reg = Regex::new(r"cid:(.)+").unwrap(); // Country ID
    let byr_bool: bool = byr_reg.is_match(passport);
    let iyr_bool: bool = iyr_reg.is_match(passport);
    let eyr_bool: bool = eyr_reg.is_match(passport);
    let hgt_bool: bool = hgt_reg.is_match(passport);
    let hcl_bool: bool = hcl_reg.is_match(passport);
    let ecl_bool: bool = ecl_reg.is_match(passport);
    let pid_bool: bool = pid_reg.is_match(passport);
    let cid_bool: bool = cid_reg.is_match(passport);
    let valid: bool = byr_bool && iyr_bool && eyr_bool && hgt_bool && hcl_bool && ecl_bool && pid_bool; // Country ID doesn't make a bit of difference
    
    if !valid { 
        println!("invalid {}", passport); 
        println!("byr {} iyr {} eyr {} hgt {} hcl {} ecl {} pid {}\n", byr_bool, iyr_bool, eyr_bool, hgt_bool, hcl_bool, ecl_bool, pid_bool);
    }

    return valid;
}

fn main() {
    let mut file = File::open("inputs.txt").expect("Cannot open file - please check directory");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    // println!("{}", contents);
    // need to add an iterator here! and then search for bools and validity


}
