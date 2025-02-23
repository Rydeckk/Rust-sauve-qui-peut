use crate::maze::Maze;
use commun::structs::{ActionError, RelativeDirection};

/// Représente un point (position) dans la grille du labyrinthe.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    /// Coordonnée horizontale (axe X).
    pub x: usize,
    /// Coordonnée verticale (axe Y).
    pub y: usize,
}

/// Représente un joueur avec sa position dans le labyrinthe.
pub struct Player {
    /// Position actuelle du joueur.
    pub position: Point,
}

impl Player {
    /// Crée un nouveau joueur à la position donnée.
    ///
    /// # Arguments
    /// - `x` : Coordonnée horizontale initiale du joueur.
    /// - `y` : Coordonnée verticale initiale du joueur.
    ///
    /// # Retourne
    /// Une instance de `Player` positionnée aux coordonnées `(x, y)`.
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            position: Point { x, y },
        }
    }

    /// Renvoie la position actuelle du joueur.
    ///
    /// # Retourne
    /// Une référence immuable vers `Point`, contenant les coordonnées `(x, y)`.
    pub fn get_position(&self) -> &Point {
        &self.position
    }
}

/// Vérifie si un joueur peut se déplacer dans une direction donnée.
///
/// # Arguments
/// - `maze` : Référence vers le labyrinthe.
/// - `player` : Référence vers le joueur.
/// - `direction` : Direction du déplacement (`Front`, `Right`, `Back`, `Left`).
///
/// # Retourne
/// - `Ok(Point)` : La nouvelle position du joueur si le déplacement est autorisé.
/// - `Err(ActionError::CannotPassThroughWall)` : Si le joueur tente de traverser un mur.
pub fn can_move(maze: &Maze, player: &Player, direction: RelativeDirection) -> Result<Point, ActionError> {
    let position = player.get_position();
    let mut new_x = position.x;
    let mut new_y = position.y;

    match direction {
        RelativeDirection::Front => new_y = new_y.saturating_sub(1),
        RelativeDirection::Right => new_x = new_x.saturating_add(1),
        RelativeDirection::Back => new_y = new_y.saturating_add(1),
        RelativeDirection::Left => new_x = new_x.saturating_sub(1),
    }

    if maze.is_wall(new_x, new_y) {
        return Err(ActionError::CannotPassThroughWall);
    }

    Ok(Point { x: new_x, y: new_y })
}

/// Déplace un joueur dans une direction donnée si le mouvement est possible.
///
/// # Arguments
/// - `maze` : Référence vers le labyrinthe.
/// - `player` : Référence mutable vers le joueur.
/// - `direction` : Direction du déplacement (`Front`, `Right`, `Back`, `Left`).
///
/// # Retourne
/// - `Ok(())` : Si le déplacement a été effectué.
/// - `Err(ActionError::CannotPassThroughWall)` : Si le joueur tente de traverser un mur.
pub fn move_player(maze: &Maze, player: &mut Player, direction: RelativeDirection) -> Result<(), ActionError> {
    let new_position = can_move(maze, player, direction)?;
    player.position = new_position;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::Maze;

    #[test]
    fn test_player_creation() {
        let player = Player::new(2, 3);
        assert_eq!(player.get_position().x, 2);
        assert_eq!(player.get_position().y, 3);
    }

    #[test]
    fn test_can_move_valid() {
        let maze = Maze::new(5, 5);
        let player = Player::new(2, 2);

        let result = can_move(&maze, &player, RelativeDirection::Front);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Point { x: 2, y: 1 });
    }

    #[test]
    fn test_can_move_into_wall() {
        let mut maze = Maze::new(5, 5);
        maze.set_wall(2, 1); // Mur en face du joueur

        let player = Player::new(2, 2);
        let result = can_move(&maze, &player, RelativeDirection::Front);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ActionError::CannotPassThroughWall);
    }

    #[test]
    fn test_move_player_success() {
        let maze = Maze::new(5, 5);
        let mut player = Player::new(2, 2);

        let result = move_player(&maze, &mut player, RelativeDirection::Front);
        assert!(result.is_ok());
        assert_eq!(player.get_position().x, 2);
        assert_eq!(player.get_position().y, 1);
    }

    #[test]
    fn test_move_player_into_wall() {
        let mut maze = Maze::new(5, 5);
        maze.set_wall(2, 1); // Mur devant le joueur

        let mut player = Player::new(2, 2);
        let result = move_player(&maze, &mut player, RelativeDirection::Front);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ActionError::CannotPassThroughWall);
        assert_eq!(player.get_position().x, 2); // Position inchangée
        assert_eq!(player.get_position().y, 2);
    }
}
