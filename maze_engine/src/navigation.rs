use commun::structs::RelativeDirection;
use std::collections::{HashMap, HashSet, VecDeque};
use rand::distributions::Distribution;

const MAX_FAILS: usize = 3; // Nombre maximum d'échecs avant de bannir une direction temporairement

/// Gère la navigation dans le labyrinthe en mémorisant l'historique des déplacements, les positions visitées,
/// et en déterminant le prochain mouvement à effectuer en fonction de la vue radar.
///
/// La structure `Navigator` s'appuie sur plusieurs champs internes pour éviter de revenir sur ses pas
/// lorsque d'autres options sont disponibles, et pour gérer les échecs de déplacement.
pub struct Navigator {
    pub last_direction: Option<RelativeDirection>,
    /// Historique des déplacements effectués.
    pub movement_history: VecDeque<RelativeDirection>,
    /// Ensemble des positions déjà visitées (représentées par des coordonnées `(x, y)`).
    visited_positions: HashSet<(i32, i32)>,
    /// Position actuelle dans le labyrinthe.
    pub current_position: (i32, i32),
    /// Nombre d'échecs enregistrés pour chaque direction.
    fail_count: HashMap<RelativeDirection, usize>,
    /// Ensemble des directions temporairement bannies.
    banned_directions: HashSet<RelativeDirection>,
}

impl Navigator {
    /// Crée un nouveau `Navigator` avec un état initial vide.
    ///
    /// # Exemple
    ///
    /// ```
    /// use maze_engine::navigation::Navigator;
    /// let navigator = Navigator::new();
    /// assert_eq!(navigator.current_position, (0, 0));
    /// ```
    pub fn new() -> Self {
        Self {
            last_direction: None,
            movement_history: VecDeque::new(),
            visited_positions: HashSet::new(),
            current_position: (0, 0),
            fail_count: HashMap::new(),
            banned_directions: HashSet::new(),
        }
    }

    /// Choisit le prochain déplacement en fonction de la vue radar.
    ///
    /// L'algorithme privilégie les cases non visitées et évite de revenir sur ses pas (demi-tour)
    /// si une autre option est disponible.
    ///
    /// # Arguments
    ///
    /// * `radar_view` - Une matrice 7x7 représentant la vue actuelle du labyrinthe.
    ///
    /// # Retourne
    ///
    /// Une valeur de type `RelativeDirection` indiquant le prochain mouvement à effectuer.
    ///
    /// # Exemple
    ///
    /// ```
    /// #
    /// use maze_engine::navigation::Navigator;
    /// let radar_view = [[' '; 7]; 7];
    /// let mut navigator = Navigator::new();
    /// let next_move = navigator.choose_next_move(&radar_view);
    /// // La direction retournée dépend de la vue ; ici, puisque toutes les cases sont "ouvertes" et non visitées,
    /// // c'est la première option de la liste qui sera choisie.
    /// ```
    pub fn choose_next_move(&mut self, radar_view: &[[char; 7]; 7]) -> RelativeDirection {
        println!("[Navigator] Current position: {:?}", self.current_position);
        println!("[Navigator] Radar View:");
        for row in radar_view.iter() {
            println!("{}", row.iter().collect::<String>());
        }

        // Démarrer avec toutes les directions possibles
        let mut possible_moves = vec![
            RelativeDirection::Front,
            RelativeDirection::Right,
            RelativeDirection::Left,
            RelativeDirection::Back,
        ];

        // Filtrer selon les directions bannies, celles ayant trop d'échecs ou bloquées d'après la vue radar.
        possible_moves.retain(|&dir| {
            let fails = self.fail_count.get(&dir).cloned().unwrap_or(0);
            fails < MAX_FAILS && !self.banned_directions.contains(&dir) && self.is_open(radar_view, dir)
        });

        // Si plusieurs options sont disponibles, éviter le demi-tour (la direction opposée à la dernière)
        if possible_moves.len() > 1 {
            if let Some(last_dir) = self.last_direction {
                let reverse_dir = Self::turn_back(last_dir);
                if possible_moves.len() > 1 && possible_moves.contains(&reverse_dir) {
                    possible_moves.retain(|&dir| dir != reverse_dir);
                }
            }
        }

        // Préférer une direction qui mène vers une case non visitée
        for &dir in &possible_moves {
            let new_pos = Self::calculate_new_position(self.current_position, dir);
            if !self.visited_positions.contains(&new_pos) {
                self.execute_move(dir, new_pos);
                self.last_direction = Some(dir);
                return dir;
            }
        }

        // Si toutes les options mènent vers des cases déjà visitées, choisir celle avec le moins d'échecs
        if !possible_moves.is_empty() {
            possible_moves.sort_by_key(|&dir| self.fail_count.get(&dir).cloned().unwrap_or(0));
            let chosen = possible_moves[0];
            let new_pos = Self::calculate_new_position(self.current_position, chosen);
            self.execute_move(chosen, new_pos);
            self.last_direction = Some(chosen);
            return chosen;
        }

        // Sinon, si aucune option n'est possible, effectuer un demi-tour.
        self.handle_no_moves()
    }

    /// Met à jour l'état interne du Navigator en enregistrant le déplacement effectué.
    ///
    /// Cette fonction met à jour :
    /// - La position actuelle.
    /// - L'historique des déplacements.
    /// - L'ensemble des positions visitées.
    /// - Les compteurs d'échecs et les directions bannies (en supprimant la direction si elle est désormais valide).
    ///
    /// # Arguments
    ///
    /// * `direction` - La direction dans laquelle se déplacer.
    /// * `new_pos` - La nouvelle position calculée après déplacement.
    pub fn execute_move(&mut self, direction: RelativeDirection, new_pos: (i32, i32)) {
        println!("[Navigator] Executing move {:?} to {:?}", direction, new_pos);
        self.banned_directions.remove(&direction);
        self.fail_count.remove(&direction);
        self.movement_history.push_back(direction);
        self.current_position = new_pos;
        self.visited_positions.insert(new_pos);
    }

    /// Convertit un angle (en degrés) en une Option<RelativeDirection>.
    /// - Entre -45° et 45° -> Front
    /// - Entre 45° et 135° -> Right
    /// - Au-delà de 135° (ou en dessous de -135°) -> Back
    /// - Entre -135° et -45° -> Left
    pub fn compute_direction_from_angle(angle: f32) -> Option<RelativeDirection> {
        if angle >= -45.0 && angle <= 45.0 {
            Some(RelativeDirection::Front)
        } else if angle > 45.0 && angle < 135.0 {
            Some(RelativeDirection::Right)
        } else if angle >= 135.0 || angle <= -135.0 {
            Some(RelativeDirection::Back)
        } else if angle < -45.0 && angle > -135.0 {
            Some(RelativeDirection::Left)
        } else {
            None
        }
    }

    /// Gère le cas où aucun déplacement valide n'est possible en effectuant un demi-tour.
    ///
    /// Si l'historique des mouvements n'est pas vide, la fonction effectue un retour (demi-tour) à partir
    /// de la dernière direction enregistrée, met à jour le compteur d'échecs et bannit cette direction.
    /// Sinon, elle choisit une direction aléatoire.
    ///
    /// # Retourne
    ///
    /// La direction choisie pour le demi-tour ou aléatoirement.
    fn handle_no_moves(&mut self) -> RelativeDirection {
        if let Some(last_move) = self.movement_history.pop_back() {
            println!("[Navigator] No valid moves, turning back from {:?}", last_move);
            self.banned_directions.insert(last_move);
            *self.fail_count.entry(last_move).or_insert(0) += 1;
            return Self::turn_back(last_move);
        }
        use rand::distributions::Standard;
        let mut rng = rand::thread_rng();
        let random_direction: RelativeDirection = Standard.sample(&mut rng);
        println!(
            "[Navigator] No moves and no history, choosing random direction: {:?}",
            random_direction
        );
        random_direction
    }

    /// Reçoit un signal d'échec d'un déplacement (par exemple, si le serveur renvoie "CannotPassThroughWall")
    /// et ajuste l'état en conséquence.
    ///
    /// La fonction revient à la position précédente, supprime le dernier mouvement enregistré,
    /// bannit la direction ayant échoué, incrémente son compteur d'échecs et retire la position incorrectement ajoutée.
    ///
    /// # Arguments
    ///
    /// * `direction` - La direction dans laquelle le déplacement a échoué.
    pub fn handle_move_failure(&mut self, direction: RelativeDirection) {
        println!("[Navigator] Move failed in direction {:?}, reverting move.", direction);
        let (dx, dy) = Self::delta(direction);
        let previous_position = (self.current_position.0 - dx, self.current_position.1 - dy);
        println!("[Navigator] Reverting position from {:?} to {:?}", self.current_position, previous_position);
        self.current_position = previous_position;
        if let Some(last_move) = self.movement_history.pop_back() {
            println!("[Navigator] Removing last move {:?} due to failure", last_move);
        }
        self.banned_directions.insert(direction);
        *self.fail_count.entry(direction).or_insert(0) += 1;
        // Supprimer la position incorrectement ajoutée.
        let incorrect_position = (previous_position.0 + dx, previous_position.1 + dy);
        self.visited_positions.remove(&incorrect_position);
    }

    /// Affiche l'état interne du Navigator, notamment les positions visitées et l'historique des déplacements.
    ///
    /// Cette fonction est utile pour le débogage.
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

    /// Vérifie si la case dans la direction donnée est ouverte d'après la vue radar.
    ///
    /// Une case est considérée ouverte si elle contient un espace `' '`.
    ///
    /// # Arguments
    ///
    /// * `radar_view` - La matrice 7x7 de la vue radar.
    /// * `direction` - La direction à vérifier.
    ///
    /// # Retourne
    ///
    /// `true` si la case correspondante est ouverte, sinon `false`.
    pub fn is_open(&self, radar_view: &[[char; 7]; 7], direction: RelativeDirection) -> bool {
        match direction {
            RelativeDirection::Front => radar_view[1][3] == ' ',
            RelativeDirection::Right => radar_view[3][5] == ' ',
            RelativeDirection::Back  => radar_view[5][3] == ' ',
            RelativeDirection::Left  => radar_view[3][1] == ' ',
        }
    }

    /// Calcule la nouvelle position en fonction de la position actuelle et d'une direction donnée.
    ///
    /// # Arguments
    ///
    /// * `(x, y)` - La position actuelle.
    /// * `direction` - La direction du déplacement.
    ///
    /// # Retourne
    ///
    /// Une nouvelle position `(x, y)` après déplacement.
    pub fn calculate_new_position((x, y): (i32, i32), direction: RelativeDirection) -> (i32, i32) {
        let (dx, dy) = Self::delta(direction);
        (x + dx, y + dy)
    }

    /// Retourne le décalage (delta) associé à une direction.
    ///
    /// # Arguments
    ///
    /// * `direction` - La direction pour laquelle calculer le delta.
    ///
    /// # Retourne
    ///
    /// Une paire `(dx, dy)` représentant le changement de position.
    fn delta(direction: RelativeDirection) -> (i32, i32) {
        match direction {
            RelativeDirection::Front => (0, -1),
            RelativeDirection::Right => (1, 0),
            RelativeDirection::Back  => (0, 1),
            RelativeDirection::Left  => (-1, 0),
        }
    }

    /// Retourne la direction opposée (demi-tour) à la direction donnée.
    ///
    /// # Arguments
    ///
    /// * `direction` - La direction de référence.
    ///
    /// # Retourne
    ///
    /// La direction opposée.
    fn turn_back(direction: RelativeDirection) -> RelativeDirection {
        match direction {
            RelativeDirection::Front => RelativeDirection::Back,
            RelativeDirection::Back  => RelativeDirection::Front,
            RelativeDirection::Left  => RelativeDirection::Right,
            RelativeDirection::Right => RelativeDirection::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use commun::structs::RelativeDirection;

    #[test]
    fn test_calculate_new_position() {
        // Vérifie que la fonction calcule correctement la nouvelle position
        let pos = (0, 0);
        assert_eq!(Navigator::calculate_new_position(pos, RelativeDirection::Front), (0, -1));
        assert_eq!(Navigator::calculate_new_position(pos, RelativeDirection::Right), (1, 0));
        assert_eq!(Navigator::calculate_new_position(pos, RelativeDirection::Back), (0, 1));
        assert_eq!(Navigator::calculate_new_position(pos, RelativeDirection::Left), (-1, 0));
    }

    #[test]
    fn test_turn_back() {
        // Vérifie que turn_back retourne la direction opposée
        assert_eq!(Navigator::turn_back(RelativeDirection::Front), RelativeDirection::Back);
        assert_eq!(Navigator::turn_back(RelativeDirection::Right), RelativeDirection::Left);
        assert_eq!(Navigator::turn_back(RelativeDirection::Back), RelativeDirection::Front);
        assert_eq!(Navigator::turn_back(RelativeDirection::Left), RelativeDirection::Right);
    }

    #[test]
    fn test_handle_no_moves_with_history() {
        // Simule un état où un mouvement est présent dans l'historique
        let mut navigator = Navigator::new();
        navigator.movement_history.push_back(RelativeDirection::Front);
        let result = navigator.handle_no_moves();
        // On s'attend à ce que le demi-tour de Front soit retourné
        assert_eq!(result, RelativeDirection::Back);
    }

    #[test]
    fn test_handle_move_failure() {
        // Teste la fonction handle_move_failure en simulant un échec de déplacement.
        let mut navigator = Navigator::new();
        // Simuler un déplacement réussi vers (1, 0)
        navigator.execute_move(RelativeDirection::Right, (1, 0));
        // Simuler un échec dans la direction Right
        navigator.handle_move_failure(RelativeDirection::Right);
        // La position actuelle doit être revenue à (0, 0)
        assert_eq!(navigator.current_position, (0, 0));
        // La direction Right devrait être bannie
        assert!(navigator.banned_directions.contains(&RelativeDirection::Right));
    }

    #[test]
    fn test_compute_direction_from_angle() {
        assert_eq!(Navigator::compute_direction_from_angle(0.0), Some(RelativeDirection::Front));
        assert_eq!(Navigator::compute_direction_from_angle(30.0), Some(RelativeDirection::Front));
        assert_eq!(Navigator::compute_direction_from_angle(90.0), Some(RelativeDirection::Right));
        assert_eq!(Navigator::compute_direction_from_angle(150.0), Some(RelativeDirection::Back));
        assert_eq!(Navigator::compute_direction_from_angle(-150.0), Some(RelativeDirection::Back));
        assert_eq!(Navigator::compute_direction_from_angle(-90.0), Some(RelativeDirection::Left));
    }
}
