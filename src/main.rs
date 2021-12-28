use untitled::aoc;

fn main() {
    let filename = "./misc/D14.txt";
    let problem = aoc::Problem::Two;
    let a = aoc::day_fourteen::solve(problem, filename);
    match a {
        Some(x) => println!("{}", x),
        None => println!("Sorry, no value :/"),
    };
}
