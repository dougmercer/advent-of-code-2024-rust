use advent_2024::Grid;
use image::{Rgb, RgbImage};
use itertools::Itertools;
use std::{error::Error, fs};

#[derive(Clone, Debug)]
struct Robot {
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn step(&mut self, width: usize, height: usize) {
        self.x = ((self.x as i32 + self.dx).rem_euclid(width as i32)) as usize;
        self.y = ((self.y as i32 + self.dy).rem_euclid(height as i32)) as usize;
    }
}

fn parse_line(line: &str) -> Option<(usize, usize, i32, i32)> {
    let nums: Vec<i32> = line
        .split(&[' ', '=', ','])
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    // println!("{nums:?}");
    match nums[..] {
        [x, y, dx, dy] => Some((x as usize, y as usize, dx, dy)),
        _ => None,
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let robots: Vec<Robot> = input
        .lines()
        .filter_map(parse_line)
        .map(|(x, y, dx, dy)| Robot {
            x: x as usize,
            y: y as usize,
            dx,
            dy,
        })
        .collect();
    robots
}

fn problem(input: &str, width: usize, height: usize, steps: usize) -> usize {
    let mut robots = parse_input(input);

    for _ in 0..steps {
        for robot in robots.iter_mut() {
            robot.step(width, height);
        }
    }

    compute_safety_factor(&robots, width, height)
}

#[allow(dead_code)]
fn robots_to_grid(robots: &Vec<Robot>, width: usize, height: usize) -> Grid<usize> {
    let mut grid: Grid<usize> = Grid::new(width, height, 0);
    robots
        .iter()
        .map(|robot| (robot.x, robot.y))
        .counts()
        .iter()
        .for_each(|(&position, &count)| {
            grid[position] = count;
        });
    grid
}

#[allow(dead_code)]
fn show_robots(robots: &Vec<Robot>, width: usize, height: usize) {
    println!("{}", robots_to_grid(&robots, width, height));
    println!();
}

fn compute_safety_factor(robots: &Vec<Robot>, width: usize, height: usize) -> usize {
    let mid_x = width / 2;
    let mid_y = height / 2;
    robots
        .iter()
        .filter_map(|robot| match (robot.x, robot.y) {
            (x, y) if x < mid_x && y < mid_y => Some(0),
            (x, y) if x > mid_x && y < mid_y => Some(1),
            (x, y) if x < mid_x && y > mid_y => Some(2),
            (x, y) if x > mid_x && y > mid_y => Some(3),
            _ => None,
        })
        .counts()
        .into_values()
        .map(|x| {
            // println!("{x}");
            x
        })
        .product()
}

#[allow(dead_code)]
fn problem2(input: &str, width: usize, height: usize, max_steps: usize, mut safety_factor: f64) {
    let mut robots = parse_input(input);
    let mut step = 0;
    loop {
        for robot in robots.iter_mut() {
            robot.step(width, height);
        }
        step += 1;
        let new_safety_factor = compute_safety_factor(&robots, width, height) as f64;
        if new_safety_factor <= 1.01 * safety_factor {
            safety_factor = new_safety_factor;
            println!("Step: {step}");
            println!("Safety Factor {safety_factor}");
            show_robots(&robots, width, height);
        }
        if step > max_steps {
            break;
        }
    }
}

#[allow(dead_code)]
fn render_easter_egg(
    input: &str,
    width: usize,
    height: usize,
    steps: usize,
) -> Result<(), Box<dyn Error>> {
    let mut img = RgbImage::new(width as u32, height as u32);
    let mut robots = parse_input(input);

    for _ in 0..steps {
        for robot in robots.iter_mut() {
            robot.step(width, height);
        }
    }

    let grid = robots_to_grid(&robots, width, height);

    for ((_, _, pixel), value) in img.enumerate_pixels_mut().zip(grid.iter()) {
        let r = (*value > 0) as u8;
        let g = (*value > 0) as u8;
        let b = (*value > 0) as u8;

        *pixel = Rgb([r * 255, g * 255, b * 255]);
    }

    img.save("output.png")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day14.input";
    let input = fs::read_to_string(path)?;
    let width = 101;
    let height = 103;
    let steps = 100;
    println!("Part 1: {:?}", problem(&input, width, height, steps));

    // // Run this and pipe to a text file, then scroll until you find the easter egg =]
    // problem2(&input, width, height, 10000, 136908590.0);

    // let easter_egg_step = put answer here;
    // render_easter_egg(&input, width, height, easter_egg_step)?;
    Ok(())
}

#[test]
fn test_part1() {
    let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
    assert_eq!(problem(&input, 11, 7, 100), 12);
}
