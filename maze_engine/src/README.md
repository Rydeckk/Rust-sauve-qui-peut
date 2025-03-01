# maze_engine

**maze_engine** est la crate centrale du projet Maze Game. Elle fournit la logique et les structures de données essentielles pour gérer le labyrinthe, la navigation, la gestion des défis et le calcul du score.

---

## Table des matières

- [Modules](#modules)
- [Installation](#installation)
- [Exemple d'utilisation](#exemple-dutilisation)
- [Tests](#tests)
---

## Modules

### 1. radar
Ce module contient la structure [`RadarView`](./radar.rs) qui représente la vue locale autour d'un joueur dans le labyrinthe.  
**Champs principaux :**
- `walls_horiz` : Matrice 4x3 de booléens indiquant la présence de murs horizontaux.
- `walls_vert` : Matrice 3x4 de booléens indiquant la présence de murs verticaux.
- `cells` : Matrice 3x3 de `u8` qui encode des informations sur le contenu des cellules (indices, cible, etc.).

### 2. navigation
Le module [`Navigator`](./navigation.rs) implémente l'algorithme de navigation dans le labyrinthe.  
**Fonctionnalités principales :**
- Détermine le prochain mouvement à effectuer en fonction de la vue radar.
- Enregistre l'historique des déplacements et les positions visitées.
- Gère les échecs de déplacement en effectuant un retour arrière (demi-tour) si nécessaire.

### 3. global_map
Ce module propose la structure [`GlobalMap`](./global_map.rs) qui permet de conserver une carte globale du labyrinthe.  
**Fonctionnalités principales :**
- Stocke une grille représentant le labyrinthe.
- Met à jour la carte en fonction d'une `RadarView`.
- Marque les cases explorées, les murs, les indices (`H`) et la cible (`G`).

### 4. challenge
Le module [`ChallengeManager`](./challenge.rs) gère les défis du jeu, notamment :
- **SecretSumModulo** : Calcule la somme des valeurs secrètes reçues par les joueurs, modulo un nombre donné.
- **SOS** : Gère les situations où un joueur demande de l’aide.

### 5. scoring
Ce module contient le [`ScoreManager`](./scoring.rs) qui suit le nombre de déplacements de chaque joueur et calcule le score final (moyenne des déplacements par joueur).

---

## Installation

Ajoutez la crate dans votre fichier `Cargo.toml` de la manière suivante si vous travaillez en mode workspace :

```toml
[dependencies]
maze_engine = { path = "./maze_engine" }
```

Assurez-vous que votre projet est correctement structuré en workspace afin de partager le code commun.

## Exemple d'utilisation
Voici un exemple simple montrant comment utiliser certaines fonctionnalités de la crate :

```rust
use maze_engine::navigation::Navigator;
use maze_engine::global_map::GlobalMap;
use maze_engine::challenge::ChallengeManager;
use commun::structs::RelativeDirection;

fn main() {
    // Initialisation du Navigator et de la GlobalMap
    let mut navigator = Navigator::new();
    let mut global_map = GlobalMap::new(5, 5);

    // Exemple de vue radar (ici une matrice 7x7 remplie d'espaces)
    let radar_view = [[' '; 7]; 7];

    // Choix du prochain mouvement en fonction de la vue radar
    let next_move = navigator.choose_next_move(&radar_view);
    println!("Prochain mouvement : {:?}", next_move);

    // Mise à jour de la carte globale via un RadarView (à adapter avec de vraies données)
    // global_map.update_from_radar(&radar_view_object, Some((dx, dy)));

    // Gestion des challenges
    let mut challenge_manager = ChallengeManager::new();
    challenge_manager.set_secret(1, 10);
    challenge_manager.set_secret(2, 20);
    let answer = challenge_manager.solve_secret_sum_modulo(7, &[1, 2]);
    println!("Réponse SecretSumModulo : {}", answer);
}
```

## Tests
La crate inclut des tests unitaires pour chacun des modules. Pour exécuter les tests, utilisez :

```bash
cargo test --package maze_engine
```