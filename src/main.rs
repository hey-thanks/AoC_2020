use untitled::aoc;

fn main() {
    let filename = "./misc/D15.txt";
    let problem = aoc::Problem::Two;
    let a = aoc::day_fifteen::solve(problem, filename);
    match a {
        Some(x) => println!("{}", x),
        None => println!("Sorry, no value :/"),
    };
}
