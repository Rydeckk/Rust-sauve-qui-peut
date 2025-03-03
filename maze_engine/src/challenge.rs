use commun::structs::{ActionError, Challenge};
use std::collections::HashMap;

/// Gère les challenges du jeu, notamment `SecretSumModulo` et `SOS`.
///
/// - `secrets` stocke les valeurs secrètes des joueurs utilisées pour le challenge `SecretSumModulo`.
/// - `sos_active` garde l’identifiant du joueur actuellement en SOS (s'il y en a un).
pub struct ChallengeManager {
    /// Valeurs secrètes associées aux joueurs (clé : `player_id`, valeur : `secret`).
    pub secrets: HashMap<u32, u64>,
    /// Identifiant du joueur qui a initié un challenge SOS (s’il y en a un).
    pub sos_active: Option<u32>,
}

impl ChallengeManager {
    /// Crée un nouveau gestionnaire de challenges.
    ///
    /// # Retourne
    ///
    /// Une instance vide de `ChallengeManager`.
    ///
    /// # Exemple
    ///
    /// ```
    /// use maze_engine::challenge::ChallengeManager;
    /// let manager = ChallengeManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            secrets: HashMap::new(),
            sos_active: None,
        }
    }

    /// Définit (ou met à jour) un secret pour un joueur, utilisé dans le challenge `SecretSumModulo`.
    ///
    /// # Arguments
    ///
    /// * `player_id` - L'identifiant du joueur.
    /// * `secret` - La valeur secrète associée au joueur.
    ///
    /// # Exemple
    ///
    /// ```
    /// use maze_engine::challenge::ChallengeManager;
    /// let mut manager = ChallengeManager::new();
    /// manager.set_secret(1, 42);
    /// ```
    pub fn set_secret(&mut self, player_id: u32, secret: u64) {
        println!("[ChallengeManager] Storing secret for player {}: {}", player_id, secret);
        self.secrets.insert(player_id, secret);
    }

    /// Résout le challenge `SecretSumModulo` en calculant la somme des secrets des joueurs spécifiés,
    /// puis en appliquant l'opération modulo.
    ///
    /// # Arguments
    ///
    /// * `modulo` - Le nombre utilisé pour calculer le modulo.
    /// * `player_ids` - Une tranche contenant les identifiants des joueurs à considérer.
    ///
    /// # Retourne
    ///
    /// La somme des secrets (pour les `player_ids` donnés) modulo `modulo`.
    ///
    /// # Exemple
    ///
    /// ```
    /// use maze_engine::challenge::ChallengeManager;
    /// let mut manager = ChallengeManager::new();
    /// manager.set_secret(1, 10);
    /// manager.set_secret(2, 20);
    /// manager.set_secret(3, 30);
    /// // Pour les joueurs 1, 2 et 3, la somme est 60.
    /// assert_eq!(manager.solve_secret_sum_modulo(7, &[1,2,3]), 60 % 7);
    /// ```
    pub fn solve_secret_sum_modulo(&self, modulo: u64, player_ids: &[u32]) -> u64 {
        let sum: u64 = player_ids
            .iter()
            .map(|player_id| self.secrets.get(player_id).copied().unwrap_or(0))
            .sum();
        let result = sum % modulo;
        println!(
            "[ChallengeManager] Solving SecretModulo: sum = {}, modulo = {}, result = {}",
            sum, modulo, result
        );
        result
    }

    /// Initialise un challenge SOS pour un joueur.
    ///
    /// # Arguments
    ///
    /// * `player_id` - L'identifiant du joueur qui demande de l'aide.
    ///
    /// # Retourne
    ///
    /// * `Ok(Challenge::SOS)` si l'initiation du challenge SOS a réussi.
    /// * `Err(ActionError::NoRunningChallenge)` si un challenge SOS est déjà en cours.
    ///
    /// # Exemple
    ///
    /// ```
    /// use maze_engine::challenge::ChallengeManager;
    /// let mut manager = ChallengeManager::new();
    /// assert!(manager.initiate_sos(1).is_ok());
    /// ```
    pub fn initiate_sos(&mut self, player_id: u32) -> Result<Challenge, ActionError> {
        if self.sos_active.is_some() {
            return Err(ActionError::NoRunningChallenge);
        }
        self.sos_active = Some(player_id);
        Ok(Challenge::SOS)
    }

    /// Résout un challenge SOS en permettant à un équipier de secourir le joueur en détresse.
    ///
    /// # Arguments
    ///
    /// * `rescuer_id` - L'identifiant du joueur qui tente de secourir.
    ///
    /// # Retourne
    ///
    /// * `Ok(())` si le secours est réussi (c'est-à-dire que le joueur qui secourt n'est pas celui en détresse).
    /// * `Err(ActionError::InvalidChallengeSolution)` si le joueur tente de se secourir lui-même.
    /// * `Err(ActionError::NoRunningChallenge)` si aucun challenge SOS n'est actif.
    ///
    /// # Exemple
    ///
    /// ```
    /// use maze_engine::challenge::ChallengeManager;
    /// let mut manager = ChallengeManager::new();
    /// manager.initiate_sos(1).unwrap();
    /// // Le joueur 2 secourt le joueur 1 :
    /// assert!(manager.resolve_sos(2).is_ok());
    /// assert!(manager.sos_active.is_none());
    /// ```
    pub fn resolve_sos(&mut self, rescuer_id: u32) -> Result<(), ActionError> {
        if let Some(player_id) = self.sos_active {
            if rescuer_id != player_id {
                self.sos_active = None;
                return Ok(()); // Sauvetage réussi
            }
            return Err(ActionError::InvalidChallengeSolution);
        }
        Err(ActionError::NoRunningChallenge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_sum_modulo_with_values() {
        let mut challenge_manager = ChallengeManager::new();
        // Définir des secrets pour plusieurs joueurs.
        challenge_manager.set_secret(1, 10);
        challenge_manager.set_secret(2, 20);
        challenge_manager.set_secret(3, 30);
        // Pour les joueurs 1, 2 et 3, la somme est 60.
        assert_eq!(challenge_manager.solve_secret_sum_modulo(10, &[1, 2, 3]), 60 % 10);
        assert_eq!(challenge_manager.solve_secret_sum_modulo(7, &[1, 2, 3]), 60 % 7);
        // Pour un joueur non existant, la valeur doit être considérée comme 0.
        assert_eq!(challenge_manager.solve_secret_sum_modulo(5, &[4]), 0);
    }

    #[test]
    fn test_secret_sum_modulo_empty() {
        let challenge_manager = ChallengeManager::new();
        // Si la liste de joueurs est vide, le résultat doit être 0 quel que soit le modulo.
        assert_eq!(challenge_manager.solve_secret_sum_modulo(10, &[]), 0);
    }

    #[test]
    fn test_initiate_sos_success() {
        let mut challenge_manager = ChallengeManager::new();
        // L'initiation du SOS pour un joueur doit réussir.
        let result = challenge_manager.initiate_sos(1);
        assert!(result.is_ok());
        assert_eq!(challenge_manager.sos_active, Some(1));
    }

    #[test]
    fn test_double_sos_initiation_fails() {
        let mut challenge_manager = ChallengeManager::new();
        // La première initiation réussit.
        assert!(challenge_manager.initiate_sos(1).is_ok());
        // Une deuxième initiation échoue.
        let result = challenge_manager.initiate_sos(2);
        assert!(result.is_err());
        // L'état reste inchangé.
        assert_eq!(challenge_manager.sos_active, Some(1));
    }

    #[test]
    fn test_resolve_sos_success() {
        let mut challenge_manager = ChallengeManager::new();
        // Initier le SOS pour le joueur 1.
        challenge_manager.initiate_sos(1).unwrap();
        // Un autre joueur (2) secourt le joueur 1.
        let result = challenge_manager.resolve_sos(2);
        assert!(result.is_ok());
        // L'état SOS doit être réinitialisé.
        assert!(challenge_manager.sos_active.is_none());
    }

    #[test]
    fn test_resolve_sos_without_active() {
        let mut challenge_manager = ChallengeManager::new();
        // Sans SOS actif, la résolution échoue.
        let result = challenge_manager.resolve_sos(2);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_sos_by_self_fails() {
        let mut challenge_manager = ChallengeManager::new();
        // Initier le SOS pour le joueur 1.
        challenge_manager.initiate_sos(1).unwrap();
        // Le joueur 1 ne peut pas se secourir lui-même.
        let result = challenge_manager.resolve_sos(1);
        assert!(result.is_err());
        assert_eq!(challenge_manager.sos_active, Some(1));
    }

    #[test]
    fn test_resolve_sos_clears_state() {
        let mut challenge_manager = ChallengeManager::new();
        // Initier le SOS pour le joueur 3.
        challenge_manager.initiate_sos(3).unwrap();
        // Le joueur 2 secourt le joueur 3, ce qui doit réinitialiser l'état.
        let result = challenge_manager.resolve_sos(2);
        assert!(result.is_ok());
        assert!(challenge_manager.sos_active.is_none());
    }
}
