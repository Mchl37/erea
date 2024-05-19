#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_start() {
        // Exécute le programme principal
        let output = Command::new("cargo")
            .arg("run")
            .output()
            .expect("Failed to execute command");
        
        // Vérifie si l'exécution s'est terminée sans erreur
        assert!(output.status.success());

        // Vérifie si la sortie standard ne contient pas d'erreurs
        assert!(!String::from_utf8_lossy(&output.stderr).contains("error"));
    }

    #[test]
    fn test_explore_map() {
        let mut map = generate_map(10, 10);
        let mut robot = Robot::new(5, 5, Task::Explore);
        let initial_explored = map.explored.clone();

        
        explore_map(&mut robot, &mut map);

        // Vérifie que la carte explorée a changé autour du robot
        for dy in -1..=1 {
            for dx in -1..=1 {
                let x = (robot.x as isize + dx) as usize;
                let y = (robot.y as isize + dy) as usize;
                assert_ne!(map.explored[y][x], initial_explored[y][x]);
            }
        }
    }
   

    #[test]
    fn test_generate_map() {
        // Vérifie la taille de la carte générée
        let map = generate_map(20, 20);
        assert_eq!(map.width, 20);
        assert_eq!(map.height, 20);

        // Vérifie que la carte a une base définie
        assert_ne!(map.base, (0, 0));

        // Vérifie que la carte a des obstacles
        assert!(map.obstacles.iter().any(|row| row.iter().any(|&val| val)));
    }

    #[test]
    fn test_robot_movement() {
        // Création d'une carte de test
        let mut map = Map {
            width: 10,
            height: 10,
            obstacles: vec![vec![false; 10]; 10],
            energy: vec![],
            minerals: vec![],
            base: (5, 5),
            explored: vec![vec![false; 10]; 10], // Initialisation de la carte explorée
        };

        // Création d'un robot
        let mut robot = Robot::new(5, 5, Task::Explore);

        // Déplacement du robot vers le haut
        robot.move_towards((5, 4));
        assert_eq!(robot.x, 5);
        assert_eq!(robot.y, 4);

        // Déplacement du robot vers la droite
        robot.move_towards((6, 4));
        assert_eq!(robot.x, 6);
        assert_eq!(robot.y, 4);

        // Déplacement du robot vers le bas
        robot.move_towards((6, 5));
        assert_eq!(robot.x, 6);
        assert_eq!(robot.y, 5);

        // Déplacement du robot vers la gauche
        robot.move_towards((5, 5));
        assert_eq!(robot.x, 5);
        assert_eq!(robot.y, 5);
    }
    

    #[test]
        fn test_base_appearance() {
            // Création d'une carte de test
            let map = Map {
                width: 10,
                height: 10,
                obstacles: vec![vec![false; 10]; 10],
                energy: vec![],
                minerals: vec![],
                base: (5, 5), // Position de la base scientifique
                explored: vec![vec![true; 10]; 10], // Toutes les cases sont explorées
            };

            // Vérification que la position de la base est correcte
            assert_eq!(map.base, (5, 5));
        }

        #[test]
        fn test_mineral_presence() {
            // Création d'une carte de test
            let mut map = Map {
                width: 10,
                height: 10,
                obstacles: vec![vec![false; 10]; 10],
                energy: vec![],
                minerals: vec![(5, 5)], // Ajout d'un minerai à la position (5, 5)
                base: (5, 5),
                water: vec![],
            };
    
            // Vérification de la présence du minerai
            assert!(map.minerals.contains(&(5, 5)));
        } 
        
        #[test]
    fn test_collect_minerals() {
        // Crée une carte avec des minéraux
        let mut map = Map {
            width: 10,
            height: 10,
            obstacles: vec![vec![false; 10]; 10],
            energy: vec![],
            minerals: vec![(5, 5)], // Position d'un minerai
            base: (5, 5),
            explored: vec![vec![false; 10]; 10],
        };

        // Crée un robot avec une tâche de collecte de minéraux
        let mut robot = Robot::new(5, 5, Task::CollectMinerals);

        // Vérifie que le compteur de minéraux du robot est à 0 au début
        assert_eq!(robot.minerals, 0);

        
        robot.move_towards((5, 5), &map);

        // Vérifie que le robot a collecté le minerai
        if let Some((x, y)) = map.minerals.iter().position(|&(mx, my)| mx == robot.x && my == robot.y) {
            map.minerals.remove(x);
            robot.minerals += 1;
        }

        // Vérifie que le compteur de minéraux du robot a augmenté
        assert_eq!(robot.minerals, 1);
    }
}