use serde::{Serialize, Deserialize};

/// Représente un labyrinthe avec une grille de cellules.
///
/// La grille est stockée sous forme de tableau 2D de caractères :
/// - `.` représente un passage libre.
/// - `#` représente un mur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maze {
    /// Largeur du labyrinthe en nombre de cellules.
    pub width: u32,
    /// Hauteur du labyrinthe en nombre de cellules.
    pub height: u32,
    /// Grille du labyrinthe stockant chaque cellule (`.` pour libre, `#` pour mur).
    grid: Vec<Vec<char>>,
}

impl Maze {
    /// Crée un nouveau labyrinthe vide sans murs (`.` partout).
    ///
    /// # Arguments
    /// - `width` : Largeur du labyrinthe en cellules.
    /// - `height` : Hauteur du labyrinthe en cellules.
    ///
    /// # Retourne
    /// Un labyrinthe initialisé avec uniquement des passages libres.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            grid: vec![vec!['.'; width as usize]; height as usize],
        }
    }

    /// Vérifie si une cellule contient un mur.
    ///
    /// # Arguments
    /// - `x` : Coordonnée horizontale de la cellule.
    /// - `y` : Coordonnée verticale de la cellule.
    ///
    /// # Retourne
    /// - `true` si la cellule est un mur (`#`) ou hors limites.
    /// - `false` si la cellule est un passage libre (`.`).
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        if x >= self.width as usize || y >= self.height as usize {
            return true; // Considérer hors limites comme un mur
        }
        self.grid[y][x] == '#'
    }

    /// Place un mur dans le labyrinthe à une position donnée.
    ///
    /// # Arguments
    /// - `x` : Coordonnée horizontale de la cellule.
    /// - `y` : Coordonnée verticale de la cellule.
    ///
    /// # Retourne
    /// - `true` si le mur a été placé avec succès.
    /// - `false` si les coordonnées sont hors limites.
    pub fn set_wall(&mut self, x: usize, y: usize) -> bool {
        if x >= self.width as usize || y >= self.height as usize {
            return false; // Hors limites
        }
        self.grid[y][x] = '#';
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_creation() {
        let maze = Maze::new(5, 5);
        assert_eq!(maze.width, 5);
        assert_eq!(maze.height, 5);
        assert_eq!(maze.is_wall(2, 2), false); // Par défaut, tout est libre
    }

    #[test]
    fn test_is_wall_detection() {
        let mut maze = Maze::new(4, 4);
        maze.set_wall(1, 1);

        assert_eq!(maze.is_wall(1, 1), true);  // Un mur a été placé
        assert_eq!(maze.is_wall(0, 0), false); // Pas de mur ici
    }

    #[test]
    fn test_is_wall_out_of_bounds() {
        let maze = Maze::new(3, 3);
        assert_eq!(maze.is_wall(3, 3), true); // Hors limites = mur
        assert_eq!(maze.is_wall(2, 2), false); // Dans les limites = passage libre
    }

    #[test]
    fn test_set_wall() {
        let mut maze = Maze::new(4, 4);

        assert_eq!(maze.set_wall(2, 2), true);  // Placement réussi
        assert_eq!(maze.is_wall(2, 2), true);   // Vérification
        assert_eq!(maze.set_wall(5, 5), false); // Hors limites
    }
}
