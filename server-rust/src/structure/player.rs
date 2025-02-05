use crate::shared::libs::*;
use crate::send_to_client;
use crate::shared::utils::generate_random_number;
use super::maze::*;

pub const DISTANCE: i32 = 3;
pub const SIZE_VIEW: usize = 7;

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    position: Point
}

impl Player {
    pub fn new(name: String) -> Self {
        let mut x = generate_random_number(RangeInclusive::new(0, WIDTH as i32 - 1));
        let mut y = generate_random_number(RangeInclusive::new(0, HEIGHT as i32 - 1));

        if MAZE[x as usize][y as usize] != " " || (x % 2 == 0 || y % 2 == 0) {
            while MAZE[x as usize][y as usize] != " " || (x % 2 == 0 || y % 2 == 0) {
                x = generate_random_number(RangeInclusive::new(0, WIDTH as i32 - 1));
                y = generate_random_number(RangeInclusive::new(0, HEIGHT as i32 - 1));
            }
        }
        Self {
            name: name,
            position: Point { 
                x: x, 
                y: y
            }
        }
    }

    pub fn get_radar_view(self) {
        let mut radar_view: [[&str; SIZE_VIEW]; SIZE_VIEW] = [["#";SIZE_VIEW];SIZE_VIEW];

        for (i, row) in radar_view.iter_mut().enumerate() {
            let maze_y: i32 = (self.position.y - DISTANCE) + (i as i32);

            if maze_y < 0 || maze_y >= HEIGHT as i32 {
                continue;
            }

            let mut row_to_add: Vec<&str> = vec![];
            for i in 0..SIZE_VIEW { 
                let maze_x = (self.position.x - DISTANCE) + (i as i32);
                if maze_x < 0 || maze_x >= WIDTH as i32 {
                    row_to_add.push("#");
                    continue;
                } else  {
                    row_to_add.push(MAZE[maze_y as usize][maze_x as usize]);
                }
            }
            row.copy_from_slice(&row_to_add);
        }

        radar_view[3][3] = "H";

        for row in radar_view.iter() {
            println!("{:?}", row.concat()); 
        }
    }
}