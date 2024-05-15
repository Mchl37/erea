use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::{thread, time};

// Structure représentant la carte
struct Map {
    width: usize,
    height: usize,
    obstacles: Vec<Vec<bool>>, // true si obstacle, false sinon
    energy: Vec<(usize, usize)>, // positions des sources d'énergie
    minerals: Vec<(usize, usize)>, // positions des gisements de minerais
    base: (usize, usize), // position de la base
    water: Vec<(usize, usize)>, // positions des zones d'eau
    explored: Vec<Vec<bool>>,
}

// Structure représentant un robot
struct Robot {
    x: usize,
    y: usize,
    energy: usize,
    minerals: usize,
    task: Task,
}

// Enumération des tâches des robots
#[derive(PartialEq)] // Ajout du dérive pour PartialEq
enum Task {
    CollectEnergy,
    CollectMinerals,
    Explore,
}

impl Robot {
    // Fonction pour créer un nouveau robot avec des valeurs initiales
    fn new(x: usize, y: usize, task: Task) -> Self {
        Robot {
            x,
            y,
            energy: 0,
            minerals: 0,
            task,
        }
    }

    fn move_towards(&mut self, target: (usize, usize), map: &mut Map) {
        // Mettre à jour la carte explorée
        let current_pos = (self.x, self.y);
    
        // Calculer les déplacements nécessaires pour se rapprocher de la cible
        let dx = if self.x < target.0 { 1 } else if self.x > target.0 { -1 } else { 0 };
        let dy = if self.y < target.1 { 1 } else if self.y > target.1 { -1 } else { 0 };
    
        // Calculer la nouvelle position potentielle
        let new_x = (self.x as isize).wrapping_add(dx as isize) as usize;
        let new_y = (self.y as isize).wrapping_add(dy as isize) as usize;
    
        // Vérifier si la nouvelle position est valide et si elle n'est pas un obstacle
        if new_x < map.width && new_y < map.height && !map.obstacles[new_y][new_x] {
            // Déplacer le robot directement
            self.x = new_x;
            self.y = new_y;
        }
    
        // Mettre à jour les cases environnantes comme explorées
        for dy in -1..=1 {
            for dx in -1..=1 {
                let x = (self.x as isize).wrapping_add(dx);
                let y = (self.y as isize).wrapping_add(dy);
                if x >= 0 && x < map.width as isize && y >= 0 && y < map.height as isize {
                    map.explored[y as usize][x as usize] = true;
                }
            }
        }
    }
}

fn draw_map(window: &mut Window, map: &Map, robots: &[Robot]) {
    let mut buffer: Vec<u32> = vec![0; map.width * map.height];
    for y in 0..map.height {
        for x in 0..map.width {
            let index = y * map.width + x;
            if map.explored[y][x] {
                if map.obstacles[y][x] {
                    buffer[index] = 0xFF_000000; // noir pour les obstacles
                } else if map.water.contains(&(x, y)) {
                    buffer[index] = 0xFF_0000FF; // bleu pour les zones d'eau
                } else if map.energy.contains(&(x, y)) || map.minerals.contains(&(x, y)) {
                    if !map.explored[y][x] {
                        buffer[index] = 0xFF_AAAAAA; // Gris pour les ressources cachées sous le brouillard
                    } else {
                        buffer[index] = if map.energy.contains(&(x, y)) {
                            0xFF_00FF00 // vert pour l'énergie
                        } else {
                            0xFFFF0000 // rouge pour les minerais
                        };
                    }
                } else if (x == map.base.0 && y >= map.base.1.saturating_sub(1) && y <= map.base.1.saturating_add(1))
                    || (y == map.base.1 && x >= map.base.0.saturating_sub(1) && x <= map.base.0.saturating_add(1))
                {
                    // Dessiner une croix pour la base
                    buffer[index] = 0xFF_00FFFF; // cyan pour la base
                } else {
                    buffer[index] = 0xFFFFFFFF; // blanc pour le sol exploré
                }
            } else {
                buffer[index] = 0xFF_AAAAAA; // Gris pour les cases non explorées
            }
        }
    }

    // Dessiner les robots
    for robot in robots {
        let index = robot.y * map.width + robot.x;
        buffer[index] = match robot.task {
            Task::CollectEnergy => 0xFF_FF00FF, // violet pour les robots qui collectent de l'énergie
            Task::CollectMinerals => 0xFFFF00FF, // rose pour les robots qui collectent des minerais
            Task::Explore => 0xFF_FFFF00, // Jaune pour le robot d'exploration
        };
    }

    // Afficher le contenu du buffer
    window.update_with_buffer(&buffer, map.width, map.height).unwrap();
}

fn explore_map(robot: &mut Robot, map: &mut Map) {
    // Recherche de la case non explorée la plus proche, y compris celles recouvertes de brouillard
    let mut target = (robot.x, robot.y);
    let mut min_distance = isize::MAX;

    // Recherche parmi toutes les cases de la carte
    for y in 0..map.height {
        for x in 0..map.width {
            if !map.explored[y][x] {
                let distance = ((robot.x as isize - x as isize).abs() + (robot.y as isize - y as isize).abs()) as isize;
                if distance < min_distance {
                    min_distance = distance;
                    target = (x, y);
                }
            }
        }
    }

    // Déplacement vers la case non explorée la plus proche
    robot.move_towards(target, map);
}

fn main() {
    let mut rng = rand::thread_rng();

    let width = 35;
    let height = 35;

    // Initialisation de la carte avec des obstacles, de l'énergie, des minerais, une base et des zones d'eau
    let mut map = Map {
        width,
        height,
        obstacles: vec![vec![false; width]; height],
        energy: vec![],
        minerals: vec![],
        base: (width / 2, height / 2), // base au centre de la carte
        water: vec![],
        explored: vec![vec![false; width]; height],
    };

    // Placement aléatoire des obstacles sur les bords de la carte
    for i in 0..width {
        map.obstacles[0][i] = true;
        map.obstacles[height - 1][i] = true;
    }
    for i in 0..height {
        map.obstacles[i][0] = true;
        map.obstacles[i][width - 1] = true;
    }

    // Placement aléatoire de l'énergie (hors des zones d'eau et non sur la base)
    for _ in 0..10 {
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(1..width - 1);
            y = rng.gen_range(1..height - 1);
            if !map.water.contains(&(x, y)) && (x, y) != map.base {
                break;
            }
        }
        map.energy.push((x, y));
    }

    // Placement aléatoire des minerais (hors des zones d'eau et non sur la base)
    for _ in 0..10 {
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(1..width - 1);
            y = rng.gen_range(1..height - 1);
            if !map.water.contains(&(x, y)) && (x, y) != map.base {
                break;
            }
        }
        map.minerals.push((x, y));
    }

    // Placement aléatoire des zones d'eau
    for _ in 0..5 {
        let x = rng.gen_range(1..width - 1);
        let y = rng.gen_range(1..height - 1);
        map.water.push((x, y));
    }

    // Création des robots
    let mut robots = Vec::new();
    // Garder uniquement le robot explorateur
    let x = map.base.0;
    let y = map.base.1;
    robots.push(Robot::new(x, y, Task::Explore));

    // Création de la fenêtre de visualisation
    let mut window = Window::new(
        "Rust Game",
        width * 20,
        height * 20,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Boucle principale de simulation
    while window.is_open() {
        // Déplacements des robots
        for robot in &mut robots {
            // Si le robot n'est pas sur un obstacle
            if !map.obstacles[robot.y][robot.x] {
                match robot.task {
                    Task::Explore => {
                        // Exploration de la carte
                        explore_map(robot, &mut map);
                    }
                    _ => {} // Ignorer les autres tâches
                }
            }
        }
        // Dessiner la carte avec les robots
        draw_map(&mut window, &map, &robots);
        // Pause pour contrôler la vitesse de rafraîchissement
        thread::sleep(time::Duration::from_millis(100));
    }
}
