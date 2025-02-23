# Crate `maze_solver`

Ce crate gère la logique du moteur de jeu pour le projet **Rust-sauve-qui-peut**.  
Il inclut :
- La gestion du labyrinthe (`maze.rs`).
- La gestion des déplacements des joueurs (`movement.rs`).
- La gestion des challenges (`challenge.rs`).
- Le calcul du score (`scoring.rs`).
- La gestion des erreurs (`errors.rs`).

---

## **Installation et Utilisation**
Ce crate est utilisé en interne dans le projet et n'est pas prévu pour être installé séparément.

### **Ajout dans un autre crate**
Si besoin d'utiliser ce crate ailleurs :
```toml
[dependencies]
maze_solver = { path = "../maze_solver" }
```

### **Import des modules**
```rust
use maze_solver::maze::Maze;
use maze_solver::movement::{Player, can_move, move_player};
use maze_solver::challenge::ChallengeManager;
use maze_solver::scoring::ScoreManager;
use maze_solver::errors::ActionError;
```

---

## **Modules et Explications**
### `maze.rs` - Gestion du labyrinthe
- `Maze` → Représente un labyrinthe avec une grille (`.` = libre, `#` = mur).
- `is_wall(x, y)` → Vérifie si une cellule contient un mur.
- `set_wall(x, y)` → Place un mur dans la grille.

### `movement.rs` - Déplacements des joueurs
- `Player` → Représente un joueur avec sa position.
- `can_move(maze, player, direction)` → Vérifie si un déplacement est possible.
- `move_player(maze, player, direction)` → Déplace le joueur si possible.

### `challenge.rs` - Gestion des challenges
- `ChallengeManager` → Gère `SecretSumModulo` et `SOS`.
- `initiate_sos(player_id)` → Active un challenge SOS.
- `resolve_sos(rescuer_id)` → Un joueur vient au secours d'un autre.

### `scoring.rs` - Calcul du score
- `ScoreManager` → Suivi des déplacements et calcul du score.
- `add_move(player_id)` → Enregistre un déplacement pour un joueur.
- `compute_score()` → Retourne le score moyen de l'équipe.

### `errors.rs` - Gestion des erreurs
- `ActionError` → Liste des erreurs possibles (`CannotPassThroughWall`, `NoRunningChallenge`, etc.).

---

## **Tests**
Tous les modules possèdent leurs propres tests intégrés avec `#[cfg(test)]`.  
Exécuter les tests avec :
```sh
cargo test
```
