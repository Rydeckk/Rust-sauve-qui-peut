use serde::{Serialize, Deserialize};

/// Définit les erreurs possibles rencontrées dans le moteur du jeu.
///
/// Ces erreurs peuvent être retournées lors des déplacements,
/// des challenges ou d'autres actions des joueurs.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ActionError {
    /// Le joueur tente de traverser un mur.
    CannotPassThroughWall,
    /// Le joueur tente de passer à travers un adversaire.
    CannotPassThroughOpponent,
    /// Aucune challenge actif pour être résolu.
    NoRunningChallenge,
    /// Le joueur doit résoudre un challenge avant de continuer.
    SolveChallengeFirst,
    /// La solution fournie pour un challenge est incorrecte.
    InvalidChallengeSolution,
    /// Le joueur en `SOS` a tenté de bouger sans être secouru.
    PlayerMustBeRescued,
}
