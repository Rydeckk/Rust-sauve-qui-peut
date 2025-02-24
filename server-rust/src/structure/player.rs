use commun::encodage::{encode_b64, encode_radar_view_binary};

use super::maze::*;

pub const DISTANCE: i32 = 3;
pub const SIZE_VIEW: usize = 7;

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    position: Point,
    challenge_actif: bool
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            position: Point {
                x: 3,
                y: 3
            },
            challenge_actif: false
        }
    }

    pub fn get_position(self) -> Point {
        self.position
    }

    pub fn set_position(&mut self, new_position: Point) {
        self.position = new_position;
    }

    pub fn get_is_challenge_actif(self) -> bool {
        self.challenge_actif
    }

    pub fn set_is_challenge_actif(&mut self, is_challenge_actif: bool) {
        self.challenge_actif = is_challenge_actif;
    }

    pub fn get_radar_view(self) -> String {
        let mut radar_view: [[char; SIZE_VIEW]; SIZE_VIEW] = [['#';SIZE_VIEW];SIZE_VIEW];

        for (i, row) in radar_view.iter_mut().enumerate() {
            let maze_y: i32 = (self.position.y - DISTANCE) + (i as i32);

            if maze_y < 0 || maze_y >= HEIGHT as i32 {
                continue;
            }

            let mut row_to_add: Vec<char> = vec![];
            for i in 0..SIZE_VIEW { 
                let maze_x = (self.position.x - DISTANCE) + (i as i32);
                if maze_x < 0 || maze_x >= WIDTH as i32 {
                    row_to_add.push('#');
                    continue;
                } else  {
                    row_to_add.push(MAZE[maze_y as usize][maze_x as usize]);
                }
            }
            row.copy_from_slice(&row_to_add);
        }

        let binary_radar_view = encode_radar_view_binary(radar_view);
        
        let encode_radar_view = encode_b64(&binary_radar_view);

        encode_radar_view

    }
}