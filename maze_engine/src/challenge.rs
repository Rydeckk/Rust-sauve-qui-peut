use std::collections::HashMap;
use commun::structs::{Challenge, ActionError};

/// Gère les challenges du jeu, y compris `SecretSumModulo` et `SOS`.
///
/// - `secrets` stocke les valeurs secrètes des joueurs pour le challenge `SecretSumModulo`.
/// - `sos_active` garde une référence à l'ID du joueur en attente de secours (`SOS`).
pub struct ChallengeManager {
    /// Stocke les valeurs secrètes des joueurs pour `SecretSumModulo`.
    pub secrets: HashMap<u32, u64>,
    /// Stocke l'ID du joueur actuellement en `SOS` (s'il y en a un).
    pub sos_active: Option<u32>,
}

impl ChallengeManager {
    /// Crée un nouveau gestionnaire de challenges.
    ///
    /// # Retourne
    /// Une instance vide de `ChallengeManager`.
    pub fn new() -> Self {
        Self {
            secrets: HashMap::new(),
            sos_active: None,
        }
    }

    /// Définit un secret pour un joueur, utilisé dans `SecretSumModulo`.
    ///
    /// # Arguments
    /// - `player_id` : L'identifiant du joueur.
    /// - `secret` : La valeur secrète associée au joueur.
    pub fn set_secret(&mut self, player_id: u32, secret: u64) {
        self.secrets.insert(player_id, secret);
    }

    /// Résout le challenge `SecretSumModulo` en calculant la somme des secrets modulo un nombre donné.
    ///
    /// # Arguments
    /// - `modulo` : Le nombre utilisé pour le calcul du modulo.
    ///
    /// # Retourne
    /// La somme des secrets des joueurs modulo `modulo`.
    pub fn solve_secret_sum_modulo(&self, modulo: u64) -> u64 {
        let sum: u64 = self.secrets.values().sum();
        sum % modulo
    }

    /// Initialise un challenge `SOS` pour un joueur.
    ///
    /// # Arguments
    /// - `player_id` : L'identifiant du joueur demandant de l'aide.
    ///
    /// # Retourne
    /// - `Ok(Challenge::SOS)` si le SOS est bien initié.
    /// - `Err(ActionError::NoRunningChallenge)` si un autre joueur est déjà en `SOS`.
    pub fn initiate_sos(&mut self, player_id: u32) -> Result<Challenge, ActionError> {
        if self.sos_active.is_some() {
            return Err(ActionError::NoRunningChallenge);
        }
        self.sos_active = Some(player_id);
        Ok(Challenge::SOS)
    }

    /// Résout un `SOS` (un équipier vient secourir le joueur en détresse).
    ///
    /// # Arguments
    /// - `rescuer_id` : L'identifiant du joueur venant aider.
    ///
    /// # Retourne
    /// - `Ok(())` si le joueur a bien été secouru.
    /// - `Err(ActionError::InvalidChallengeSolution)` si le joueur tente de se secourir lui-même.
    /// - `Err(ActionError::NoRunningChallenge)` si aucun `SOS` n'est actif.
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
    fn test_secret_sum_modulo() {
        let mut challenge_manager = ChallengeManager::new();
        challenge_manager.set_secret(1, 10);
        challenge_manager.set_secret(2, 20);
        challenge_manager.set_secret(3, 30);

        assert_eq!(challenge_manager.solve_secret_sum_modulo(10), 0);
        assert_eq!(challenge_manager.solve_secret_sum_modulo(7), 4);
    }

    #[test]
    fn test_initiate_sos() {
        let mut challenge_manager = ChallengeManager::new();
        assert!(challenge_manager.initiate_sos(1).is_ok());
        assert_eq!(challenge_manager.sos_active, Some(1));
    }

    #[test]
    fn test_double_sos_initiation_fails() {
        let mut challenge_manager = ChallengeManager::new();
        assert!(challenge_manager.initiate_sos(1).is_ok());
        assert!(challenge_manager.initiate_sos(2).is_err());
    }

    #[test]
    fn test_resolve_sos_success() {
        let mut challenge_manager = ChallengeManager::new();
        challenge_manager.initiate_sos(1).unwrap();
        assert!(challenge_manager.resolve_sos(2).is_ok());
        assert_eq!(challenge_manager.sos_active, None);
    }

    #[test]
    fn test_resolve_sos_without_active_challenge() {
        let mut challenge_manager = ChallengeManager::new();
        assert!(challenge_manager.resolve_sos(2).is_err());
    }

    #[test]
    fn test_resolve_sos_by_self_fails() {
        let mut challenge_manager = ChallengeManager::new();
        challenge_manager.initiate_sos(1).unwrap();
        assert!(challenge_manager.resolve_sos(1).is_err());
    }

    #[test]
    fn test_resolve_sos_clears_state() {
        let mut challenge_manager = ChallengeManager::new();
        challenge_manager.initiate_sos(3).unwrap();
        assert!(challenge_manager.resolve_sos(2).is_ok());
        assert!(challenge_manager.sos_active.is_none());
    }
}
