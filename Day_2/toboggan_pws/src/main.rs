use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File could not be opened - please check location");
    
    let lines = contents.lines();

    let mut adhere = 0;
    let mut adhere2 = 0;

    for line in lines {
        let mut parts = line.split(" ");
        let range: Vec<usize> = parts.next().unwrap().split("-").map(|x| x.parse::<usize>().unwrap()).collect();
        let letter = parts.next().unwrap().chars().next().unwrap();
        let pw = parts.next().unwrap();

        println!("{}-{} {}: {}", range[0], range[1], letter, pw);

        let letter_count = pw.matches(letter).count();

        println!("count: {}", letter_count);

        if letter_count >= range[0] && letter_count <= range[1] {
            adhere += 1;
        }

        let p1 = pw.chars().nth(range[0] - 1).unwrap();
        println!("character at place 1: {} \n", p1);

        let p2 = pw.chars().nth(range[1] - 1).unwrap();
        println!("character at place 2: {} \n", p2);

        if (p1 == letter) ^ (p2 == letter) {
            adhere2 += 1;
        }
    }

    println!("~~~{} pws adherent to rule #1~~~", adhere);
    println!("~~~{} pws adherent to rule #2~~~", adhere2);
}