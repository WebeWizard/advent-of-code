const STACKS_INPUT: &str = "[G]                 [D] [R]        
[W]         [V]     [C] [T] [M]    
[L]         [P] [Z] [Q] [F] [V]    
[J]         [S] [D] [J] [M] [T] [V]
[B]     [M] [H] [L] [Z] [J] [B] [S]
[R] [C] [T] [C] [T] [R] [D] [R] [D]
[T] [W] [Z] [T] [P] [B] [B] [H] [P]
[D] [S] [R] [D] [G] [F] [S] [L] [Q]
 1   2   3   4   5   6   7   8   9 ";

const MOVES_INPUT: &str = "move 1 from 3 to 5
move 5 from 5 to 4
move 6 from 7 to 3
move 6 from 1 to 3
move 1 from 1 to 9
move 1 from 1 to 4
move 3 from 6 to 9
move 2 from 7 to 5
move 1 from 5 to 7
move 1 from 7 to 2
move 2 from 2 to 5
move 2 from 6 to 3
move 6 from 8 to 9
move 7 from 3 to 9
move 1 from 8 to 7
move 8 from 9 to 7
move 5 from 4 to 8
move 1 from 6 to 2
move 2 from 8 to 4
move 9 from 9 to 1
move 2 from 8 to 5
move 1 from 8 to 5
move 5 from 9 to 2
move 1 from 6 to 8
move 5 from 1 to 7
move 1 from 8 to 2
move 2 from 1 to 7
move 1 from 2 to 6
move 4 from 5 to 4
move 2 from 1 to 4
move 13 from 7 to 8
move 3 from 8 to 6
move 2 from 6 to 8
move 10 from 3 to 5
move 2 from 7 to 6
move 3 from 5 to 6
move 10 from 8 to 1
move 1 from 8 to 6
move 6 from 2 to 4
move 1 from 5 to 8
move 5 from 6 to 3
move 2 from 8 to 6
move 1 from 7 to 9
move 2 from 2 to 7
move 3 from 5 to 1
move 2 from 7 to 2
move 6 from 6 to 3
move 7 from 5 to 6
move 5 from 3 to 2
move 10 from 1 to 8
move 2 from 1 to 3
move 8 from 3 to 7
move 9 from 4 to 8
move 1 from 9 to 2
move 2 from 7 to 8
move 4 from 6 to 9
move 1 from 4 to 9
move 5 from 7 to 4
move 3 from 6 to 5
move 1 from 1 to 5
move 14 from 4 to 8
move 3 from 9 to 7
move 4 from 5 to 9
move 2 from 4 to 1
move 27 from 8 to 6
move 2 from 7 to 2
move 2 from 7 to 4
move 4 from 2 to 9
move 7 from 8 to 4
move 10 from 4 to 1
move 18 from 6 to 5
move 6 from 9 to 2
move 1 from 9 to 5
move 11 from 2 to 6
move 2 from 5 to 4
move 1 from 2 to 8
move 2 from 4 to 9
move 2 from 8 to 3
move 1 from 6 to 8
move 4 from 9 to 7
move 4 from 7 to 8
move 7 from 5 to 1
move 4 from 6 to 3
move 2 from 3 to 7
move 6 from 5 to 3
move 2 from 8 to 2
move 14 from 6 to 2
move 3 from 8 to 1
move 15 from 2 to 3
move 1 from 6 to 1
move 14 from 3 to 2
move 2 from 2 to 5
move 1 from 9 to 3
move 13 from 1 to 3
move 4 from 2 to 6
move 10 from 1 to 3
move 2 from 6 to 9
move 6 from 2 to 9
move 6 from 5 to 2
move 2 from 6 to 8
move 7 from 9 to 5
move 1 from 5 to 8
move 2 from 7 to 6
move 34 from 3 to 6
move 19 from 6 to 2
move 12 from 6 to 9
move 3 from 6 to 3
move 2 from 3 to 2
move 1 from 6 to 5
move 17 from 2 to 8
move 2 from 3 to 2
move 8 from 9 to 4
move 7 from 5 to 2
move 5 from 4 to 1
move 4 from 1 to 6
move 1 from 1 to 6
move 6 from 6 to 8
move 2 from 8 to 4
move 17 from 8 to 6
move 2 from 4 to 5
move 17 from 6 to 9
move 22 from 9 to 7
move 1 from 5 to 2
move 20 from 2 to 7
move 29 from 7 to 9
move 1 from 4 to 7
move 3 from 8 to 3
move 1 from 8 to 5
move 3 from 8 to 2
move 2 from 2 to 4
move 27 from 9 to 7
move 2 from 3 to 2
move 1 from 5 to 2
move 18 from 7 to 5
move 1 from 3 to 2
move 1 from 5 to 6
move 18 from 5 to 3
move 1 from 6 to 3
move 2 from 9 to 5
move 10 from 3 to 5
move 4 from 3 to 6
move 1 from 7 to 1
move 1 from 5 to 1
move 6 from 7 to 6
move 1 from 6 to 2
move 4 from 4 to 8
move 5 from 5 to 4
move 1 from 3 to 8
move 2 from 1 to 8
move 2 from 2 to 5
move 3 from 3 to 8
move 6 from 8 to 2
move 1 from 3 to 9
move 1 from 6 to 3
move 6 from 2 to 8
move 7 from 8 to 4
move 8 from 5 to 2
move 5 from 4 to 6
move 2 from 8 to 3
move 2 from 3 to 9
move 1 from 3 to 9
move 2 from 7 to 1
move 2 from 1 to 2
move 12 from 2 to 4
move 1 from 9 to 7
move 1 from 6 to 2
move 9 from 7 to 9
move 1 from 8 to 2
move 9 from 9 to 8
move 6 from 7 to 8
move 4 from 4 to 1
move 6 from 2 to 5
move 1 from 4 to 9
move 3 from 1 to 9
move 6 from 4 to 5
move 5 from 8 to 9
move 8 from 4 to 6
move 3 from 9 to 8
move 1 from 9 to 3
move 3 from 8 to 3
move 5 from 9 to 2
move 3 from 2 to 6
move 3 from 6 to 9
move 3 from 6 to 2
move 4 from 2 to 6
move 6 from 9 to 7
move 1 from 1 to 8
move 8 from 8 to 5
move 20 from 5 to 3
move 2 from 2 to 8
move 6 from 7 to 1
move 10 from 6 to 3
move 4 from 6 to 7
move 4 from 1 to 9
move 2 from 1 to 2
move 3 from 6 to 9
move 5 from 8 to 3
move 3 from 7 to 9
move 17 from 3 to 2
move 1 from 6 to 2
move 2 from 6 to 9
move 1 from 6 to 4
move 12 from 9 to 2
move 1 from 4 to 7
move 8 from 3 to 8
move 8 from 8 to 9
move 7 from 9 to 2
move 1 from 9 to 7
move 18 from 2 to 9
move 1 from 7 to 2
move 2 from 7 to 1
move 1 from 1 to 2
move 4 from 2 to 7
move 15 from 9 to 3
move 1 from 9 to 1
move 2 from 1 to 8
move 6 from 2 to 4
move 8 from 2 to 1
move 2 from 8 to 5
move 2 from 9 to 3
move 4 from 4 to 1
move 2 from 5 to 8
move 2 from 8 to 9
move 14 from 3 to 1
move 2 from 9 to 7
move 2 from 4 to 3
move 1 from 2 to 9
move 5 from 7 to 9
move 21 from 1 to 9
move 2 from 1 to 6
move 3 from 2 to 4
move 1 from 7 to 3
move 19 from 9 to 5
move 1 from 2 to 7
move 1 from 7 to 2
move 3 from 4 to 2
move 19 from 5 to 7
move 2 from 2 to 5
move 1 from 5 to 3
move 1 from 3 to 4
move 8 from 9 to 4
move 1 from 6 to 3
move 1 from 2 to 6
move 1 from 2 to 1
move 8 from 7 to 3
move 5 from 4 to 7
move 2 from 6 to 4
move 1 from 5 to 9
move 1 from 1 to 6
move 1 from 1 to 2
move 2 from 4 to 7
move 1 from 4 to 2
move 2 from 4 to 9
move 1 from 6 to 8
move 1 from 1 to 5
move 1 from 8 to 6
move 1 from 1 to 4
move 25 from 3 to 1
move 1 from 4 to 2
move 2 from 3 to 6
move 3 from 1 to 9
move 6 from 9 to 8
move 1 from 6 to 3
move 1 from 2 to 9
move 15 from 7 to 6
move 2 from 2 to 6
move 1 from 3 to 8
move 1 from 1 to 4
move 6 from 8 to 4
move 1 from 3 to 8
move 1 from 8 to 5
move 2 from 5 to 2
move 8 from 6 to 7
move 1 from 8 to 7
move 1 from 9 to 4
move 9 from 4 to 5
move 19 from 1 to 3
move 9 from 3 to 5
move 6 from 7 to 2
move 2 from 1 to 7
move 7 from 2 to 4
move 7 from 5 to 6
move 5 from 4 to 3
move 3 from 5 to 8
move 1 from 2 to 4
move 2 from 4 to 8
move 14 from 6 to 1
move 6 from 5 to 6
move 1 from 5 to 2
move 7 from 1 to 6
move 1 from 2 to 4
move 4 from 6 to 4
move 1 from 5 to 4
move 2 from 1 to 9
move 2 from 9 to 4
move 2 from 1 to 8
move 9 from 3 to 6
move 3 from 7 to 4
move 4 from 8 to 6
move 3 from 7 to 6
move 1 from 7 to 2
move 1 from 7 to 5
move 3 from 8 to 4
move 26 from 6 to 1
move 8 from 1 to 2
move 1 from 6 to 4
move 5 from 2 to 7
move 2 from 2 to 4
move 10 from 4 to 7
move 1 from 6 to 1
move 22 from 1 to 2
move 1 from 6 to 1
move 6 from 4 to 7
move 1 from 5 to 1
move 1 from 1 to 2
move 21 from 7 to 2
move 38 from 2 to 3
move 8 from 2 to 6
move 2 from 4 to 8
move 2 from 8 to 2
move 1 from 1 to 3
move 1 from 2 to 8
move 1 from 2 to 5
move 6 from 6 to 4
move 2 from 4 to 2
move 2 from 2 to 6
move 1 from 8 to 2
move 28 from 3 to 1
move 11 from 1 to 2
move 8 from 1 to 7
move 4 from 6 to 4
move 8 from 3 to 1
move 8 from 2 to 5
move 6 from 5 to 4
move 2 from 5 to 4
move 8 from 3 to 4
move 22 from 4 to 1
move 2 from 3 to 5
move 33 from 1 to 5
move 26 from 5 to 6
move 4 from 5 to 7
move 2 from 2 to 7
move 2 from 7 to 2
move 2 from 7 to 8
move 2 from 8 to 3
move 6 from 1 to 3
move 5 from 5 to 1
move 1 from 5 to 7
move 7 from 7 to 5
move 4 from 5 to 6
move 5 from 1 to 8
move 4 from 2 to 4
move 2 from 7 to 4
move 2 from 7 to 3
move 5 from 4 to 6
move 1 from 8 to 2
move 1 from 2 to 4
move 10 from 3 to 6
move 44 from 6 to 9
move 2 from 5 to 7
move 1 from 5 to 8
move 41 from 9 to 1
move 1 from 6 to 4
move 2 from 8 to 1
move 1 from 7 to 3
move 1 from 3 to 8
move 2 from 9 to 8
move 29 from 1 to 9
move 2 from 1 to 5
move 2 from 8 to 3
move 1 from 3 to 5
move 2 from 5 to 9
move 1 from 5 to 7
move 25 from 9 to 2
move 10 from 2 to 1
move 1 from 7 to 8
move 2 from 4 to 1
move 2 from 8 to 9
move 1 from 8 to 6
move 4 from 2 to 4
move 4 from 2 to 5
move 1 from 6 to 5
move 1 from 2 to 7
move 2 from 4 to 1
move 18 from 1 to 3
move 8 from 9 to 4
move 15 from 3 to 9
move 3 from 4 to 8
move 4 from 5 to 8
move 4 from 2 to 4
move 10 from 9 to 4
move 4 from 8 to 5
move 2 from 7 to 2
move 11 from 4 to 9
move 12 from 4 to 9
move 2 from 5 to 7
move 4 from 2 to 4
move 5 from 8 to 1
move 1 from 5 to 6
move 1 from 4 to 6
move 1 from 3 to 9
move 1 from 5 to 7
move 4 from 1 to 6
move 6 from 1 to 5
move 6 from 5 to 9
move 3 from 7 to 6
move 9 from 6 to 5
move 8 from 5 to 2
move 7 from 2 to 3
move 1 from 3 to 1
move 7 from 3 to 5
move 2 from 4 to 1
move 1 from 2 to 6
move 2 from 1 to 3
move 8 from 5 to 9
move 3 from 1 to 3
move 1 from 6 to 1
move 2 from 4 to 1
move 1 from 5 to 2
move 2 from 1 to 6
move 2 from 6 to 3
move 2 from 3 to 2
move 2 from 2 to 4
move 1 from 2 to 6
move 3 from 3 to 9
move 2 from 4 to 8
move 3 from 3 to 1
move 4 from 1 to 7
move 2 from 8 to 4
move 7 from 9 to 6
move 1 from 1 to 4
move 11 from 9 to 7
move 3 from 9 to 3
move 14 from 9 to 5
move 6 from 6 to 5
move 4 from 5 to 9
move 10 from 7 to 6
move 1 from 3 to 7
move 2 from 4 to 1
move 4 from 7 to 9
move 9 from 6 to 1
move 3 from 6 to 5
move 15 from 9 to 1
move 1 from 4 to 7
move 4 from 9 to 7
move 12 from 5 to 1
move 3 from 7 to 3
move 4 from 7 to 2
move 1 from 9 to 3
move 22 from 1 to 2
move 21 from 2 to 6
move 3 from 1 to 9
move 1 from 3 to 7
move 1 from 7 to 3
move 1 from 3 to 2
move 8 from 1 to 4
move 1 from 9 to 2
move 7 from 4 to 8
move 3 from 3 to 9
move 3 from 3 to 5
move 4 from 2 to 3
move 1 from 1 to 3
move 4 from 8 to 5
move 2 from 8 to 3
move 5 from 3 to 2
move 6 from 5 to 3
move 2 from 5 to 8
move 2 from 1 to 7
move 2 from 7 to 4
move 15 from 6 to 9
move 8 from 3 to 1
move 3 from 5 to 9
move 2 from 4 to 9
move 8 from 1 to 3
move 8 from 9 to 8
move 1 from 1 to 4
move 3 from 5 to 9
move 4 from 8 to 1
move 1 from 3 to 9
move 2 from 4 to 3
move 2 from 8 to 6
move 3 from 8 to 7
move 8 from 2 to 5
move 3 from 5 to 2
move 4 from 3 to 4
move 3 from 6 to 1
move 2 from 5 to 9
move 4 from 4 to 1
move 2 from 5 to 6
move 1 from 5 to 4
move 2 from 2 to 1
move 4 from 3 to 9
move 1 from 7 to 3
move 2 from 7 to 4
move 2 from 4 to 7
move 1 from 6 to 7
move 1 from 2 to 8
move 2 from 3 to 9
move 14 from 1 to 8
move 1 from 6 to 2
move 2 from 7 to 1
move 3 from 8 to 3
move 6 from 8 to 5";

fn starting_stacks(stacks_input: &str) -> Vec<Vec<char>> {
    // ----- parse the stack input into a usable struct -----
    let stacks_helper: Vec<Vec<char>> = stacks_input
        .split('\n')
        .map(|line| line.chars().collect())
        .rev()
        .collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for (i, stack_num_char) in stacks_helper[0].iter().enumerate() {
        match stack_num_char.to_digit(10) {
            None => {}
            Some(stack_num) => {
                stacks.push(Vec::new());
                stacks[(stack_num as usize) - 1] = stacks_input
                    .split('\n')
                    .map(|line| line.chars().nth(i).unwrap())
                    .filter(|chr| chr.is_ascii_alphabetic())
                    .rev()
                    .collect()
            }
        }
    }
    return stacks;
}

fn move_crates(
    mut stacks: Vec<Vec<char>>,
    moves_input: &str,
    crate_mover_9001: bool,
) -> Vec<Vec<char>> {
    // ----- process the moves -----
    moves_input.split('\n').for_each(|command_str| {
        let command = command_str
            .split(' ')
            .filter_map(|part| part.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        let amount = command[0];
        let origin = command[1];
        let destination = command[2];

        let origin_length = stacks[origin - 1].len();
        let mut moving_crates: Vec<char> = stacks
            .get_mut(origin - 1)
            .unwrap()
            .split_off(origin_length - amount);

        if crate_mover_9001 {
            moving_crates.reverse();
        }

        stacks
            .get_mut(destination - 1)
            .unwrap()
            .append(&mut moving_crates);
    });

    return stacks;
}

fn main() {
    let mut enable_crate_mover_9001 = false;
    // ----- PART 1 -----
    let mut stacks = starting_stacks(STACKS_INPUT);
    stacks = move_crates(stacks, MOVES_INPUT, enable_crate_mover_9001);

    let result: String = stacks.iter().map(|line| *line.last().unwrap()).collect();

    println!("{:?}", result);

    // ----- PART 2 -----
    enable_crate_mover_9001 = true;
    let mut stacks = starting_stacks(STACKS_INPUT);
    stacks = move_crates(stacks, MOVES_INPUT, enable_crate_mover_9001);

    let result: String = stacks.iter().map(|line| *line.last().unwrap()).collect();

    println!("{:?}", result);
}
