use std::collections::HashSet;

/// Largeur maximale de la carte.
pub const MAP_WIDTH: usize = 20;
/// Hauteur maximale de la carte.
pub const MAP_HEIGHT: usize = 20;

/// Liste des caractères représentant un mur.
const WALLS: [char; 3] = ['•', '-', '|'];

/// **Carte globale du labyrinthe**.
///
/// Cette structure permet de représenter la carte globale en tenant compte des mises à jour reçues via des
/// `RadarView`. Elle stocke une grille de caractères ainsi qu'un ensemble des positions déjà explorées.
pub struct GlobalMap {
    grid: Vec<Vec<char>>,             // Carte stockant les murs et passages.
    explored: HashSet<(usize, usize)>, // Ensemble des cases déjà explorées.
}

impl GlobalMap {
    /// Crée une carte vide remplie de `#` (zones inconnues) et place le joueur à la position de départ.
    ///
    /// # Arguments
    ///
    /// * `start_x` - Position horizontale de départ.
    /// * `start_y` - Position verticale de départ.
    ///
    /// # Exemples
    ///
    /// ```
    /// use maze_engine::global_map::GlobalMap;
    ///
    /// let map = GlobalMap::new(3, 4);
    /// // La position initiale du joueur est (3, 4).
    /// ```
    pub fn new(start_x: usize, start_y: usize) -> Self {
        let mut grid = vec![vec!['#'; MAP_WIDTH]; MAP_HEIGHT];
        grid[start_y][start_x] = 'P'; // Position initiale du joueur

        Self {
            grid,
            explored: HashSet::new(),
        }
    }

    /// Vérifie si une case a déjà été explorée.
    ///
    /// # Arguments
    ///
    /// * `x` - Position horizontale.
    /// * `y` - Position verticale.
    ///
    /// # Retourne
    ///
    /// `true` si la case (x, y) est dans l'ensemble des cases explorées, sinon `false`.
    pub fn is_visited(&self, x: usize, y: usize) -> bool {
        self.explored.contains(&(x, y))
    }

    /// Vérifie si une case contient un mur.
    ///
    /// # Arguments
    ///
    /// * `x` - Position horizontale.
    /// * `y` - Position verticale.
    ///
    /// # Retourne
    ///
    /// `true` si le caractère de la case correspond à l'un des caractères de mur définis dans `WALLS`, sinon `false`.
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        WALLS.contains(&self.grid[y][x])
    }

    /// Ajoute un mur à la carte et marque la case comme explorée.
    ///
    /// # Arguments
    ///
    /// * `x` - Position horizontale.
    /// * `y` - Position verticale.
    /// * `wall_type` - Caractère représentant le type de mur (ex. '-' ou '|').
    pub fn set_wall(&mut self, x: usize, y: usize, wall_type: char) {
        if x < MAP_WIDTH && y < MAP_HEIGHT {
            self.grid[y][x] = wall_type;
            self.explored.insert((x, y));
        }
    }

    /// Marque une case comme explorée et y assigne un contenu.
    ///
    /// # Arguments
    ///
    /// * `x` - Position horizontale.
    /// * `y` - Position verticale.
    /// * `cell_content` - Caractère indiquant le contenu de la case (ex. ' ' pour vide).
    pub fn set_explored(&mut self, x: usize, y: usize, cell_content: char) {
        if x < MAP_WIDTH && y < MAP_HEIGHT {
            self.grid[y][x] = cell_content;
            self.explored.insert((x, y));
        }
    }

    /// Affiche la carte globale dans la console.
    ///
    /// Chaque ligne de la carte est affichée sous forme de chaîne de caractères.
    pub fn print_map(&self) {
        println!("Carte globale :");
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_wall_and_is_wall() {
        let mut map = GlobalMap::new(0, 0);
        map.set_wall(2, 3, '-');
        assert!(map.is_wall(2, 3));
        assert!(map.is_visited(2, 3));
    }

    #[test]
    fn test_set_explored_and_is_visited() {
        let mut map = GlobalMap::new(0, 0);
        map.set_explored(5, 5, ' ');
        assert!(map.is_visited(5, 5));
        // La case ne doit pas être considérée comme mur
        assert!(!map.is_wall(5, 5));
    }

    #[test]
    fn test_print_map() {
        let mut map = GlobalMap::new(0, 0);
        map.set_explored(1, 1, ' ');
        map.set_wall(2, 2, '-');
        map.print_map();
    }
}
