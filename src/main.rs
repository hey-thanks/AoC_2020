use untitled::aoc;

fn main() {
    let filename = "./misc/D17.txt";
    let problem = aoc::Problem::Two;
    let a = aoc::day_seventeen::solve(problem, filename);
    match a {
        Some(x) => println!("{}", x),
        None => println!("Sorry, no value :/"),
    };
}
