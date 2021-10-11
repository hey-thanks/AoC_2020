use untitled::aoc;

fn main() {
    let filename = "/home/alex/IdeaProjects/untitled/misc/D08.txt";
    let problem = aoc::Problem::Two;
    let a = aoc::day_eight::solve(problem, filename);
    println!("{}", a);
}
