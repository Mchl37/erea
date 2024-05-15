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
enum Task {
    CollectEnergy,
    CollectMinerals,
}

impl Robot {
    fn new(x: usize, y: usize, task: Task) -> Self {
        Robot {
            x,
            y,
            energy: 0,
            minerals: 0,
            task,
        }
    }

    fn move_towards(&mut self, target: (usize, usize), map: &Map) {
        if map.water.contains(&target) {
            return; // Ne pas se déplacer si la case cible est de l'eau
        }
        
        if self.x < target.0 {
            self.x += 1;
        } else if self.x > target.0 {
            self.x -= 1;
        }
        if self.y < target.1 {
            self.y += 1;
        } else if self.y > target.1 {
            self.y -= 1;
        }
    }

    // Autres méthodes de la struct Robot
}

fn draw_map(window: &mut Window, map: &Map, robots: &[Robot]) {
    let mut buffer: Vec<u32> = vec![0; map.width * map.height];
    // Dessiner les obstacles
    for y in 0..map.height {
        for x in 0..map.width {
            let index = y * map.width + x;
            buffer[index] = if map.obstacles[y][x] {
                0xFF_000000 // noir pour les obstacles
            } else if map.water.contains(&(x, y)) {
                0xFF_0000FF // bleu pour les zones d'eau
            } else {
                0xFFFFFFFF // blanc pour le sol
            };
        }
    }
    // Dessiner les ressources
    for &(x, y) in &map.energy {
        let index = y * map.width + x;
        buffer[index] = 0xFF_00FF00; // vert pour l'énergie
    }
    for &(x, y) in &map.minerals {
        let index = y * map.width + x;
        buffer[index] = 0xFFFF0000; // rouge pour les minerais
    }
    // Dessiner la base
    let (bx, by) = map.base;
    let index = by * map.width + bx;
    buffer[index] = 0xFF_00FFFF; // cyan pour la base
    // Dessiner les robots
    for robot in robots {
        let index = robot.y * map.width + robot.x;
        buffer[index] = match robot.task {
            Task::CollectEnergy => 0xFF_FF00FF, // violet pour les robots qui collectent de l'énergie
            Task::CollectMinerals => 0xFFFF00FF, // rose pour les robots qui collectent des minerais
        };
    }
    // Afficher le contenu du buffer
    window.update_with_buffer(&buffer, map.width, map.height).unwrap();
}

fn main() {
    let mut rng = rand::thread_rng();

    let width = 20;
    let height = 20;

    // Initialisation de la carte avec des obstacles, de l'énergie, des minerais, une base et des zones d'eau
    let mut map = Map {
        width,
        height,
        obstacles: vec![vec![false; width]; height],
        energy: vec![],
        minerals: vec![],
        base: (width / 2, height / 2), // base au centre de la carte
        water: vec![],
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

    // Placement aléatoire de l'énergie (hors des zones d'eau)
    for _ in 0..10 {
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(1..width - 1);
            y = rng.gen_range(1..height - 1);
            if !map.water.contains(&(x, y)) {
                break;
            }
        }
        map.energy.push((x, y));
    }

    // Placement aléatoire des minerais (hors des zones d'eau)
    for _ in 0..10 {
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(1..width - 1);
            y = rng.gen_range(1..height - 1);
            if !map.water.contains(&(x, y)) {
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
    for _ in 0..3 {
        let x = map.base.0;
        let y = map.base.1;
        robots.push(Robot::new(x, y, Task::CollectEnergy));
    }
    for _ in 0..3 {
        let x = map.base.0;
        let y = map.base.1;
        robots.push(Robot::new(x, y, Task::CollectMinerals));
    }

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
          // Si le robot est dans une zone d'eau
          if map.water.contains(&(robot.x, robot.y)) {
              // Trouver un chemin contournant l'eau
              let mut new_position = (robot.x, robot.y);
              
              // Vérifier les cases adjacentes pour trouver une case non dans l'eau
              let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // déplacements horizontaux et verticaux
              for &(dx, dy) in &offsets {
                  let (new_x, new_y) = (robot.x as isize + dx, robot.y as isize + dy);
                  if (0..map.width as isize).contains(&new_x) && (0..map.height as isize).contains(&new_y) {
                      let (x, y) = (new_x as usize, new_y as usize);
                      if !map.water.contains(&(x, y)) && !map.obstacles[y][x] {
                          new_position = (x, y);
                          break;
                      }
                  }
              }
              
              // Mettre à jour les coordonnées du robot
              robot.x = new_position.0;
              robot.y = new_position.1;
              
              // Continuer avec le prochain robot
              continue;
          }
          
          // Si le robot n'est pas dans une zone d'eau, ni sur un obstacle
          if !map.obstacles[robot.y][robot.x] && !map.water.contains(&(robot.x, robot.y)) {
              match robot.task {
                  Task::CollectEnergy => {
                      if let Some((x, y)) = map.energy.iter().enumerate().find(|&(_, &(ex, ey))| ex == robot.x && ey == robot.y) {
                          robot.energy += 1;
                          map.energy.remove(x);
                      } else {
                          // Déplacement vers la source d'énergie la plus proche
                          if let Some(&(ex, ey)) = map.energy.iter().min_by_key(|&&(ex, ey)| {
                              ((robot.x as isize - ex as isize).abs() + (robot.y as isize - ey as isize).abs()) as usize
                          }) {
                              robot.move_towards((ex, ey), &map);
                          }
                      }
                  }
                  Task::CollectMinerals => {
                      if let Some((x, y)) = map.minerals.iter().enumerate().find(|&(_, &(mx, my))| mx == robot.x && my == robot.y) {
                          robot.minerals += 1;
                          map.minerals.remove(x);
                      } else {
                          // Déplacement vers le gisement de minerais le plus proche
                          if let Some(&(mx, my)) = map.minerals.iter().min_by_key(|&&(mx, my)| {
                              ((robot.x as isize - mx as isize).abs() + (robot.y as isize - my as isize).abs()) as usize
                          }) {
                              robot.move_towards((mx, my), &map);
                          }
                      }
                  }
              }
          }
        }
        // Dessiner la carte avec les robots
        draw_map(&mut window, &map, &robots);
        // Pause pour contrôler la vitesse de rafraîchissement
        thread::sleep(time::Duration::from_millis(500));
    }
}
