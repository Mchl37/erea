
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::io::ErrorKind;
    
    
    

    #[test]
    fn test_game_start() {
        // Exécuter le programme principal
        let output = Command::new("cargo")
            .arg("run")
            .output()
            .expect("Failed to execute command");
        
        // Vérifier si l'exécution s'est terminée sans erreur
        assert!(output.status.success());

        // Vérifier si la sortie standard ne contient pas d'erreurs
        assert!(!String::from_utf8_lossy(&output.stderr).contains("error"));
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
            water: vec![],
        };

        // Création d'un robot
        let mut robot = Robot::new(5, 5, Task::CollectEnergy);

        // Déplacement du robot vers le haut
        robot.move_towards((5, 4), &map);
        assert_eq!(robot.x, 5);
        assert_eq!(robot.y, 4);

        // Déplacement du robot vers la droite
        robot.move_towards((6, 4), &map);
        assert_eq!(robot.x, 6);
        assert_eq!(robot.y, 4);

        // Déplacement du robot vers le bas
        robot.move_towards((6, 5), &map);
        assert_eq!(robot.x, 6);
        assert_eq!(robot.y, 5);

        // Déplacement du robot vers la gauche
        robot.move_towards((5, 5), &map);
        assert_eq!(robot.x, 5);
        assert_eq!(robot.y, 5);
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
   

    
}

