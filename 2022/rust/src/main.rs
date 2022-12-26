mod days;

fn main() {
    /* days::solve takes two argument, the first is the day and the second is the part */
    let day = 11;
    println!("--Day {}--",day);
    println!("--Part 1--\n{}", days::solve(day,1));
    println!("--Part 2--\n{}", days::solve(day,2));
}
