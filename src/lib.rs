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

    pub mod day_twelve {
        use std::mem::swap;

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum Direction {
            North,
            South,
            East,
            West,
            Left,
            Right,
            Forward,
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct Instruction {
            direction: Direction,
            value: isize,
        }

        impl Instruction {
            fn new(instruction: &str) -> Instruction {
                let dir = instruction.chars().next().expect("No direction to parse.");
                let value: isize = instruction
                    .get(1..)
                    .expect("No value to parse.")
                    .parse()
                    .expect("Value could not be parsed into type `isize`.");

                let direction = match dir {
                    'N' => Some(Direction::North),
                    'S' => Some(Direction::South),
                    'E' => Some(Direction::East),
                    'W' => Some(Direction::West),
                    'L' => Some(Direction::Left),
                    'R' => Some(Direction::Right),
                    'F' => Some(Direction::Forward),
                    _ => None,
                };

                Instruction {
                    direction: direction.expect("Invalid direction."),
                    value,
                }
            }
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum Facing {
            North,
            South,
            East,
            West,
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct Location {
            x: isize,
            y: isize,
        }

        trait State {
            fn initialize() -> Self
            where
                Self: Sized;
            fn update(&mut self, instruction: Instruction);
            fn manhattan_distance(&self) -> isize;
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct P1State {
            facing: Facing,
            position: Location,
        }

        impl P1State {
            fn turn_left(&mut self) {
                match self.facing {
                    Facing::North => self.facing = Facing::West,
                    Facing::West => self.facing = Facing::South,
                    Facing::South => self.facing = Facing::East,
                    Facing::East => self.facing = Facing::North,
                };
            }

            fn turn_right(&mut self) {
                match self.facing {
                    Facing::North => self.facing = Facing::East,
                    Facing::East => self.facing = Facing::South,
                    Facing::South => self.facing = Facing::West,
                    Facing::West => self.facing = Facing::North,
                };
            }

            fn move_forward(&mut self, value: isize) {
                match self.facing {
                    Facing::North => self.position.y += value,
                    Facing::South => self.position.y -= value,
                    Facing::East => self.position.x += value,
                    Facing::West => self.position.x -= value,
                };
            }
        }

        impl State for P1State {
            fn initialize() -> P1State {
                P1State {
                    facing: Facing::East,
                    position: Location { x: 0, y: 0 },
                }
            }

            fn update(&mut self, instruction: Instruction) {
                match instruction.direction {
                    Direction::Left => {
                        let num_turns = instruction.value / 90;
                        for _i in 0..num_turns {
                            self.turn_left();
                        }
                    }
                    Direction::Right => {
                        let num_turns = instruction.value / 90;
                        for _i in 0..num_turns {
                            self.turn_right();
                        }
                    }
                    Direction::North => self.position.y += instruction.value,
                    Direction::South => self.position.y -= instruction.value,
                    Direction::East => self.position.x += instruction.value,
                    Direction::West => self.position.x -= instruction.value,
                    Direction::Forward => self.move_forward(instruction.value),
                };
            }

            fn manhattan_distance(&self) -> isize {
                self.position.x.abs() + self.position.y.abs()
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct P2State {
            waypoint: Location,
            position: Location,
        }

        impl P2State {
            fn rotate_waypoint_left(&mut self) {
                swap(&mut self.waypoint.x, &mut self.waypoint.y);
                self.waypoint.x *= -1;
            }

            fn rotate_waypoint_right(&mut self) {
                swap(&mut self.waypoint.x, &mut self.waypoint.y);
                self.waypoint.y *= -1;
            }

            fn move_forward(&mut self, value: isize) {
                for _i in 0..value {
                    self.position.x += self.waypoint.x;
                    self.position.y += self.waypoint.y;
                }
            }
        }

        impl State for P2State {
            fn initialize() -> P2State {
                P2State {
                    waypoint: Location { x: 10, y: 1 },
                    position: Location { x: 0, y: 0 },
                }
            }

            fn update(&mut self, instruction: Instruction) {
                match instruction.direction {
                    Direction::North => self.waypoint.y += instruction.value,
                    Direction::South => self.waypoint.y -= instruction.value,
                    Direction::East => self.waypoint.x += instruction.value,
                    Direction::West => self.waypoint.x -= instruction.value,
                    Direction::Left => {
                        let num_turns = instruction.value / 90;
                        for _i in 0..num_turns {
                            self.rotate_waypoint_left();
                        }
                    }
                    Direction::Right => {
                        let num_turns = instruction.value / 90;
                        for _i in 0..num_turns {
                            self.rotate_waypoint_right();
                        }
                    }
                    Direction::Forward => self.move_forward(instruction.value),
                }
            }

            fn manhattan_distance(&self) -> isize {
                self.position.x.abs() + self.position.y.abs()
            }
        }

        pub fn solve(problem: super::Problem, file: &str) -> Option<isize> {
            let instructions: Vec<Instruction> = super::lines_from_file(file)
                .iter()
                .map(|x| Instruction::new(x))
                .collect();

            let mut state: Box<dyn State> = match problem {
                super::Problem::One => Box::new(P1State::initialize()),
                super::Problem::Two => Box::new(P2State::initialize()),
            };

            for instruction in instructions {
                state.update(instruction);
            }

            Some(state.manhattan_distance())
        }
    }

    pub mod day_thirteen {
        use crate::aoc::lines_from_file;

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct P1Bus {
            id: usize,
            wait_time: usize,
        }

        impl P1Bus {
            fn new(id: usize, earliest_departure_time: usize) -> P1Bus {
                P1Bus {
                    id,
                    wait_time: ((earliest_departure_time as f32 / id as f32).ceil() as usize * id)
                        - earliest_departure_time,
                }
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct P2Bus {
            id: usize,
            position: usize,
        }

        impl P2Bus {
            fn new(id: usize, position: usize) -> P2Bus {
                P2Bus { id, position }
            }
        }

        pub fn solve(problem: super::Problem, filename: &str) -> Option<usize> {
            match problem {
                super::Problem::One => problem_one(filename),
                super::Problem::Two => problem_two(filename),
            }
        }

        fn problem_one(filename: &str) -> Option<usize> {
            let input = lines_from_file(filename);
            let earliest_departure_time: usize = input[0]
                .parse()
                .expect("Could not parse first line of input.");
            let bus_ids: Vec<usize> = input[1].split(',').filter_map(|s| s.parse().ok()).collect();
            let bus_list: Vec<P1Bus> = bus_ids
                .iter()
                .map(|x| P1Bus::new(*x, earliest_departure_time))
                .collect();

            let mut shortest_wait_bus = bus_list[0].clone();
            for bus in bus_list {
                if bus.wait_time < shortest_wait_bus.wait_time {
                    shortest_wait_bus = bus;
                }
            }

            Some(shortest_wait_bus.wait_time * shortest_wait_bus.id)
        }

        fn problem_two(filename: &str) -> Option<usize> {
            let input = lines_from_file(filename);
            // ignore input[0], as it is no longer relevant
            let bus_ids: Vec<Option<usize>> = input[1].split(',').map(|s| s.parse().ok()).collect();
            let mut bus_list: Vec<P2Bus> = vec![];
            for (i, bus_id) in bus_ids.into_iter().enumerate() {
                if let Some(x) = bus_id {
                    bus_list.push(P2Bus::new(x, i));
                }
            }

            let mut largest_bus_id_bus: P2Bus = P2Bus::new(0, 0);
            for bus in &bus_list {
                if bus.id > largest_bus_id_bus.id {
                    largest_bus_id_bus = bus.clone();
                }
            }

            // n is set to one-hundred billion due to the hint in the problem, which states that
            // the timestamp we are looking for is greater than one-hundred trillion.
            // The loop below probably takes well over 12 hours to complete.
            let mut n: usize = 100_000_000_000;
            loop {
                let timestamp: usize = largest_bus_id_bus.id * n - largest_bus_id_bus.position;
                if check_all_buses(&bus_list, timestamp) {
                    break;
                }
                n += 1;
            }

            Some(largest_bus_id_bus.id * n - largest_bus_id_bus.position)
        }

        fn check_all_buses(bus_list: &[P2Bus], t: usize) -> bool {
            for bus in bus_list {
                if (t + bus.position) % bus.id != 0 {
                    return false;
                }
            }
            true
        }
    }

    pub mod day_fourteen {
        use crate::aoc::lines_from_file;
        use itertools::Itertools;
        use std::collections::HashMap;

        struct Mask {
            save: usize,
            transfer: usize,
        }

        struct Memory {
            address: usize,
            value: usize,
        }

        struct MaskP2 {
            xs_zeroed: usize,
            x_positions: Vec<usize>,
        }

        pub fn solve(problem: super::Problem, filename: &str) -> Option<usize> {
            let input = lines_from_file(filename);
            match problem {
                super::Problem::One => solve_p1(&input),
                super::Problem::Two => solve_p2(&input),
            }
        }

        fn parse_mask_p1(line: &str) -> Mask {
            let mask = line
                .strip_prefix("mask = ")
                .expect("Error parsing mask assignment instruction.");
            let transfer = usize::from_str_radix(&*mask.replace("X", "0"), 2)
                .expect("Error parsing mask string to usize.");
            let save = usize::from_str_radix(
                &*mask
                    .replace("0", "Z")
                    .replace("1", "Z")
                    .replace("X", "1")
                    .replace("Z", "0"),
                2,
            )
            .expect("Error parsing mask string to usize.");
            Mask { save, transfer }
        }

        fn solve_p1(input: &[String]) -> Option<usize> {
            let mut mask = Mask {
                save: 0,
                transfer: 0,
            };
            let mut mem = HashMap::new();

            for line in input {
                if line.contains("mask") {
                    mask = parse_mask_p1(&line);
                } else if line.contains("mem") {
                    let memory = parse_assignment(&line);
                    mem.insert(memory.address, (memory.value & mask.save) ^ mask.transfer);
                } else {
                    panic!("Input instructions are ill-formatted.")
                }
            }
            Some(mem.values().sum())
        }

        fn solve_p2(input: &[String]) -> Option<usize> {
            let mut mask = MaskP2 {
                xs_zeroed: 0,
                x_positions: vec![],
            };
            let mut mem = HashMap::new();

            for line in input {
                if line.contains("mask") {
                    mask = parse_mask_p2(&line);
                } else if line.contains("mem") {
                    let mut memory = parse_assignment(&line);
                    // First we zero out the x's of the mask (which is done by parse_mask_p2 and
                    // returned as part of mask) and combine that with the address.
                    memory.address |= mask.xs_zeroed;
                    // Then we zero out the positions where the x's should have been.
                    for pos in &mask.x_positions {
                        memory.address &= !(1 << pos);
                    }
                    let addresses = generate_addresses(&mask, &memory);
                    for address in addresses {
                        mem.insert(address, memory.value);
                    }
                } else {
                    panic!("Input instructions are ill-formatted.")
                }
            }
            Some(mem.values().sum())
        }

        fn generate_addresses(mask: &MaskP2, memory: &Memory) -> Vec<usize> {
            let mask_list: Vec<usize> = mask.x_positions.iter().map(|x| 1 << x).collect();
            let mut addresses: Vec<usize> = vec![];
            for i in 0..mask_list.len() {
                addresses.extend(
                    mask_list
                        .iter()
                        .combinations(i + 1)
                        .map(|v| v.into_iter().sum::<usize>()),
                );
            }
            addresses.iter_mut().for_each(|x| *x += memory.address);
            addresses.push(memory.address);
            addresses
        }

        fn parse_mask_p2(line: &str) -> MaskP2 {
            let mask = line
                .strip_prefix("mask = ")
                .expect("Error parsing mask assignment instruction.");
            let xs_zeroed = usize::from_str_radix(&*mask.replace("X", "0"), 2)
                .expect("Error parsing mask string to usize.");
            let x_positions = mask.match_indices('X').map(|x| 35 - x.0).collect();
            MaskP2 {
                xs_zeroed,
                x_positions,
            }
        }

        fn parse_assignment(line: &str) -> Memory {
            let memory_instruction: Vec<&str> = line.split("] = ").collect();
            let address = memory_instruction[0]
                .strip_prefix("mem[")
                .expect("Memory assignment line is ill-formatted.")
                .parse()
                .expect("Error parsing memory address.");
            let value = memory_instruction[1]
                .parse()
                .expect("Error parsing value assigned to memory address.");
            Memory { address, value }
        }
    }

    pub mod day_fifteen {
        use crate::aoc::lines_from_file;
        use std::collections::HashMap;

        pub fn solve(problem: super::Problem, filename: &str) -> Option<usize> {
            let input = lines_from_file(filename);
            let starting_numbers: Vec<usize> = input[0]
                .split(',')
                .map(|n| n.parse().expect("Could not parse starting number."))
                .collect();
            let final_turn = match problem {
                super::Problem::One => 2020,
                super::Problem::Two => 30_000_000,
            };

            let mut sequence = HashMap::new();
            let mut turn: usize = 1;
            for num in &starting_numbers {
                sequence.insert(*num, vec![turn]);
                turn += 1;
            }

            let mut most_recent_num = *starting_numbers
                .last()
                .expect("List of starting numbers is empty.");

            loop {
                if turn == final_turn + 1 {
                    return Some(most_recent_num);
                }
                // The next line is guaranteed to not fail (most_recent_num will be present
                // because by definition, it is the last thing we added to sequence).
                if sequence.get(&most_recent_num).unwrap().len() == 1 {
                    sequence
                        .entry(0)
                        .and_modify(|e| e.push(turn))
                        .or_insert_with(|| vec![turn]);
                    most_recent_num = 0;
                } else {
                    // The next line is guaranteed to not fail (most_recent_num will be present
                    // because by definition, it is the last thing we added to sequence).
                    let prev_turns = sequence.get(&most_recent_num).unwrap();
                    let next_num =
                        prev_turns[prev_turns.len() - 1] - prev_turns[prev_turns.len() - 2];
                    sequence
                        .entry(next_num)
                        .and_modify(|e| e.push(turn))
                        .or_insert_with(|| vec![turn]);
                    most_recent_num = next_num;
                }
                turn += 1;
            }
        }
    }

    pub mod day_sixteen {
        use crate::aoc::lines_from_file;
        use itertools::Itertools;
        use std::collections::HashSet;

        #[derive(Debug, Clone, Eq, PartialEq)]
        struct ValidRange {
            begin: usize,
            end: usize,
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        struct Category {
            name: String,
            valid_ranges: (ValidRange, ValidRange),
        }

        impl Category {
            fn is_num_valid(&self, num: &usize) -> bool {
                (self.valid_ranges.0.begin..=self.valid_ranges.0.end).contains(num)
                    || (self.valid_ranges.1.begin..=self.valid_ranges.1.end).contains(num)
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        struct Info {
            categories: Vec<Category>,
            my_ticket: Vec<usize>,
            nearby_tickets: Vec<Vec<usize>>,
        }

        pub fn solve(problem: super::Problem, filename: &str) -> Option<usize> {
            let input = lines_from_file(filename);
            let collected_info = parse_input(&input);

            let mut invalid_values: Vec<usize> = vec![];
            let mut invalid_tickets: HashSet<Vec<usize>> = HashSet::new();
            for ticket in &collected_info.nearby_tickets {
                for num in ticket {
                    if !valid_number(*num, &collected_info.categories) {
                        invalid_values.push(*num);
                        invalid_tickets.insert(ticket.clone());
                    }
                }
            }

            let valid_tickets: Vec<&Vec<usize>> = collected_info
                .nearby_tickets
                .iter()
                .filter(|&x| !invalid_tickets.contains(x))
                .collect();
            let all_possible_label_positions = calculate_positions(&valid_tickets, &collected_info);
            let mut refined_possible_label_positions = refine_positions(
                &all_possible_label_positions,
                &collected_info,
                valid_tickets.len(),
            );
            let labels =
                calculate_final_labels(&collected_info, &mut refined_possible_label_positions);

            let mut total = 1;
            for (i, label) in labels.iter().enumerate() {
                if label.contains("departure") {
                    total *= collected_info.my_ticket[i];
                }
            }

            match problem {
                super::Problem::One => Some(invalid_values.iter().sum()),
                super::Problem::Two => Some(total),
            }
        }

        fn calculate_positions(
            valid_tickets: &[&Vec<usize>],
            info: &Info,
        ) -> Vec<Vec<HashSet<String>>> {
            let mut result: Vec<Vec<HashSet<String>>> = vec![];
            for ticket in valid_tickets {
                result.push(vec![]);
                for num in ticket.iter() {
                    let valid_set: HashSet<String> = info
                        .categories
                        .iter()
                        .filter(|c| c.is_num_valid(num))
                        .map(|c| c.name.to_string())
                        .collect();
                    result.last_mut().unwrap().push(valid_set);
                }
            }
            result
        }

        fn refine_positions(
            possible_positions: &[Vec<HashSet<String>>],
            info: &Info,
            num_valid_tickets: usize,
        ) -> Vec<HashSet<String>> {
            let mut reduced_set = vec![];
            for i in 0..info.categories.len() {
                let mut set = possible_positions[0][i].to_owned();
                for j in 0..num_valid_tickets - 1 {
                    set = set
                        .intersection(&possible_positions[j + 1][i])
                        .cloned()
                        .collect();
                }
                reduced_set.push(set);
            }
            reduced_set
        }

        fn calculate_final_labels(
            info: &Info,
            possible_label_positions: &mut Vec<HashSet<String>>,
        ) -> Vec<String> {
            let mut labels: Vec<String> = vec![];
            for _ in 0..info.categories.len() {
                labels.push("".to_string());
            }
            loop {
                if !labels.iter().any(|s| s.is_empty()) {
                    break;
                }
                for (i, possible_labels) in possible_label_positions.iter().enumerate() {
                    if possible_labels.len() == 1 {
                        labels[i] = possible_labels.iter().collect_vec()[0].to_string();
                    }
                }
                for label in &labels {
                    for elem in possible_label_positions.iter_mut() {
                        elem.remove(label);
                    }
                }
            }
            labels
        }

        fn valid_number(num: usize, categories: &[Category]) -> bool {
            for category in categories {
                if category.is_num_valid(&num) {
                    return true;
                }
            }
            false
        }

        fn parse_input(input: &[String]) -> Info {
            let mut categories: Vec<Category> = vec![];
            let mut my_ticket: Vec<usize> = vec![];
            let mut nearby_tickets: Vec<Vec<usize>> = vec![];
            let mut index: usize = 0;
            loop {
                if index >= input.len() {
                    break;
                }
                let line = &input[index];
                if line.contains(" or ") {
                    let range_definition: Vec<&str> = line.split(": ").collect();
                    let name = range_definition[0].to_string();
                    let valid_ranges = range_definition[1]
                        .split(" or ")
                        .map(|s| parse_range(s))
                        .collect_tuple()
                        .expect("Error parsing range.");
                    categories.push(Category { name, valid_ranges });
                    index += 1;
                } else if line.contains("your") {
                    index += 1;
                    my_ticket = parse_ticket(&input[index]);
                    index += 1;
                } else if line.contains("nearby") {
                    index += 1;
                    while index < input.len() {
                        nearby_tickets.push(parse_ticket(&input[index]));
                        index += 1;
                    }
                } else {
                    index += 1;
                }
            }
            Info {
                categories,
                my_ticket,
                nearby_tickets,
            }
        }

        fn parse_range(range: &str) -> ValidRange {
            let num_range: Vec<&str> = range.split('-').collect();
            ValidRange {
                begin: num_range[0]
                    .parse()
                    .expect("Could not parse range beginning."),
                end: num_range[1].parse().expect("Could not parse range end."),
            }
        }

        fn parse_ticket(line: &str) -> Vec<usize> {
            let numbers: Vec<&str> = line.split(',').collect();
            numbers
                .iter()
                .map(|x| x.parse().expect("Could not parse ticket number."))
                .collect()
        }
    }

    pub mod day_seventeen {
        use crate::aoc::lines_from_file;
        use std::collections::BTreeSet;
        use std::convert::TryFrom;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
        struct Point<T> {
            x: T,
            y: T,
            z: T,
            w: T,
        }

        pub fn solve(problem: super::Problem, filename: &str) -> Option<usize> {
            let input = lines_from_file(filename);
            let active = parse_initial_state(problem, &input);
            let mut inactive = generate_inactive_set(problem);
            inactive.retain(|p| !active.contains(p));

            let final_active_total = run_cycles(6, active, inactive);

            Some(final_active_total)
        }

        fn run_cycles(num_cycles: usize, mut active: BTreeSet<Point<isize>>, mut inactive: BTreeSet<Point<isize>>) -> usize {
            for _i in 0..num_cycles {
                let active_copy = active.clone();
                let inactive_copy = inactive.clone();
                for point in &active_copy {
                    let active_count = count_active_neighbors(point, &active_copy);
                    if !(2..=3).contains(&active_count) {
                        active.remove(point);
                        inactive.insert(point.clone());
                    }
                }
                for point in &inactive_copy {
                    let active_count = count_active_neighbors(point, &active_copy);
                    if active_count == 3 {
                        active.insert(point.clone());
                        inactive.remove(point);
                    }
                }
            }
            active.len()
        }

        fn count_active_neighbors(point: &Point<isize>, active: &BTreeSet<Point<isize>>) -> usize {
            active
                .iter()
                .filter(|&p| {
                    (p.x - point.x != 0)
                        || (p.y - point.y != 0)
                        || (p.z - point.z != 0)
                        || (p.w - point.w != 0)
                })
                .filter(|&p| {
                    ((p.x - point.x).abs() <= 1)
                        && ((p.y - point.y).abs() <= 1)
                        && ((p.z - point.z).abs() <= 1)
                        && ((p.w - point.w).abs() <= 1)
                })
                .count()
        }

        fn parse_initial_state(problem: super::Problem, input: &[String]) -> BTreeSet<Point<isize>> {
            let mut active = BTreeSet::new();
            for (y, line) in input.iter().enumerate() {
                for (x, char) in line.chars().into_iter().enumerate() {
                    if char == '#' {
                        match problem {
                            super::Problem::One => {
                                active.insert(Point {
                                    x: isize::try_from(x).unwrap() + 11,
                                    y: isize::try_from(y).unwrap() + 11,
                                    z: 12,
                                    w: 0,
                                });
                            }
                            super::Problem::Two => {
                                active.insert(Point {
                                    x: isize::try_from(x).unwrap() + 11,
                                    y: isize::try_from(y).unwrap() + 11,
                                    z: 12,
                                    w: 12,
                                });
                            }
                        }
                    }
                }
            }
            active
        }

        fn generate_inactive_set(problem: super::Problem) -> BTreeSet<Point<isize>> {
            let mut inactive = BTreeSet::new();
            for x in 0..25 {
                for y in 0..25 {
                    for z in 0..25 {
                        match problem {
                            super::Problem::One => {
                                inactive.insert(Point { x, y, z, w: 0 });
                            }
                            super::Problem::Two => {
                                for w in 0..25 {
                                    inactive.insert(Point { x, y, z, w });
                                }
                            }
                        }
                    }
                }
            }
            inactive
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

    #[test]
    fn day_twelve() {
        let filename = "./misc/D12.txt";
        let p1 = aoc::day_twelve::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_twelve::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(1631));
        assert_eq!(p2, Some(58606));
    }

    #[test]
    fn day_thirteen() {
        let filename = "./misc/D13.txt";
        let p1 = aoc::day_thirteen::solve(aoc::Problem::One, filename);
        assert_eq!(p1, Some(2165));
        // Omit the second test because it would take over 12 hours to complete.
        // let p2 = aoc::day_thirteen::solve(aoc::Problem::Two, filename);
        // assert_eq!(p2, Some(534035653563227));
    }

    #[test]
    fn day_fourteen() {
        let filename = "./misc/D14.txt";
        let p1 = aoc::day_fourteen::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_fourteen::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(10452688630537));
        assert_eq!(p2, Some(2881082759597));
    }

    #[test]
    fn day_fifteen() {
        let filename = "./misc/D15.txt";
        let p1 = aoc::day_fifteen::solve(aoc::Problem::One, filename);
        // This test takes significantly longer than the others to run. (~60 sec vs ~6 sec)
        // let p2 = aoc::day_fifteen::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(403));
        // assert_eq!(p2, Some(6823));
    }

    #[test]
    fn day_sixteen() {
        let filename = "./misc/D16.txt";
        let p1 = aoc::day_sixteen::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_sixteen::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(26053));
        assert_eq!(p2, Some(1515506256421));
    }

    #[test]
    fn day_seventeen() {
        let filename = "./misc/D17.txt";
        let p1 = aoc::day_seventeen::solve(aoc::Problem::One, filename);
        let p2 = aoc::day_seventeen::solve(aoc::Problem::Two, filename);
        assert_eq!(p1, Some(336));
        assert_eq!(p2, Some(2620));
    }
}
