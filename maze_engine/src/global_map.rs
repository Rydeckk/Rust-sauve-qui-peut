use crate::radar::RadarView;
use std::collections::HashSet;

/// Largeur et hauteur maximales de la carte.
pub const MAP_WIDTH: usize = 20;
pub const MAP_HEIGHT: usize = 20;

/// Liste des caractères représentant un mur.
const WALLS: [char; 3] = ['•', '-', '|'];

/// **Carte globale du labyrinthe**, mise à jour en fonction des `RadarView`.
pub struct GlobalMap {
    grid: Vec<Vec<char>>,           // Carte stockant les murs et passages
    explored: HashSet<(usize, usize)>, // Liste des cases déjà visitées
    pub(crate) player_pos: (usize, usize), // Dernière position connue du joueur
}

impl GlobalMap {
    /// **Crée une carte vide remplie de `#` (zones inconnues).**
    pub fn new(start_x: usize, start_y: usize) -> Self {
        let mut grid = vec![vec!['#'; MAP_WIDTH]; MAP_HEIGHT];
        grid[start_y][start_x] = 'P'; // Position initiale du joueur

        Self {
            grid,
            explored: HashSet::new(),
            player_pos: (start_x, start_y),
        }
    }

    /// **Met à jour la carte en fonction d'une `RadarView`.**
    ///
    /// - Déduit la position actuelle du joueur.
    /// - Marque les murs et les passages explorés.
    /// - Ajoute les indices (`H`) et la cible (`G`).
    pub fn update_from_radar(&mut self, radar: &RadarView, direction: Option<(isize, isize)>) {
        // Déplacement estimé du joueur
        if let Some((dx, dy)) = direction {
            let new_x = (self.player_pos.0 as isize + dx) as usize;
            let new_y = (self.player_pos.1 as isize + dy) as usize;
            self.player_pos = (new_x, new_y);
        }

        // Positionner la vue radar autour du joueur
        let start_x = self.player_pos.0 as isize - 1;
        let start_y = self.player_pos.1 as isize - 1;

        for dy in 0..3 {
            for dx in 0..3 {
                let global_x = start_x + dx as isize;
                let global_y = start_y + dy as isize;

                if global_x < 0 || global_y < 0 || global_x as usize >= MAP_WIDTH || global_y as usize >= MAP_HEIGHT {
                    continue;
                }

                let gx = global_x as usize;
                let gy = global_y as usize;
                let cell = radar.get_cell(dx, dy);

                // Mise à jour des murs
                if radar.is_wall_horizontal(dy, dx) {
                    self.set_wall(gx, gy, '-');
                }
                if radar.is_wall_vertical(dy, dx) {
                    self.set_wall(gx, gy, '|');
                }

                // Mise à jour des zones explorées
                if cell & 0b1000 != 0 {
                    self.set_goal(gx, gy);
                } else if cell & 0b0100 != 0 {
                    self.set_hint(gx, gy);
                } else {
                    self.set_explored(gx, gy, ' ');
                }
            }
        }
    }

    /// **Vérifie si une case a déjà été explorée.**
    pub fn is_visited(&self, x: usize, y: usize) -> bool {
        self.explored.contains(&(x, y))
    }

    /// **Vérifie si une case est un mur.**
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        WALLS.contains(&self.grid[y][x])
    }

    /// **Ajoute un mur à la carte.**
    pub fn set_wall(&mut self, x: usize, y: usize, wall_type: char) {
        if x < MAP_WIDTH && y < MAP_HEIGHT {
            self.grid[y][x] = wall_type;
            self.explored.insert((x, y));
        }
    }

    /// **Marque une case comme explorée.**
    pub fn set_explored(&mut self, x: usize, y: usize, cell_content: char) {
        if x < MAP_WIDTH && y < MAP_HEIGHT {
            self.grid[y][x] = cell_content;
            self.explored.insert((x, y));
        }
    }

    /// **Ajoute un indice (`H`) sur la carte.**
    pub fn set_hint(&mut self, x: usize, y: usize) {
        self.grid[y][x] = 'H';
    }

    /// **Ajoute la cible (`G`) sur la carte.**
    pub fn set_goal(&mut self, x: usize, y: usize) {
        self.grid[y][x] = 'G';
    }

    /// **Affiche la carte globale.**
    pub fn print_map(&self) {
        println!("Carte globale :");
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
