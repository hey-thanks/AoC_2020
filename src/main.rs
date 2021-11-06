use untitled::aoc;

fn main() {
    let filename = "/home/alex/IdeaProjects/untitled/misc/D09.txt";
    let problem = aoc::Problem::One;
    let a = aoc::day_nine::solve(problem, filename);
    match a {
        Some(x) => println!("{}", x),
        None => println!("Sorry, no value :/"),
    };
}
