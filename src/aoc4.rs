#![allow(dead_code)]
use itertools::Itertools;

#[derive(Debug)]
enum Bingo {
    Free(i32),
    Marked(i32),
}

pub fn run_part2(path: &str) -> i32 {
    let (boards, numbers) = read_data(path);
    solve2(boards, numbers)
}

pub fn run_part1(path: &str) -> i32 {
    let (boards, numbers) = read_data(path);
    solve1(boards, numbers)
}

fn read_data(path: &str) -> (Vec<Vec<Vec<Bingo>>>, Vec<i32>) {
    let data = std::fs::read_to_string(path).unwrap();
    let mut data = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(|n| n.to_string());

    let numbers = data
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec();

    let mut boards: Vec<Vec<Vec<Bingo>>> = Vec::new();

    while let Some(n) = data.next() {
        let mut board: Vec<Vec<Bingo>> = Vec::new();
        let line = n
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| Bingo::Free(x.parse::<i32>().unwrap()))
            .collect_vec();

        board.push(line);

        for _ in 0..4 {
            let line = data
                .next()
                .unwrap()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| Bingo::Free(x.parse::<i32>().unwrap()))
                .collect_vec();
            board.push(line);
        }
        boards.push(board);
    }

    (boards, numbers)
}

fn solve2(mut boards: Vec<Vec<Vec<Bingo>>>, numbers: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut winners = vec![false; boards.len()];
    for nr in numbers {
        for i in 0..boards.len() {
            for line in boards[i].iter_mut() {
                for cell in line.iter_mut() {
                    if let Bingo::Free(x) = cell {
                        if *x == nr {
                            *cell = Bingo::Marked(*x);
                        }
                    }
                }
            }
            if !winners[i] && check_win(&boards[i]) {
                ret = nr * free_sum(&boards[i]);
                winners[i] = true;
            }
        }
    }

    ret
}

fn solve1(mut boards: Vec<Vec<Vec<Bingo>>>, numbers: Vec<i32>) -> i32 {
    let mut ret = 0;
    'main_loop: for nr in numbers {
        for board in boards.iter_mut() {
            for line in board.iter_mut() {
                for cell in line.iter_mut() {
                    if let Bingo::Free(x) = cell {
                        if *x == nr {
                            *cell = Bingo::Marked(*x);
                        }
                    }
                }
            }
            if check_win(board) {
                ret = nr * free_sum(board);
                break 'main_loop;
            }
        }
    }

    ret
}

fn free_sum(board: &[Vec<Bingo>]) -> i32 {
    let mut sum = 0;

    for line in board {
        for cell in line {
            if let Bingo::Free(n) = cell {
                sum += n;
            }
        }
    }

    sum
}

fn check_win(board: &[Vec<Bingo>]) -> bool {
    let mut v_count = [0; 5];
    for i in 0..5 {
        let mut h_count = 0;
        for j in 0..5 {
            if matches!(board[i][j], Bingo::Marked(_)) {
                h_count += 1;
                v_count[j] += 1;
            }
        }
        if h_count == 5 {
            return true;
        }
        if v_count.contains(&5) {
            return true;
        }
    }

    false
}
