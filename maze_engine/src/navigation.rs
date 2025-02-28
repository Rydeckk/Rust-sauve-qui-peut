use crate::global_map::GlobalMap;
use crate::radar::RadarView;
use commun::structs::RelativeDirection;
use std::collections::VecDeque;

const HISTORY_SIZE: usize = 5;

#[derive(Debug)]
pub struct Navigator {
    history: VecDeque<RelativeDirection>,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    pub fn choose_next_move(&mut self, radar: &RadarView, global_map: &GlobalMap) -> Option<RelativeDirection> {
        println!("\n[Navigator] Début du choix du mouvement");
        println!("RadarView reçue : {:?}", radar);
        println!("GlobalMap actuelle :");
        global_map.print_map();

        let mut possible_moves = Vec::new();

        for &dir in &[RelativeDirection::Front, RelativeDirection::Right, RelativeDirection::Back, RelativeDirection::Left] {
            if radar.is_open(dir) {
                println!("-> Direction {:?} est ouverte.", dir);
                possible_moves.push(dir);
            } else {
                println!("-> Direction {:?} est bloquée.", dir);
            }
        }

        println!("[Navigator] Après analyse des murs, mouvements possibles : {:?}", possible_moves);

        possible_moves.retain(|&dir| {
            let (dx, dy) = Self::direction_to_offset(dir);
            let new_x = (global_map.player_pos.0 as isize + dx) as usize;
            let new_y = (global_map.player_pos.1 as isize + dy) as usize;

            if global_map.is_wall(new_x, new_y) {
                println!("-> Direction {:?} bloquée par un mur sur la GlobalMap.", dir);
                return false;
            }
            if global_map.is_visited(new_x, new_y) {
                println!("-> Direction {:?} déjà visitée, évitée.", dir);
                return false;
            }
            true
        });

        println!("[Navigator] Après filtrage des murs et des positions visitées : {:?}", possible_moves);

        if let Some(&best_move) = possible_moves.iter().find(|&&d| radar.has_hint(d) || radar.has_goal(d)) {
            println!("[Navigator] Choix prioritaire car indice ou objectif trouvé en {:?}", best_move);
            return Some(best_move);
        }

        possible_moves.retain(|&dir| !self.history.contains(&dir));
        println!("[Navigator] Après filtrage de l'historique : {:?}", possible_moves);
        println!("Historique des déplacements : {:?}", self.history);

        let chosen_move = possible_moves.first().copied();

        if let Some(dir) = chosen_move {
            if self.history.len() == HISTORY_SIZE {
                self.history.pop_front();
            }
            self.history.push_back(dir);
            println!("[Navigator] Choix final : {:?}", dir);
        } else {
            println!("[Navigator] Aucun mouvement possible trouvé !");
        }

        chosen_move
    }

    fn direction_to_offset(dir: RelativeDirection) -> (isize, isize) {
        match dir {
            RelativeDirection::Front => (0, -2),
            RelativeDirection::Right => (2, 0),
            RelativeDirection::Back => (0, 2),
            RelativeDirection::Left => (-2, 0),
        }
    }
}