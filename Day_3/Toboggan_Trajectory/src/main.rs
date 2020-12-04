// I had to look up a lot for this one, and its only the 3rd of the month...

const TXT: &str = include_str!("../inputs.txt");
fn main() {
    let forest: Vec<Vec<bool>> = TXT
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let w = forest[0].len();
    let mut x = 0;
    let mut trees = 0;
    
    for row in forest.iter() {
        if row[x] {trees += 1;
        }
        x = (x + 3) % w;
    }
    
    println!("~~~part one gets you crashing into {} trees~~~", trees);

    let mut end: i64 = 1;
    for &(dx, dy) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut x = 0;
        let mut y = 0;
        let mut total = 0;
        while y < forest.len() {
            if forest[y][x] {
                total += 1;
            }
            x = (x + dx) % w;
            y += dy;
        }
        end *= total;
    }
    println!("~~~part two gets you crashing into {} trees~~~", end);
    println!("~~~OUCH!!!~~~");
    
}