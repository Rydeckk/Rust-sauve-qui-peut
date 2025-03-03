# Sauve Qui Peut

Ce projet consiste en la réalisation d'un client et d'un serveur pour un jeu d'évasion dans un labyrinthe. Les joueurs évoluent dans le labyrinthe, reçoivent des vues radar pour se repérer, et doivent éviter les murs tout en cherchant la sortie.

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) installé (avec Cargo).
- Les dépendances du projet seront automatiquement installées via Cargo.

## Organisation du Projet

Le projet est structuré en plusieurs crates :
- **client** : Le client qui se connecte au serveur et gère la navigation dans le labyrinthe.
- **server-rust** : Notre serveur de test.
- **commun** : Les éléments communs (structures, protocoles, encodage/décodage).
- **maze_engine** : Le crate gérant la navigation dans le labyrinthe et la gestion du jeu.

## Commandes de Lancement

### Lancer le Serveur en mode Debug

Pour démarrer le serveur en mode debug (affichage détaillé des logs), placez-vous dans le répertoire du serveur et exécutez :

```bash
./server run --debug
```

Cela lancera le serveur en mode debug sur le port par défaut (8778).

### Lancer le Client

Pour lancer le client, assurez-vous que le serveur est bien lancé et écoute sur l'adresse souhaitée. Dans le répertoire de la crate client, exécutez :

```bash
cargo run -- 127.0.0.1
```

L'argument `127.0.0.1` représente l'adresse du serveur. Vous pouvez le remplacer par l'adresse IP ou le nom d'hôte correspondant.

## Tests

Le projet inclut une suite de tests unitaires pour vérifier le bon fonctionnement de :
- La navigation (calcul des positions, gestion des échecs, etc.)
- Les fonctions d'encodage et de décodage.

Pour exécuter l'ensemble des tests, utilisez :

```bash
cargo test
```
