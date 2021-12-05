use untitled::aoc;

fn main() {
    let filename = "./misc/D12.txt";
    let problem = aoc::Problem::One;
    let a = aoc::day_twelve::solve(problem, filename);
    match a {
        Some(x) => println!("{}", x),
        None => println!("Sorry, no value :/"),
    };
}
