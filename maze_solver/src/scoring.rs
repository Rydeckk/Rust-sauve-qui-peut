use std::collections::HashMap;

/// Gère le score en suivant les mouvements des joueurs.
///
/// Chaque joueur accumule un nombre de mouvements, et le score moyen est calculé en fonction de la taille de l'équipe.
pub struct ScoreManager {
    /// Stocke le nombre de déplacements effectués par chaque joueur (`player_id` -> `nombre de déplacements`).
    player_moves: HashMap<u32, u32>,
    /// Nombre total de joueurs dans l'équipe.
    team_size: u32,
}

impl ScoreManager {
    /// Crée un nouveau gestionnaire de score pour une équipe donnée.
    ///
    /// # Arguments
    /// - `team_size` : Nombre total de joueurs dans l'équipe.
    ///
    /// # Retourne
    /// Une instance vide de `ScoreManager`.
    pub fn new(team_size: u32) -> Self {
        Self {
            player_moves: HashMap::new(),
            team_size,
        }
    }

    /// Enregistre un mouvement pour un joueur.
    ///
    /// # Arguments
    /// - `player_id` : Identifiant du joueur effectuant un mouvement.
    ///
    /// Augmente le compteur de mouvements du joueur.
    pub fn add_move(&mut self, player_id: u32) {
        let counter = self.player_moves.entry(player_id).or_insert(0);
        *counter += 1;
    }

    /// Calcule le score final (moyenne des mouvements par joueur).
    ///
    /// # Retourne
    /// - `0.0` si la taille de l'équipe est `0` (évite une division par zéro).
    /// - Sinon, la moyenne des mouvements effectués par l'ensemble de l'équipe.
    pub fn compute_score(&self) -> f64 {
        if self.team_size == 0 {
            return 0.0; // Évite une division par zéro
        }

        let total_moves: u32 = self.player_moves.values().sum();
        total_moves as f64 / self.team_size as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_move() {
        let mut score_manager = ScoreManager::new(3);
        score_manager.add_move(1);
        score_manager.add_move(1);
        score_manager.add_move(2);

        assert_eq!(*score_manager.player_moves.get(&1).unwrap(), 2);
        assert_eq!(*score_manager.player_moves.get(&2).unwrap(), 1);
        assert_eq!(score_manager.player_moves.get(&3), None);
    }

    #[test]
    fn test_compute_score() {
        let mut score_manager = ScoreManager::new(3);
        score_manager.add_move(1);
        score_manager.add_move(1);
        score_manager.add_move(2);

        let expected_score = (2 + 1) as f64 / 3.0;
        assert_eq!(score_manager.compute_score(), expected_score);
    }

    #[test]
    fn test_compute_score_with_no_moves() {
        let score_manager = ScoreManager::new(3);
        assert_eq!(score_manager.compute_score(), 0.0);
    }

    #[test]
    fn test_compute_score_with_no_players() {
        let score_manager = ScoreManager::new(0);
        assert_eq!(score_manager.compute_score(), 0.0);
    }

    #[test]
    fn test_multiple_moves_different_players() {
        let mut score_manager = ScoreManager::new(4);
        score_manager.add_move(1);
        score_manager.add_move(1);
        score_manager.add_move(2);
        score_manager.add_move(3);
        score_manager.add_move(3);
        score_manager.add_move(3);

        let expected_score = (2 + 1 + 3) as f64 / 4.0;
        assert_eq!(score_manager.compute_score(), expected_score);
    }

    #[test]
    fn test_add_move_updates_correctly() {
        let mut score_manager = ScoreManager::new(3);
        score_manager.add_move(1);
        score_manager.add_move(2);
        score_manager.add_move(1);

        assert_eq!(*score_manager.player_moves.get(&1).unwrap(), 2);
        assert_eq!(*score_manager.player_moves.get(&2).unwrap(), 1);
    }
}
