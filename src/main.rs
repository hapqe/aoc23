use itertools::Itertools;
use std::{env::args, fs, ops::Index, str::Lines};

fn main() {
    println!(
        "{}",
        args()
            .skip(1)
            .zip([24, 2])
            .filter_map(|(arg, max)| arg.parse().ok().filter(|num| (0..max).contains(num)))
            .next_tuple()
            .map::<(_, [[&dyn Fn(Lines) -> i32; 1]; 4]), _>(|info| (
                info,
                [
                    [&|lines| {
                        lines
                            .map(|line| {
                                line.chars()
                                    .filter_map(|c| c.to_string().parse::<i32>().ok())
                            })
                            .map(|mut nums| {
                                nums.clone().last().unwrap() + nums.next().unwrap() * 10
                            })
                            .sum()
                    }],
                    [&|lines| {
                        lines
                            .map(|line| line.split(&[':', ';']))
                            .map(|mut parts| {
                                (
                                    parts.next().clone().unwrap()[5..].parse::<i32>().unwrap(),
                                    parts,
                                )
                            })
                            .map(|(game, mut parts)| {
                                parts
                                    .all(|part| {
                                        part.split(',').map(|cubes| cubes.split_whitespace()).all(
                                            |mut cubes| {
                                                cubes.next().unwrap().parse::<i32>().unwrap()
                                                    <= [12, 13, 14][(['r', 'g', 'b']
                                                        .iter()
                                                        .position(|color| {
                                                            cubes
                                                                .clone()
                                                                .last()
                                                                .unwrap()
                                                                .chars()
                                                                .next()
                                                                .unwrap()
                                                                == *color
                                                        }))
                                                    .unwrap()]
                                            },
                                        )
                                    })
                                    .then(|| game)
                                    .or(Some(0))
                            })
                            .flatten()
                            .sum()
                    }],
                    [&|_lines| { unimplemented!() }],
                    [&|lines| {
                        lines
                            .map(|line| {
                                line.split(&[':', '|']).skip(1).map(|game| {
                                    game.split_whitespace()
                                        .map(|nums| nums.parse::<usize>().unwrap())
                                        .collect::<Vec<_>>()
                                })
                            })
                            .map(|mut nums| (nums.next().unwrap(), nums.next().unwrap()))
                            .map(|(cards, sol)| {
                                cards.iter().filter(|card| sol.contains(card)).count()
                            })
                            .filter_map(|count| (count > 0).then(|| 2i32.pow((count - 1) as _)))
                            .sum()
                    }]
                ]
            ))
            .map(|((day, part), sol)| sol[day as usize][part as usize](
                fs::read_to_string(format!("input/{day}_{part}.txt"))
                    .unwrap()
                    .lines()
            )
            .to_string())
            .unwrap_or("Usage: ./main <day (0-23)> <part (0/1)> of aoc problem".into())
    );
}
