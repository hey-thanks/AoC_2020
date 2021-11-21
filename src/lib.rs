pub mod aoc {
    use std::fs;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    #[derive(Debug, Clone, Copy)]
    pub enum Problem {
        One,
        Two,
    }

    fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = fs::File::open(filename).expect("File not found.");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line."))
            .collect()
    }

    pub mod day_one {
        use itertools::Itertools;
        use std::fs;

        pub fn solve(problem: super::Problem, file: &str) -> i32 {
            let subset_size = match problem {
                super::Problem::One => 2,
                super::Problem::Two => 3,
            };

            let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

            let combos: Vec<Vec<i32>> = contents
                .split_whitespace()
                .map(|s| s.parse().expect("parse error"))
                .dedup()
                .combinations(subset_size)
                .filter(|v: &Vec<i32>| v.iter().copied().sum::<i32>() == 2020)
                .collect();

            combos[0].iter().product()
        }
    }

    pub mod day_two {
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;

        #[derive(Debug, Clone)]
        struct PasswordInfo {
            range: (i32, i32),
            letter: char,
            password: String,
        }

        impl PasswordInfo {
            fn new(line: &str) -> Self {
                let fields: Vec<&str> = line.split_whitespace().collect();
                let range = PasswordInfo::parse_range(fields[0]);
                let letter = fields[1]
                    .chars()
                    .next()
                    .expect("Letter could not be parsed.");
                let password = fields[2].to_string();
                PasswordInfo {
                    range,
                    letter,
                    password,
                }
            }

            fn parse_range(range: &str) -> (i32, i32) {
                let min_to_max: Vec<&str> = range.split('-').collect();
                let min = min_to_max[0].parse().expect("Error parsing range minimum.");
                let max = min_to_max[1].parse().expect("Error parsing range maximum.");
                (min, max)
            }

            fn d02_p01_valid(&self) -> bool {
                let letter_count = self.password.matches(self.letter).count() as i32;
                letter_count >= self.range.0 && letter_count <= self.range.1
            }

            fn d02_p02_valid(&self) -> bool {
                let first_index = self.range.0 - 1;
                let second_index = self.range.1 - 1;
                let first_letter = self.password.chars().nth(first_index as usize).expect("");
                let second_letter = self.password.chars().nth(second_index as usize).expect("");
                first_letter == self.letter && second_letter != self.letter
                    || first_letter != self.letter && second_letter == self.letter
            }
        }

        fn helper(filename: &str, validator: fn(p: &PasswordInfo) -> bool) -> i32 {
            let file = File::open(filename).expect("Could not open file.");
            let reader = BufReader::new(file);

            reader
                .lines()
                .map(|x| PasswordInfo::new(&x.expect("")))
                .filter(|x| validator(&x))
                .count() as i32
        }

        pub fn solve(problem: super::Problem, file: &str) -> i32 {
            match problem {
                super::Problem::One => helper(file, PasswordInfo::d02_p01_valid),
                super::Problem::Two => helper(file, PasswordInfo::d02_p02_valid),
            }
        }
    }

    pub mod day_three {

        struct Move {
            down: usize,
            right: usize,
        }

        pub fn solve(problem: super::Problem, filename: &str) -> i32 {
            let terrain = super::lines_from_file(filename);

            match problem {
                super::Problem::One => tree_counter(&terrain, &Move { down: 1, right: 3 }),
                super::Problem::Two => {
                    let moves: Vec<Move> = vec![
                        Move { down: 1, right: 1 },
                        Move { down: 1, right: 3 },
                        Move { down: 1, right: 5 },
                        Move { down: 1, right: 7 },
                        Move { down: 2, right: 1 },
                    ];
                    moves.iter().map(|m| tree_counter(&terrain, m)).product()
                }
            }
        }

        fn tree_counter(lines: &[String], movement: &Move) -> i32 {
            let mut tree_count = 0;
            let mut char_index = 0;
            for (line_count, line) in lines.iter().enumerate() {
                if line_count > 0 && line_count % movement.down == 0 {
                    char_index += movement.right;
                    char_index %= line.chars().count();
                }
                if line.chars().nth(char_index) == Some('#')
                    && line_count > 0
                    && line_count % movement.down == 0
                {
                    tree_count += 1;
                }
            }
            tree_count
        }
    }

    pub mod day_four {
        use std::fs;
        use std::fs::File;
        use std::io::Read;

        fn valid(entry: &str, problem: super::Problem) -> bool {
            match problem {
                super::Problem::One => {
                    let field_count = entry.split_whitespace().count();
                    field_count == 8 || (field_count == 7 && !entry.contains("cid"))
                }
                super::Problem::Two => {
                    if !valid(entry, super::Problem::One) {
                        return false;
                    }
                    for field in entry.split_whitespace() {
                        let title = field.split(':').collect::<Vec<&str>>()[0];
                        let value = field.split(':').collect::<Vec<&str>>()[1];
                        match title {
                            "byr" => {
                                if !(1920..=2002).contains(
                                    &value.parse::<i32>().expect("Could not parse 'byr' value"),
                                ) {
                                    return false;
                                }
                            }
                            "iyr" => {
                                if !(2010..=2020).contains(
                                    &value.parse::<i32>().expect("Could not parse 'iyr' value"),
                                ) {
                                    return false;
                                }
                            }
                            "eyr" => {
                                if !(2020..=2030).contains(
                                    &value.parse::<i32>().expect("Could not parse 'eyr' value"),
                                ) {
                                    return false;
                                }
                            }
                            "hgt" => {
                                if value.contains("cm") {
                                    if !(150..=193).contains(
                                        &value
                                            .trim_end_matches("cm")
                                            .parse::<i32>()
                                            .expect("Could not parse 'hgt cm' value"),
                                    ) {
                                        return false;
                                    }
                                } else if value.contains("in") {
                                    if !(59..=76).contains(
                                        &value
                                            .trim_end_matches("in")
                                            .parse::<i32>()
                                            .expect("Could not parse 'hgt in' value"),
                                    ) {
                                        return false;
                                    }
                                } else {
                                    return false;
                                }
                            }
                            "hcl" => {
                                if value.starts_with('#') && value.len() == 7 {
                                    if !(0..0xffffff).contains(
                                        &i64::from_str_radix(&value[1..], 16)
                                            .expect("Could not parse 'hcl #' in value"),
                                    ) {
                                        return false;
                                    }
                                } else {
                                    return false;
                                }
                            }
                            "ecl" => {
                                let eye_colors =
                                    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                                if eye_colors
                                    .iter()
                                    .map(|c| c == &value)
                                    .filter(|x| *x)
                                    .count()
                                    != 1
                                {
                                    return false;
                                }
                            }
                            "pid" => {
                                if value.len() != 9
                                    || !(0..=999999999).contains(
                                        &value.parse::<i32>().expect("Could not parse 'pid' value"),
                                    )
                                {
                                    return false;
                                }
                            }
                            "cid" => {}
                            _ => panic!("Title of entry does not match known titles"),
                        };
                    }
                    true
                }
            }
        }

        pub fn solve(problem: super::Problem, filename: &str) -> i32 {
            let mut s = String::new();
            let mut file = File::open(filename).expect("Could not open file.");
            fs::File::read_to_string(&mut file, &mut s).expect("Could not parse file.");

            s.split("\n\n")
                .map(|e| valid(e, problem))
                .filter(|x| *x)
                .count() as i32
        }
    }

    pub mod day_five {
        fn boarding_zone_to_seat_id(zone: &str) -> i32 {
            let row = &zone[..7].replace('F', "0").replace('B', "1");
            let col = &zone[7..].replace('L', "0").replace('R', "1");
            let row_num = i64::from_str_radix(row, 2).expect("Could not parse row");
            let col_num = i64::from_str_radix(col, 2).expect("Could not parse col");
            (row_num * 8 + col_num) as i32
        }

        pub fn solve(problem: super::Problem, filename: &str) -> i32 {
            let boarding_zones = super::lines_from_file(filename);
            let seat_ids: Vec<_> = boarding_zones
                .iter()
                .map(|x| boarding_zone_to_seat_id(x))
                .collect();
            match problem {
                super::Problem::One => *seat_ids.iter().max().unwrap() as i32,
                super::Problem::Two => {
                    for seat in &seat_ids {
                        if !seat_ids.contains(&(seat + 1)) && seat_ids.contains(&(seat + 2)) {
                            return (*seat + 1) as i32;
                        }
                    }
                    -42
                }
            }
        }
    }

    pub mod day_six {
        use itertools::Itertools;
        use std::fs;
        use std::fs::File;
        use std::io::Read;

        pub fn solve(problem: super::Problem, filename: &str) -> i32 {
            let mut s = String::new();
            let mut file = File::open(filename).expect("Could not open file.");
            fs::File::read_to_string(&mut file, &mut s).expect("Could not parse file.");

            let groups = s.split("\n\n");
            match problem {
                super::Problem::One => groups.map(|x| count_anyone(x)).sum::<i32>() as i32,
                super::Problem::Two => groups.map(|x| count_everyone(x)).sum::<i32>() as i32,
            }
        }

        fn count_anyone(group: &str) -> i32 {
            group
                .split_whitespace()
                .collect::<String>()
                .chars()
                .unique()
                .count() as i32
        }

        fn count_everyone(group: &str) -> i32 {
            let num_members_in_group = group.matches('\n').count() + 1;
            let letters_in_group: Vec<_> = group
                .split_whitespace()
                .collect::<String>()
                .chars()
                .unique()
                .collect();
            letters_in_group
                .iter()
                .map(|&x| group.matches(x).count())
                .filter(|&x| x == num_members_in_group)
                .count() as i32
        }
    }

    pub mod day_seven {
        use crate::aoc::lines_from_file;
        use std::collections::HashMap;
        use std::collections::HashSet;

        pub fn solve(problem: super::Problem, filename: &str) -> i32 {
            let rules = lines_from_file(filename);

            let mut contains = HashMap::new();
            for rule in rules {
                let rule_parts: Vec<String> = rule
                    .split(" bags contain ")
                    .map(|s| s.to_string())
                    .collect();
                let bag_type = String::from(&rule_parts[..1].join(" "));
                let contents = parse_contents(&rule_parts[1..].join(" "));

                contains.insert(bag_type, contents);
            }

            match problem {
                super::Problem::One => {
                    let mut acc: HashSet<String> = HashSet::new();
                    find_bag(&contains, "shiny gold", &mut acc);
                    find_bag_helper(&contains, &mut acc);
                    acc.len() as i32
                }
                super::Problem::Two => calculate(&contains, "shiny gold"),
            }
        }

        fn calculate(map: &HashMap<String, Option<Vec<(usize, String)>>>, bag: &str) -> i32 {
            let mut result: i32 = 0;
            if map.contains_key(bag) {
                match &map[bag] {
                    None => result = 0,
                    Some(value) => {
                        for tuple in value {
                            let num_bags = tuple.0 as i32;
                            let inner_bag = &tuple.1;
                            result += num_bags + num_bags * calculate(map, inner_bag);
                        }
                    }
                }
            }
            result
        }

        fn find_bag_helper(
            map: &HashMap<String, Option<Vec<(usize, String)>>>,
            accu: &mut HashSet<String>,
        ) {
            let accu_copy = accu.clone();
            for bag in accu_copy.iter() {
                find_bag(&map, bag, accu);
            }

            if accu.len() > accu_copy.len() {
                find_bag_helper(map, accu);
            }
        }

        fn find_bag(
            map: &HashMap<String, Option<Vec<(usize, String)>>>,
            bag: &str,
            accu: &mut HashSet<String>,
        ) {
            for (key, value) in map {
                match value {
                    None => (),
                    Some(val) => {
                        for tuple in val {
                            if tuple.1 == bag {
                                accu.insert(key.to_string());
                            }
                        }
                    }
                }
            }
        }

        fn parse_contents(contents: &str) -> Option<Vec<(usize, String)>> {
            if contents.contains(char::is_numeric) {
                if contents.contains(',') {
                    Some(
                        contents
                            .split(", ")
                            .map(|s| parse_single_bag(s))
                            .collect::<Vec<(usize, String)>>(),
                    )
                } else {
                    Some(vec![parse_single_bag(contents)])
                }
            } else {
                None
            }
        }

        fn parse_single_bag(contents: &str) -> (usize, String) {
            let num = contents
                .chars()
                .next()
                .expect("Could not parse bag number character.")
                .to_string()
                .parse::<usize>()
                .expect("Could not parse bag number.");
            let temp = &contents.split_whitespace().collect::<Vec<&str>>()[1..3];
            let bag_type = format!("{} {}", temp[0], temp[1]);
            (num, bag_type)
        }
    }

    pub mod day_eight {
        use crate::aoc::lines_from_file;

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        enum OpCode {
            Nop,
            Acc,
            Jmp,
        }

        #[derive(Debug, Copy, Clone)]
        struct Instruction {
            op: OpCode,
            val: i32,
        }

        #[derive(Debug, Copy, Clone)]
        struct AccumulatorInfo {
            value: i32,
            end_index: i32,
        }

        pub fn solve(problem: super::Problem, filename: &str) -> Option<i32> {
            let ops = lines_from_file(filename);
            let instructions: Vec<Instruction> =
                ops.iter().map(|op| parse_instruction(op)).collect();

            match problem {
                super::Problem::One => Some(problem_one(&instructions).value),
                super::Problem::Two => problem_two(&instructions),
            }
        }

        fn problem_one(instructions: &[Instruction]) -> AccumulatorInfo {
            let mut global_accumulator = 0;
            let mut visited: Vec<i32> = vec![];
            let mut line_num: i32 = 0;

            while (!visited.contains(&line_num)) && (line_num < instructions.len() as i32) {
                visited.push(line_num);
                let instr = instructions[line_num as usize];
                match instr.op {
                    OpCode::Acc => {
                        global_accumulator += instr.val;
                        line_num += 1;
                    }
                    OpCode::Jmp => line_num += instr.val,
                    _ => line_num += 1,
                }
            }

            AccumulatorInfo {
                value: global_accumulator,
                end_index: line_num,
            }
        }

        fn problem_two(instructions: &[Instruction]) -> Option<i32> {
            let mut index = 0;
            loop {
                if index == instructions.len() {
                    return None;
                }
                match instructions[index].op {
                    OpCode::Jmp | OpCode::Nop => {
                        let mut temp = instructions.to_owned();
                        swap_jmp_and_nop(&mut temp, index);
                        let accumulator = problem_one(&temp);
                        if accumulator.end_index == temp.len() as i32 {
                            return Some(accumulator.value);
                        }
                        index += 1;
                    }
                    _ => index += 1,
                }
            }
        }

        fn swap_jmp_and_nop(instructions: &mut [Instruction], index: usize) {
            match instructions[index].op {
                OpCode::Jmp => instructions[index].op = OpCode::Nop,
                OpCode::Nop => instructions[index].op = OpCode::Jmp,
                _ => (),
            }
        }

        fn parse_instruction(instr: &str) -> Instruction {
            let instruction_parts: Vec<&str> = instr.split(' ').collect();
            let op = match instruction_parts[0] {
                "acc" => OpCode::Acc,
                "jmp" => OpCode::Jmp,
                _ => OpCode::Nop,
            };
            let val = instruction_parts[1]
                .parse::<i32>()
                .expect("Could not parse instruction value.");
            Instruction { op, val }
        }
    }

    pub mod day_nine {
        use crate::aoc::lines_from_file;
        use itertools::Itertools;

        #[derive(Debug, Copy, Clone)]
        struct InvalidNumberInfo {
            number: i64,
            index: usize,
        }

        pub fn solve(problem: super::Problem, filename: &str) -> Option<i64> {
            let numbers: Vec<i64> = lines_from_file(filename)
                .iter()
                .map(|x| x.parse().expect("Could not parse integer."))
                .collect();

            match problem {
                super::Problem::One => Some(problem_one(&numbers)?.number),
                super::Problem::Two => problem_two(&numbers),
            }
        }

        fn problem_one(numbers: &[i64]) -> Option<InvalidNumberInfo> {
            let window_size = 25;
            for i in window_size..numbers.len() {
                if !is_number_valid(numbers, i, window_size) {
                    return Some(InvalidNumberInfo {
                        number: numbers[i],
                        index: i,
                    });
                }
            }
            None
        }

        fn is_number_valid(numbers: &[i64], index: usize, window_size: usize) -> bool {
            let start = index - window_size;
            let perms = numbers[start..index].iter().copied().permutations(2);
            perms
                .map(|p| p.iter().sum::<i64>())
                .any(|x| x == numbers[index])
        }

        fn problem_two(numbers: &[i64]) -> Option<i64> {
            let info = problem_one(&numbers)?;

            let mut start_index = 0;
            let mut end_index = 2;
            loop {
                let sum: i64 = numbers[start_index..end_index].iter().sum();
                if (sum < info.number) && (end_index < info.index) {
                    end_index += 1;
                } else if (sum > info.number) || (end_index > info.index) {
                    start_index += 1;
                    end_index = start_index + 2;
                } else if sum == info.number {
                    let minmax = numbers[start_index..end_index]
                        .iter()
                        .minmax()
                        .into_option()?;
                    return Some(minmax.0 + minmax.1);
                } else {
                    return None;
                }
            }
        }
    }

    pub mod day_ten {
        use crate::aoc::lines_from_file;
        use itertools::Itertools;

        pub fn solve(problem: super::Problem, filename: &str) -> Option<i64> {
            let mut joltage_ratings: Vec<i32> = lines_from_file(filename)
                .iter()
                .map(|x| x.parse().expect("Could not parse integer."))
                .sorted()
                .collect();

            joltage_ratings.insert(0, 0);
            joltage_ratings.push(joltage_ratings.last().expect("Empty vector.") + 3);

            let mut differences: Vec<i32> = vec![];
            for i in 1..joltage_ratings.len() {
                differences.push(joltage_ratings[i] - joltage_ratings[i - 1]);
            }

            match problem {
                super::Problem::One => problem_one(&differences),
                super::Problem::Two => problem_two(&differences),
            }
        }

        fn problem_one(differences: &[i32]) -> Option<i64> {
            Some(
                differences.iter().filter(|x| **x == 1).count() as i64
                    * differences.iter().filter(|x| **x == 3).count() as i64,
            )
        }

        fn problem_two(differences: &[i32]) -> Option<i64> {
            Some(
                differences
                    .split(|num| *num == 3)
                    .filter(|n| !n.is_empty())
                    .map(|x| calculate_combinations(x.len() as i64))
                    .product(),
            )
        }

        fn calculate_combinations(num: i64) -> i64 {
            let total = num - 1;
            let mut num_choices = total;
            let mut num_combinations = 0;
            loop {
                if (total - num_choices > 2) || (num_choices < 0) {
                    break;
                } else if num_choices == 0 {
                    num_combinations += 1;
                    break;
                } else {
                    num_combinations += count_combinations(total, num_choices);
                    num_choices -= 1;
                }
            }
            num_combinations
        }

        fn count_combinations(n: i64, r: i64) -> i64 {
            if r > n {
                0
            } else {
                (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
            }
        }
    }

    pub mod day_eleven {
        use crate::aoc::lines_from_file;

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum Seat {
            Empty,
            Occupied,
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        struct Grid {
            num_rows: usize,
            num_cols: usize,
            layout: Vec<Vec<Option<Seat>>>,
        }

        impl Grid {
            fn initialize(filename: &str) -> Grid {
                let positions = lines_from_file(filename);
                let mut layout: Vec<Vec<Option<Seat>>> = vec![];
                for pos in &positions {
                    layout.push(
                        pos.chars()
                            .map(|c| match c {
                                'L' => Some(Seat::Empty),
                                '#' => Some(Seat::Occupied),
                                _ => None,
                            })
                            .collect(),
                    );
                }
                Grid {
                    num_rows: positions.len(),
                    num_cols: positions[0].len(),
                    layout,
                }
            }

            fn next(&mut self, problem: super::Problem) {
                match problem {
                    super::Problem::One => self.p1_next(),
                    super::Problem::Two => self.p2_next(),
                }
            }

            fn p1_next(&mut self) {
                let temp = self.clone();
                for i in 0..temp.num_rows {
                    for j in 0..temp.num_cols {
                        let num_occupied = temp.count_occupied_adjacent_seats(i, j);
                        match temp.layout[i][j] {
                            Some(Seat::Empty) => {
                                if num_occupied == 0 {
                                    self.layout[i][j] = Some(Seat::Occupied);
                                }
                            }
                            Some(Seat::Occupied) => {
                                if num_occupied >= 4 {
                                    self.layout[i][j] = Some(Seat::Empty);
                                }
                            }
                            None => (),
                        };
                    }
                }
            }

            fn p2_next(&mut self) {
                let temp = self.clone();
                for i in 0..temp.num_rows {
                    for j in 0..temp.num_cols {
                        let num_occupied = temp.count_visible_occupied_seats(i, j);
                        match temp.layout[i][j] {
                            Some(Seat::Empty) => {
                                if num_occupied == 0 {
                                    self.layout[i][j] = Some(Seat::Occupied);
                                }
                            }
                            Some(Seat::Occupied) => {
                                if num_occupied >= 5 {
                                    self.layout[i][j] = Some(Seat::Empty);
                                }
                            }
                            None => (),
                        };
                    }
                }
            }

            fn count_visible_occupied_seats(&self, row: usize, col: usize) -> usize {
                self.count_horizontal(row, col)
                    + self.count_vertical(row, col)
                    + self.count_diagonal_sw_ne(row, col)
                    + self.count_diagonal_nw_se(row, col)
            }

            fn count_horizontal(&self, row: usize, col: usize) -> usize {
                let mut total = 0;
                for j in col + 1..self.num_cols {
                    match self.layout[row][j] {
                        Some(Seat::Occupied) => {
                            total += 1;
                            break;
                        }
                        Some(Seat::Empty) => break,
                        _ => (),
                    }
                }
                for j in (0..col).rev() {
                    match self.layout[row][j] {
                        Some(Seat::Occupied) => {
                            total += 1;
                            break;
                        }
                        Some(Seat::Empty) => break,
                        _ => (),
                    }
                }
                total
            }

            fn count_vertical(&self, row: usize, col: usize) -> usize {
                let mut total = 0;
                for i in row + 1..self.num_rows {
                    match self.layout[i][col] {
                        Some(Seat::Occupied) => {
                            total += 1;
                            break;
                        }
                        Some(Seat::Empty) => break,
                        _ => (),
                    }
                }
                for i in (0..row).rev() {
                    match self.layout[i][col] {
                        Some(Seat::Occupied) => {
                            total += 1;
                            break;
                        }
                        Some(Seat::Empty) => break,
                        _ => (),
                    }
                }
                total
            }

            fn count_diagonal_sw_ne(&self, row: usize, col: usize) -> usize {
                let mut total = 0;
                for (i, j) in (0..row).rev().zip(col + 1..self.num_cols) {
                    match self.layout[i][j] {
                        Some(Seat::Occupied) => {
                            total += 1;
                            break;
                        }
                        Some(Seat::Empty) => break,
                        _ => (),
                    }
                }
                for (i, j) in (row + 1..self.num_rows).zip((0..col).rev()) {
                    match self.layout[i][j] {
                        Some(Seat::Occupied) => {
                            total += 1;
                            break;
                        }
                        Some(Seat::Empty) => break,
                        _ => (),
                    }
                }
                total
            }

            fn count_diagonal_nw_se(&self, row: usize, col: usize) -> usize {
                let mut total = 0;
                for (i, j) in (0..row).rev().zip((0..col).rev()) {
                    match self.layout[i][j] {
                        Some(Seat::Occupied) => {
                            total += 1;
                            break;
                        }
                        Some(Seat::Empty) => break,
                        _ => (),
                    }
                }
                for (i, j) in (row + 1..self.num_rows).zip(col + 1..self.num_cols) {
                    match self.layout[i][j] {
                        Some(Seat::Occupied) => {
                            total += 1;
                            break;
                        }
                        Some(Seat::Empty) => break,
                        _ => (),
                    }
                }
                total
            }

            fn count_occupied_adjacent_seats(&self, row: usize, col: usize) -> usize {
                let mut rows_to_check = vec![row];
                if row > 0 {
                    rows_to_check.push(row - 1);
                }
                if row < self.num_rows - 1 {
                    rows_to_check.push(row + 1);
                }

                let mut cols_to_check = vec![col];
                if col > 0 {
                    cols_to_check.push(col - 1);
                }
                if col < self.num_cols - 1 {
                    cols_to_check.push(col + 1);
                }

                let mut total_occupied = 0;

                for i_row in &rows_to_check {
                    for j_col in &cols_to_check {
                        if *i_row == row && *j_col == col {
                            continue;
                        }
                        if let Some(Seat::Occupied) = self.layout[*i_row][*j_col] {
                            total_occupied += 1
                        }
                    }
                }

                total_occupied
            }

            fn total_occupied(&self) -> usize {
                let mut total_occupied = 0;
                for i in 0..self.num_rows {
                    for j in 0..self.num_cols {
                        if let Some(Seat::Occupied) = self.layout[i][j] {
                            total_occupied += 1
                        }
                    }
                }

                total_occupied
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        struct State {
            round: usize,
            grid: Grid,
        }

        impl State {
            fn initialize(grid: Grid) -> State {
                State { round: 0, grid }
            }

            fn next(&mut self, problem: super::Problem) {
                self.round += 1;
                self.grid.next(problem);
            }
        }

        pub fn solve(problem: super::Problem, filename: &str) -> Option<i32> {
            let mut current_state = State::initialize(Grid::initialize(filename));
            let mut previous_state = current_state.clone();

            current_state.next(problem);
            loop {
                if current_state.grid == previous_state.grid {
                    break;
                }
                previous_state = current_state.clone();
                current_state.next(problem);
            }

            Some(current_state.grid.total_occupied() as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_one() {
        let filename = "./misc/D01.txt";
        let p1 = aoc::day_one::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_one::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, 800139);
        assert_eq!(p2, 59885340);
    }

    #[test]
    fn day_two() {
        let filename = "./misc/D02.txt";
        let p1 = aoc::day_two::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_two::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, 515);
        assert_eq!(p2, 711);
    }

    #[test]
    fn day_three() {
        let filename = "./misc/D03.txt";
        let p1 = aoc::day_three::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_three::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, 220);
        assert_eq!(p2, 2138320800);
    }

    #[test]
    fn day_four() {
        let filename = "./misc/D04.txt";
        let p1 = aoc::day_four::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_four::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, 239);
        assert_eq!(p2, 188);
    }

    #[test]
    fn day_five() {
        let filename = "./misc/D05.txt";
        let p1 = aoc::day_five::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_five::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, 892);
        assert_eq!(p2, 625);
    }

    #[test]
    fn day_six() {
        let filename = "./misc/D06.txt";
        let p1 = aoc::day_six::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_six::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, 6351);
        assert_eq!(p2, 3143);
    }

    #[test]
    fn day_seven() {
        let filename = "./misc/D07.txt";
        let p1 = aoc::day_seven::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_seven::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, 252);
        assert_eq!(p2, 35487);
    }

    #[test]
    fn day_eight() {
        let filename = "./misc/D08.txt";
        let p1 = aoc::day_eight::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_eight::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(1528));
        assert_eq!(p2, Some(640));
    }

    #[test]
    fn day_nine() {
        let filename = "./misc/D09.txt";
        let p1 = aoc::day_nine::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_nine::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(248131121));
        assert_eq!(p2, Some(31580383));
    }

    #[test]
    fn day_ten() {
        let filename = "./misc/D10.txt";
        let p1 = aoc::day_ten::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_ten::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(1980));
        assert_eq!(p2, Some(4628074479616));
    }

    #[test]
    fn day_eleven() {
        let filename = "./misc/D11.txt";
        let p1 = aoc::day_eleven::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_eleven::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(2494));
        assert_eq!(p2, Some(2306));
    }
}
