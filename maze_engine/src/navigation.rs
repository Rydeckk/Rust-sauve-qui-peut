use commun::structs::RelativeDirection;
use std::collections::{HashMap, HashSet, VecDeque};

const MEMORY_SIZE: usize = 100; // Taille de la mémoire du labyrinthe
const MAX_FAILS: usize = 3; // Nombre max d'échecs avant de bannir une direction temporairement

pub struct Navigator {
    last_direction: Option<RelativeDirection>,
    pub movement_history: VecDeque<RelativeDirection>, // Historique des derniers déplacements
    visited_positions: HashSet<(i32, i32)>, // Pour une recherche rapide des positions déjà visitées
    visited_paths: HashMap<(i32, i32), Vec<RelativeDirection>>, // Carte mémoire du labyrinthe
    current_position: (i32, i32), // Position relative dans le labyrinthe
    fail_count: HashMap<RelativeDirection, usize>, // Nombre d'échecs par direction
    banned_directions: HashSet<RelativeDirection>, // Directions bannies temporairement
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            last_direction: None,
            movement_history: VecDeque::new(),
            visited_positions: HashSet::new(),
            visited_paths: HashMap::new(),
            current_position: (0, 0), // Initialisation par défaut
            fail_count: HashMap::new(),
            banned_directions: HashSet::new(),
        }
    }

    /// Choisit le prochain déplacement en fonction de la vue radar.
    /// Si aucun déplacement n'est possible (ou non visité), effectue un demi-tour.
    pub fn choose_next_move(&mut self, radar_view: &[[char; 7]; 7]) -> RelativeDirection {
        println!("[Navigator] Current position: {:?}", self.current_position);
        println!("[Navigator] Radar View:");
        for row in radar_view.iter() {
            println!("{}", row.iter().collect::<String>());
        }

        let mut possible_moves = vec![
            RelativeDirection::Front,
            RelativeDirection::Right,
            RelativeDirection::Left,
            RelativeDirection::Back,
        ];

        // Exclure les directions interdites ou avec trop d'échecs
        possible_moves.retain(|&dir| {
            let fails = self.fail_count.get(&dir).cloned().unwrap_or(0);
            fails < MAX_FAILS && !self.banned_directions.contains(&dir) && self.is_open(radar_view, dir)
        });

        // Trier pour essayer d'abord celles avec le moins d'échecs
        possible_moves.sort_by_key(|&dir| self.fail_count.get(&dir).cloned().unwrap_or(0));

        // Si aucune direction n'est possible, faire demi-tour
        if possible_moves.is_empty() {
            return self.handle_no_moves();
        }

        // Sélectionner le premier déplacement qui mène vers une position non visitée
        for &direction in &possible_moves {
            let new_pos = Self::calculate_new_position(self.current_position, direction);
            println!("[Navigator] Considering move {:?} to {:?}", direction, new_pos);
            if !self.visited_positions.contains(&new_pos) {
                self.execute_move(direction, new_pos);
                return direction;
            }
        }

        // Si toutes les positions ont déjà été visitées, faire demi-tour
        self.handle_no_moves()
    }

    /// Effectue un déplacement en mettant à jour l'état du Navigator.
    fn execute_move(&mut self, direction: RelativeDirection, new_pos: (i32, i32)) {
        println!("[Navigator] Executing move {:?} to {:?}", direction, new_pos);
        self.banned_directions.remove(&direction);
        self.fail_count.remove(&direction);
        self.movement_history.push_back(direction);
        self.current_position = new_pos;
        self.visited_positions.insert(new_pos);
    }

    /// Gère le cas où aucun déplacement valide n'est possible en faisant demi-tour.
    fn handle_no_moves(&mut self) -> RelativeDirection {
        if let Some(last_move) = self.movement_history.pop_back() {
            println!("[Navigator] No valid moves, turning back from {:?}", last_move);
            self.banned_directions.insert(last_move);
            *self.fail_count.entry(last_move).or_insert(0) += 1;
            return Self::turn_back(last_move);
        }
        let random_direction = rand::random();
        println!("[Navigator] No moves and no history, choosing random direction: {:?}", random_direction);
        random_direction
    }

    /// Appelée lorsque le déplacement échoue (par exemple, si le serveur renvoie "CannotPassThroughWall").
    /// Revert la mise à jour de la position en soustrayant le delta correspondant à la direction échouée.
    pub fn handle_move_failure(&mut self, direction: RelativeDirection) {
        println!("[Navigator] Move failed in direction {:?}, reverting move.", direction);
        // Calculer le delta associé à la direction
        let (dx, dy) = Self::delta(direction);
        // Revenir à la position précédente
        let previous_position = (self.current_position.0 - dx, self.current_position.1 - dy);
        println!("[Navigator] Reverting position from {:?} to {:?}", self.current_position, previous_position);
        self.current_position = previous_position;
        // Retirer le mouvement erroné de l'historique (si présent)
        if let Some(last_move) = self.movement_history.pop_back() {
            println!("[Navigator] Removing last move {:?} due to failure", last_move);
        }
        // Bannir cette direction et incrémenter son compteur d'échecs
        self.banned_directions.insert(direction);
        *self.fail_count.entry(direction).or_insert(0) += 1;
        // Supprimer la position incorrectement ajoutée, si elle a été marquée comme visitée
        let incorrect_position = (previous_position.0 + dx, previous_position.1 + dy);
        self.visited_positions.remove(&incorrect_position);
    }

    /// Retourne le delta (dx, dy) associé à une direction.
    fn delta(direction: RelativeDirection) -> (i32, i32) {
        match direction {
            RelativeDirection::Front => (0, -1),
            RelativeDirection::Right => (1, 0),
            RelativeDirection::Back  => (0, 1),
            RelativeDirection::Left  => (-1, 0),
        }
    }

    /// Affiche l'état interne du Navigator (positions visitées et historique des déplacements).
    pub fn display_memory_map(&self) {
        println!("Visited positions:");
        for (x, y) in &self.visited_positions {
            println!("({}, {})", x, y);
        }
        println!("Movement history:");
        for direction in &self.movement_history {
            println!("{:?}", direction);
        }
    }

    /// Retourne true si la case dans la direction donnée est ouverte (i.e. contient un espace " ").
    fn is_open(&self, radar_view: &[[char; 7]; 7], direction: RelativeDirection) -> bool {
        match direction {
            RelativeDirection::Front => radar_view[1][3] == ' ',
            RelativeDirection::Right => radar_view[3][5] == ' ',
            RelativeDirection::Back  => radar_view[5][3] == ' ',
            RelativeDirection::Left  => radar_view[3][1] == ' ',
        }
    }

    /// Calcule la nouvelle position en fonction d'une position de départ et d'une direction.
    fn calculate_new_position((x, y): (i32, i32), direction: RelativeDirection) -> (i32, i32) {
        let (dx, dy) = Self::delta(direction);
        (x + dx, y + dy)
    }

    /// Renvoie la direction opposée.
    fn turn_back(direction: RelativeDirection) -> RelativeDirection {
        match direction {
            RelativeDirection::Front => RelativeDirection::Back,
            RelativeDirection::Back  => RelativeDirection::Front,
            RelativeDirection::Left  => RelativeDirection::Right,
            RelativeDirection::Right => RelativeDirection::Left,
        }
    }
}
