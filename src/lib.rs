/*
// src/lib.rs
use minifb::{Window, WindowOptions};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

// Structure représentant la carte
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub obstacles: Vec<Vec<bool>>,     // true si obstacle, false sinon
    pub energy: Vec<(usize, usize)>,   // positions des sources d'énergie
    pub minerals: Vec<(usize, usize)>, // positions des gisements de minerais
    pub base: (usize, usize),          // position de la base
    pub explored: Vec<Vec<bool>>,
}

// Structure représentant un robot
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub energy: usize,
    pub minerals: usize,
    pub task: Task,
    pub state: RobotState,
}

// Enumération des tâches des robots
#[derive(PartialEq)] // Ajout du dérive pour PartialEq
pub enum Task {
    CollectEnergy,
    CollectMinerals,
    Explore,
}

pub enum RobotState {
    Exploring,
    Returning,
    Collecting,
}

impl Robot {
    // Fonction pour créer un nouveau robot avec des valeurs initiales
    pub fn new(x: usize, y: usize, task: Task) -> Self {
        Robot {
            x,
            y,
            energy: 0,
            minerals: 0,
            task,
            state: RobotState::Exploring,
        }
    }

    pub fn move_towards(&mut self, target: (usize, usize)) {
        let dx = if self.x < target.0 {
            1
        } else if self.x > target.0 {
            -1
        } else {
            0
        };
        let dy = if self.y < target.1 {
            1
        } else if self.y > target.1 {
            -1
        } else {
            0
        };
        let new_x = (self.x as isize).wrapping_add(dx as isize) as usize;
        let new_y = (self.y as isize).wrapping_add(dy as isize) as usize;
        self.x = new_x;
        self.y = new_y;
    }
}

pub fn draw_map(window: &mut Window, map: &Map, robots: &[Robot]) {
    let mut buffer: Vec<u32> = vec![0; map.width * map.height];
    for y in 0..map.height {
        for x in 0..map.width {
            let index = y * map.width + x;
            if (x == map.base.0
                && y >= map.base.1.saturating_sub(1)
                && y <= map.base.1.saturating_add(1))
                || (y == map.base.1
                    && x >= map.base.0.saturating_sub(1)
                    && x <= map.base.0.saturating_add(1))
            {
                buffer[index] = 0xFF_00FFFF;
            } else if map.explored[y][x] {
                if map.obstacles[y][x] {
                    buffer[index] = 0xFF_000000;
                } else if map.energy.contains(&(x, y)) || map.minerals.contains(&(x, y)) {
                    if !map.explored[y][x] {
                        buffer[index] = 0xFF_AAAAAA;
                    } else {
                        buffer[index] = if map.energy.contains(&(x, y)) {
                            0xFF_00FF00
                        } else {
                            0xFFFF0000
                        };
                    }
                } else {
                    buffer[index] = 0xFF_FFFFFF;
                }
            } else {
                buffer[index] = 0xFF_AAAAAA;
            }
        }
    }

    for robot in robots {
        let index = robot.y * map.width + robot.x;
        let color = match robot.state {
            RobotState::Exploring => 0xFF_FFFF00, // Jaune pour l'exploration
            RobotState::Collecting => 0xFFFF00FF, // Rose pour la collecte
            RobotState::Returning => 0xFFFF00FF,  // Rose pour le retour à la base
        };
        buffer[index] = color;
    }

    window
        .update_with_buffer(&buffer, map.width, map.height)
        .unwrap();
}

pub fn explore_map(robot: &mut Robot, map: &mut Map) {
    let mut target = (robot.x, robot.y);
    let mut min_distance = isize::MAX;

    for y in 0..map.height {
        for x in 0..map.width {
            if !map.explored[y][x] {
                let distance = ((robot.x as isize - x as isize).abs()
                    + (robot.y as isize - y as isize).abs())
                    as isize;
                if distance < min_distance {
                    min_distance = distance;
                    target = (x, y);
                }
            }
        }
    }

    for dy in -1..=1 {
        for dx in -1..=1 {
            let x = (robot.x as isize + dx) as usize;
            let y = (robot.y as isize + dy) as usize;
            if x < map.width && y < map.height {
                map.explored[y][x] = true;
            }
        }
    }

    robot.move_towards(target);
}
pub fn collect_resources(robot: &mut Robot, map: &mut Map) {
    match robot.task {
        Task::CollectEnergy => {
            if let Some(&(x, y)) = map
                .energy
                .iter()
                .find(|&&(ex, ey)| ex == robot.x && ey == robot.y)
            {
                robot.energy += 1;
                map.energy.retain(|&pos| pos != (x, y));
                robot.state = RobotState::Returning;
            } else {
                if let Some(target) = map
                    .energy
                    .iter()
                    .min_by_key(|&&(x, y)| {
                        ((robot.x as isize - x as isize).abs() + (robot.y as isize - y as isize).abs())
                            as isize
                    })
                    .cloned()
                {
                    robot.move_towards(target);
                } else {
                    robot.state = RobotState::Returning;
                }
            }
        }
        Task::CollectMinerals => {
            if let Some(&(x, y)) = map
                .minerals
                .iter()
                .find(|&&(mx, my)| mx == robot.x && my == robot.y)
            {
                robot.minerals += 1;
                map.minerals.retain(|&pos| pos != (x, y));
                robot.state = RobotState::Returning;
            } else {
                if let Some(target) = map
                    .minerals
                    .iter()
                    .min_by_key(|&&(x, y)| {
                        ((robot.x as isize - x as isize).abs() + (robot.y as isize - y as isize).abs())
                            as isize
                    })
                    .cloned()
                {
                    robot.move_towards(target);
                } else {
                    robot.state = RobotState::Returning;
                }
            }
        }
        _ => {}
    }
}

pub fn is_map_fully_explored(map: &Map) -> bool {
    for row in &map.explored {
        if row.contains(&false) {
            return false;
        }
    }
    true
}

pub fn generate_map(width: usize, height: usize) -> Map {
    let mut map = Map {
        width,
        height,
        obstacles: vec![vec![false; width]; height],
        energy: vec![],
        minerals: vec![],
        base: (width / 2, height / 2),
        explored: vec![vec![false; width]; height],
    };

    let mut rng = rand::thread_rng();
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let perlin = Perlin::new(seed as u32);

    for y in 0..height {
        for x in 0..width {
            let value = perlin.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
            if value > 0.5 {
                map.obstacles[y][x] = true;
            }
        }
    }

    for _ in 0..10 {
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(0..width);
            y = rng.gen_range(0..height);
            if (x, y) != map.base {
                break;
            }
        }
        map.energy.push((x, y));
    }

    for _ in 0..10 {
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(0..width);
            y = rng.gen_range(0..height);
            if (x, y) != map.base {
                break;
            }
        }
        map.minerals.push((x, y));
    }

    map
}
*/
